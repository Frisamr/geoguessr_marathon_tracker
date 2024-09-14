use std::default::Default;
use std::time::Instant;
use std::str::FromStr;

use env_logger::{Builder, Env};
#[allow(unused_imports)]
use log::{debug, error, info, warn};

use eframe::egui::{
    self, Button, FontData, FontDefinitions, FontFamily, FontId, RichText, Style, TextStyle::*, Ui,
    Vec2,
};
use eframe::NativeOptions;

mod marathon_log;
mod utils;

use marathon_log::{AddEntryResult, MarathonLog};
use utils::time_counter;
use utils::timekeeping::HoursMinutesSeconds;
use utils::{calculate_countdown, score_from_str, timekeeping::TWENTY_FOUR_HOURS_IN_SECS};


const APP_NAME: &str = "GeoMarathonTracker";

fn main() {
    let env = Env::new().default_filter_or("INFO");
    let mut env_logger_builder = Builder::from_env(env);
    env_logger_builder.init();

    let eframe_opts = custom_native_opts(NativeOptions::default());
    let start_res = eframe::run_native(
        APP_NAME,
        eframe_opts,
        Box::new(|cc| Ok(Box::new(EguiTrackerApp::new(cc)))),
    );
    if let Err(err) = start_res {
        error!("failed to start display: {}", err);
    }
}

struct EguiTrackerApp {
    is_started: bool,
    marathon_log: MarathonLog,
    save_on_exit: bool,
    score_input_txt: String,
    file_name_txt: String,
    hours_txt: String,
    mins_txt: String,
    secs_txt: String,
    headstart_5k_txt: String,
    err_state: AppErrState,
}

impl eframe::App for EguiTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_started {
                self.show_tracker_display(ui);
            } else {
                self.show_start_display(ui);
            }
        });

        ctx.request_repaint();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if self.save_on_exit {
            let res = self.marathon_log.save_to_file();
            if res.is_err() {
                error!("error saving to file: {}", res.unwrap_err().to_string());
            }
        }
    }
}

impl EguiTrackerApp {
    fn show_start_display(&mut self, ui: &mut Ui) {
        use egui::TextEdit;

        if ui.button("Start timer").clicked() {
            self.is_started = true;
            if let Ok(hms) = HoursMinutesSeconds::from_strs(&self.hours_txt, &self.mins_txt, &self.secs_txt) {
                let total_secs = TWENTY_FOUR_HOURS_IN_SECS - hms.total_secs();
                self.marathon_log.epoch_offset_secs = total_secs;
            }
            self.marathon_log.current_epoch = Some(Instant::now());
            if let Ok(count) = u16::from_str(&self.headstart_5k_txt) {
                let added_up = self.marathon_log.add_up_5ks();
                if added_up != count {
                    if added_up > count {
                        error!("data from file has more 5ks than provided number");
                    }
                    if added_up < count {
                        let diff = count - added_up;
                        for _ in 0..diff {
                            self.marathon_log.try_add_entry(5000);
                        }
                    }
                }
                self.marathon_log.total_5ks = self.marathon_log.add_up_5ks();
            }
        }
        let save_btn_txt = if self.save_on_exit {
            "saving on exit is ON"
        } else {
            "saving on exit is OFF"
        };
        ui.label("");
        if ui.button(save_btn_txt).clicked() {
            self.save_on_exit = !self.save_on_exit;
        }

        ui.heading("File name:");
        ui.add(TextEdit::singleline(&mut self.file_name_txt));
        if ui.button("Load from file").clicked() {
            let res = self.marathon_log.load_from_file(&self.file_name_txt);
            if let Err(err) = res {
                error!("error reading file: {}", err.to_string());
            }
        };
        ui.heading("Headstart time:");
        ui.add(TextEdit::singleline(&mut self.hours_txt));
        ui.add(TextEdit::singleline(&mut self.mins_txt));
        ui.add(TextEdit::singleline(&mut self.secs_txt));
        ui.heading("Headstart 5ks:");
        ui.add(TextEdit::singleline(&mut self.headstart_5k_txt));
    }

    fn show_tracker_display(&mut self, ui: &mut Ui) {
        use egui::TextEdit;

        let time_since_epoch = match self
            .marathon_log
            .current_epoch
            .map(|epoch| u32::try_from(epoch.elapsed().as_secs()))
        {
            Some(Ok(secs)) => secs,
            Some(Err(err)) => {
                self.err_state.time_err = Some(err.to_string());
                0
            }
            None => 0,
        };
        let (is_paused, pause_btn_txt) = match self.marathon_log.current_epoch {
            Some(_) => (false, "Pause"),
            None => (true, "Unpause"),
        };
        let countdown = calculate_countdown(time_since_epoch, self.marathon_log.epoch_offset_secs);
        let time_since_5k = self
            .marathon_log
            .time_since_last_5k()
            .map_or("".to_owned(), time_counter);
        let estimated_pace = self
            .marathon_log
            .estimate_pace()
            .map_or("".to_owned(), |x| x.to_string());

        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.heading("Time left:");
                ui.label(countdown);
                ui.label("");
                ui.heading("Last 5k:");
                ui.label(time_since_5k);

                let _test = ui.label("");
                ui.label("");
                if ui.add(Button::new(pause_btn_txt)).clicked() {
                    if is_paused {
                        self.marathon_log.current_epoch = Some(Instant::now());
                        self.err_state.timer_paused = false;
                    } else {
                        // is_paused is false, so the current_epoch must be Some(_)
                        let elapsed = self.marathon_log.current_epoch.unwrap().elapsed().as_secs();
                        self.marathon_log.epoch_offset_secs += u32::try_from(elapsed)
                            .expect("this timer should not run for 136 years");
                        self.marathon_log.current_epoch = None;
                    }
                }
            });
            ui.vertical(|ui| {
                ui.heading("5k count:");
                ui.label(self.marathon_log.total_5ks.to_string());
                ui.label("");
                ui.heading("Pace:");
                ui.label(estimated_pace);

                ui.label("");
                ui.label("");
                if ui.add(Button::new("Add 5k")).clicked() {
                    match self.marathon_log.try_add_entry(5000) {
                        AddEntryResult::Ok => {
                            self.err_state.invalid_score = None;
                        }
                        AddEntryResult::TimerPaused => {
                            self.err_state.timer_paused = true;
                        }
                        AddEntryResult::ImpossibleScore { score: _ } => {
                            unreachable!();
                        }
                    };
                }
            });
        });

        ui.separator();
        /* if ui.button("print entries").clicked() {
            self.marathon_log.print_entries();
        } */
        ui.heading("Paste score:");
        let response = ui.add(TextEdit::multiline(&mut self.score_input_txt).desired_rows(2));
        if response.changed() && (self.score_input_txt.chars().filter(|&c| c == '\n').count() >= 1)
        {
            let score_conv_res = score_from_str(&self.score_input_txt);
            if let Err(err) = score_conv_res {
                self.score_input_txt.clear();
                self.err_state.invalid_score = Some(err.to_string());
            } else {
                // score_conv_res is known to not be an error
                match self.marathon_log.try_add_entry(score_conv_res.unwrap()) {
                    AddEntryResult::Ok => {
                        self.score_input_txt.clear();
                        self.err_state.invalid_score = None;
                    }
                    AddEntryResult::TimerPaused => {
                        self.clear_extra_lines();
                        self.err_state.timer_paused = true;
                    }
                    AddEntryResult::ImpossibleScore { score } => {
                        self.score_input_txt.clear();
                        self.err_state.invalid_score = Some(score.to_string());
                    }
                };
            }
        }
        ui.label(
            RichText::new(self.err_state.get_err_txt())
                .color(egui::Color32::from_rgb(240, 10, 10))
                .small(),
        );
    }

    fn clear_extra_lines(&mut self) {
        let len = self
            .score_input_txt
            .trim_end_matches(|c: char| !c.is_ascii_digit())
            .len();
        self.score_input_txt.truncate(len);
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut font_defs = FontDefinitions::default();
        font_defs.font_data.insert(
            "Recursive Mono".to_owned(),
            FontData::from_static(include_bytes!("fonts/RecursiveMonoLnrSt-Bold.ttf")),
        );
        font_defs
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "Recursive Mono".to_owned());
        font_defs.font_data.insert(
            "Recursive Sans".to_owned(),
            FontData::from_static(include_bytes!("fonts/RecursiveSansLnrSt-Bold.ttf")),
        );
        font_defs
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Recursive Sans".to_owned());
        cc.egui_ctx.set_fonts(font_defs);

        let style = (*cc.egui_ctx.style()).clone();
        cc.egui_ctx.set_style(custom_egui_styles(style));

        Self {
            is_started: false,
            marathon_log: MarathonLog::new(TWENTY_FOUR_HOURS_IN_SECS),
            save_on_exit: false,
            score_input_txt: String::new(),
            file_name_txt: String::new(),
            hours_txt: String::new(),
            mins_txt: String::new(),
            secs_txt: String::new(),
            headstart_5k_txt: String::new(),
            err_state: AppErrState {
                timer_paused: false,
                invalid_score: None,
                time_err: None,
            },
        }
    }
}

/// The string in invalid_score is the invalid score that was attempted to be added.
/// The string in time_err is the error string.
struct AppErrState {
    timer_paused: bool,
    invalid_score: Option<String>,
    time_err: Option<String>,
}

impl AppErrState {
    fn get_err_txt(&self) -> String {
        let mut err_display_txt = String::new();

        if self.timer_paused {
            err_display_txt += "entries should NOT be added while the timer is paused!";
        }
        if let Some(score_string) = &self.invalid_score {
            err_display_txt += "\r\ninvalid score: ";
            err_display_txt += score_string;
        }
        if let Some(time_err_string) = &self.time_err {
            err_display_txt += "\r\nerror getting time: ";
            err_display_txt += time_err_string;
        }

        err_display_txt
    }
}

fn custom_egui_styles(mut style: Style) -> Style {
    use egui::Margin;

    let mut spacing = style.spacing.clone();
    let window_margin = 30.0;
    spacing.window_margin = Margin {
        left: window_margin,
        right: window_margin,
        top: window_margin,
        bottom: window_margin,
    };
    spacing.item_spacing = Vec2 { x: 50.0, y: 2.0 };
    style.spacing = spacing;

    style.text_styles = [
        (Heading, FontId::new(26.0, FontFamily::Proportional)),
        (Body, FontId::new(26.0, FontFamily::Monospace)),
        (Monospace, FontId::new(26.0, FontFamily::Monospace)),
        (Button, FontId::new(20.0, FontFamily::Proportional)),
        (Small, FontId::new(16.0, FontFamily::Proportional)),
    ]
    .into();

    style
}

fn custom_native_opts(mut opts: NativeOptions) -> NativeOptions {
    let window_x = 350.0;
    let window_y = 420.0;

    use eframe::egui::IconData;
    opts.viewport = opts
        .viewport
        .with_maximize_button(false)
        .with_icon(IconData::default())
        .with_resizable(false)
        .with_inner_size((window_x, window_y))
        .with_min_inner_size((window_x, window_y))
        .with_max_inner_size((window_x, window_y));

    // TODO: use persistence
    // let mut current_path = std::env::current_dir().unwrap();
    // current_path.push(APP_NAME);
    // opts.persistence_path = Some(current_path);

    opts
}
