use std::default::Default;
use std::str::FromStr;
use std::time::Instant;

use env_logger::{Builder, Env};
use log::{debug, error, info, warn};

use eframe::egui::{
    self, Button, FontData, FontDefinitions, FontFamily, FontId, RichText, TextStyle::*, Ui, Vec2
};
use eframe::NativeOptions;

mod attempt_log;
mod utils;

use attempt_log::AttemptsLog;
use utils::timekeeping::{HoursMinutesSeconds, TWENTY_FOUR_HOURS_IN_SECS};

// TODO: re-use allocation by not constructing new strings
const APP_NAME: &str = "GeoMarathonTracker";

fn main() {
    let env = Env::new().default_filter_or("INFO");
    let mut env_logger_builder = Builder::from_env(env);
    env_logger_builder.init();

    let eframe_opts = set_native_opts(NativeOptions::default());
    let start_res = eframe::run_native(
        APP_NAME,
        eframe_opts,
        Box::new(|cc| Ok(Box::new(EguiTrackerApp::new(cc)))),
    );
    if let Err(err) = start_res {
        error!("failed to start display: {}", err);
    }
}

fn set_native_opts(mut opts: NativeOptions) -> NativeOptions {
    let window_x = 270.0;
    let window_y = 350.0;

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

pub(crate) struct EguiTrackerApp {
    is_started: bool,
    attempts_log: AttemptsLog,
    score_input_txt: String,
    err_display_txt: String,
    warning_display_txt: String,
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
}

impl EguiTrackerApp {
    fn show_start_display(&mut self, ui: &mut Ui) {
        if ui.add(Button::new("Start timer")).clicked() {
            self.is_started = true;
            self.attempts_log.current_epoch = Some(Instant::now());
        }
    }

    fn show_tracker_display(&mut self, ui: &mut Ui) {
        use egui::TextEdit;
        let time_since_epoch = match self.attempts_log.current_epoch {
            Some(epoch) => epoch.elapsed().as_secs(),
            None => 0,
        };
        let countdown = calculate_countdown(time_since_epoch, self.attempts_log.epoch_offset_secs);

        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.heading("Time Remaining");
                ui.label(countdown);
            });
            ui.vertical(|ui| {
                ui.heading("5k count");
                ui.label(self.attempts_log.total_5ks.to_string());
            });
        });

        ui.separator();
        let (is_paused, pause_btn_txt) = match self.attempts_log.current_epoch {
            Some(_) => (false, "Pause timer"),
            None => (true, "Unpause timer"),
        };
        if ui.add(Button::new(pause_btn_txt)).clicked() {
            if is_paused {
                self.attempts_log.current_epoch = Some(Instant::now());
                self.warning_display_txt.clear();
            } else {
                self.attempts_log.epoch_offset_secs +=
                    self.attempts_log.current_epoch.unwrap().elapsed().as_secs();
                self.attempts_log.current_epoch = None;
            }
        }
        ui.label("");
        if ui.add(Button::new("Print stats")).clicked() {
            self.attempts_log.print_stats();
        }
        ui.label("");
        ui.heading("Paste score:");
        let response = ui.add(TextEdit::multiline(&mut self.score_input_txt).desired_rows(1));
        if response.changed() && (self.score_input_txt.chars().filter(|&c| c == '\n').count() >= 1)
        {
            if is_paused {
                self.warning_display_txt.clear();
                self.warning_display_txt +=
                    "\r\nentries should NOT be added while the timer is paused";
            }
            let res = try_add_entry(&mut self.attempts_log, &self.score_input_txt);
            if let Err(e) = res {
                let len = self.score_input_txt.trim_end_matches(|c: char| !c.is_ascii_digit()).len();
                self.score_input_txt.truncate(len);
                self.err_display_txt.clear();
                self.err_display_txt += "error adding entry: ";
                self.err_display_txt += &e.to_string();
            } else {
                self.score_input_txt.clear();
                self.err_display_txt.clear();
            }
        }
        ui.label(RichText::new(&self.warning_display_txt).color(egui::Color32::from_rgb(240, 10, 10)).small());
        ui.label(RichText::new(&self.err_display_txt).color(egui::Color32::from_rgb(240, 10, 10)).small());
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        use egui::Margin;

        let mut style = (*cc.egui_ctx.style()).clone();
        let mut spacing = style.spacing.clone();
        let window_margin = 30.0;
        spacing.window_margin = Margin {
            left: window_margin,
            right: window_margin,
            top: window_margin,
            bottom: window_margin,
        };
        spacing.item_spacing = Vec2 { x: 30.0, y: 2.0 };
        style.spacing = spacing;

        let mut font_defs = FontDefinitions::default();
        /* font_defs.font_data.insert(
            "Recursive Sans".to_owned(),
            FontData::from_static(include_bytes!("fonts/RecursiveSansLnrSt-SemiBold.ttf")),
        ); */
        font_defs.font_data.insert(
            "Recursive Mono".to_owned(),
            FontData::from_static(include_bytes!("fonts/RecMonoLinear-Regular-1.085.ttf")),
        );
        /* font_defs
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "Recursive Sans".to_owned()); */
        font_defs
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "Recursive Mono".to_owned());
        cc.egui_ctx.set_fonts(font_defs);
        style.text_styles = [
            (Heading, FontId::new(20.0, FontFamily::Proportional)),
            (Body, FontId::new(20.0, FontFamily::Monospace)),
            (Monospace, FontId::new(20.0, FontFamily::Monospace)),
            (Button, FontId::new(20.0, FontFamily::Proportional)),
            (Small, FontId::new(14.0, FontFamily::Proportional)),
        ]
        .into();
        cc.egui_ctx.set_style(style);

        Self {
            is_started: false,
            attempts_log: AttemptsLog::new(),
            score_input_txt: String::new(),
            err_display_txt: String::new(),
            warning_display_txt: String::new(),
        }
    }
}

// TODO: impl Display on HoursMinutesSeconds and remove this fn
fn calculate_countdown(time_since_epoch: u64, epoch_offset_secs: u64) -> String {
    let time_passed = time_since_epoch + epoch_offset_secs;
    let time_remaining = TWENTY_FOUR_HOURS_IN_SECS.saturating_sub(time_passed);

    let time_hms = HoursMinutesSeconds::from_secs(time_remaining);
    let mut h = time_hms.hours.to_string();
    let mut m = time_hms.minutes.to_string();
    let mut s = time_hms.seconds.to_string();
    for s in [&mut h, &mut m, &mut s].into_iter() {
        if s.len() < 2 {
            s.insert(0, '0');
        }
    }

    h + ":" + &m + ":" + &s
}

fn try_add_entry(attempts_log: &mut AttemptsLog, input_str: &str) -> Result<(), String> {
    if input_str.lines().count() < 1 {
        return Err("no lines in input".to_owned());
    }
    let first_line = input_str.lines().next().unwrap();
    let score_str: String = first_line.chars().filter(|c| c.is_ascii_digit()).collect();
    debug!("score str: {}", &score_str);

    match u16::from_str(&score_str) {
        Ok(score) => {
            if score > 5000 {
                return Err("score cannot be greater than 5000".to_owned());
            }
            attempts_log.add_entry(score);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn countdown_test() {
        let total_time: u64 = (60 * 60 * 22) + (60 * 58) + 59;
        let mut rng = rand::thread_rng();
        let epoch_offset_secs = rng.gen_range(0..=total_time);
        let time_since_epoch = total_time - epoch_offset_secs;

        let res = calculate_countdown(time_since_epoch, epoch_offset_secs);
        assert_eq!(res, "01:01:01");
    }
}
