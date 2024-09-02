use env_logger::{Builder, Env};
use inquire::{validator::Validation, CustomUserError, InquireError, Text};
use log::{debug, error, info, warn};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let env = Env::new().default_filter_or("INFO");
    let mut builder = Builder::from_env(env);
    builder.init();

    println!("Commands are always given as a single character. Some commands may ask for additional input.");
    println!("Command list:"); // TODO: update cmd list
    println!("- {}: quit", CmdType::QUIT_CHAR);
    println!("- {}: print stats", CmdType::PRINT_STATS_CHAR);
    println!(
        "- {}: add a new entry to the list with the provided score",
        CmdType::ADD_ENTRY_CHAR
    );
    println!(
        "- {}: fix the most recent entry to use the provided score",
        CmdType::FIX_PREV_CHAR
    );

    let mut attempts_log = AttemptsLog::new();

    loop {
        let cmd_prompt = Text::new("enter command>").with_validator(cmd_validator);
        let input_res = cmd_prompt.prompt();
        if input_res.is_err() {
            error!("error getting input");
            continue;
        }
        let cmd_res = CmdType::from_str(&input_res.unwrap()); // input_res is known to not be an err
        if let Err(err_string) = cmd_res {
            warn!("invalid command: {err_string}");
            continue;
        }

        match cmd_res.unwrap() {
            //cmd_res is know to not be an err
            CmdType::Quit => break, // TODO: add saving to file on quit
            CmdType::PrintStats => attempts_log.print_stats(),
            CmdType::AddEntry => {
                let round_score_string = get_score();
                attempts_log.add_entry(&round_score_string);
            }
            CmdType::FixPrev => {
                let score_string = get_score();
                let res = attempts_log.fix_prev_entry(&score_string);
                if res.is_err() {
                    warn!("unable to fix previous entry because there are no entries in the log!");
                }
            }
        }
    }
}

fn get_score() -> String {
    loop {
        let score_prompt = Text::new("enter round score>").with_validator(score_validator);
        let round_score_res = score_prompt.prompt();

        if let Err(score_err) = round_score_res {
            match score_err {
                InquireError::InvalidConfiguration(reason) => {
                    warn!("invalid score: {reason}");
                    continue;
                }
                _ => {
                    error!("error getting input");
                    continue;
                }
            }
        };
        let round_score_str: String = round_score_res
            .unwrap()   // err case handled by guard clause
            .trim_end()
            .chars()
            .filter(|c| *c != ',')
            .collect();
        debug!("round score string: {}", round_score_str);
        return round_score_str;
    }
}

enum CmdType {
    Quit,
    PrintStats,
    AddEntry,
    FixPrev,
}

impl FromStr for CmdType {
    type Err = String; // TODO: impl custom err type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd_char: char = s.chars().next().unwrap(); //unwrap because validator would have caught it

        match cmd_char {
            Self::QUIT_CHAR => Ok(CmdType::Quit),
            Self::PRINT_STATS_CHAR => Ok(CmdType::PrintStats),
            Self::ADD_ENTRY_CHAR => Ok(CmdType::AddEntry),
            Self::FIX_PREV_CHAR => Ok(CmdType::FixPrev),
            _ => Err("unkown command character".into()),
        }
    }
}

impl CmdType {
    const QUIT_CHAR: char = 'q';
    const PRINT_STATS_CHAR: char = 's';
    const ADD_ENTRY_CHAR: char = 'a';
    const FIX_PREV_CHAR: char = 'f';
}

fn cmd_validator(input: &str) -> Result<Validation, CustomUserError> {
    let trimmed_len = input.trim_end().len();
    if trimmed_len == 1 {
        let first_char = input.chars().next().unwrap(); //string is known to have at least one char
        if first_char.is_ascii_alphabetic() {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(
                "commands should be given as a single letter".into(),
            ))
        }
    } else {
        Ok(Validation::Invalid(
            "commands should be given as a single letter".into(),
        ))
    }
}
fn score_validator(input: &str) -> Result<Validation, CustomUserError> {
    let trimmed = input.trim_end();
    let commas_removed: String = trimmed.chars().filter(|c| *c != ',').collect();
    debug!("commas_removed: {}", &commas_removed);
    if (1..=4).contains(&commas_removed.len()) {
        let is_all_digits = commas_removed
            .chars()
            .map(|c| c.is_ascii_digit())
            .all(|x| x);

        if is_all_digits {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(
                "score can only contain ASCII digits and commas".into(),
            ))
        }
    } else {
        Ok(Validation::Invalid(
            "a score should contain no more than 4 numerals".into(),
        ))
    }
}

#[allow(dead_code)] // TODO: fix this
struct Attempt {
    time: HoursMinutesSeconds,
    score: u16,
    success: bool,
}

struct AttemptsLog {
    start_time: Instant,
    log_entries: Vec<Attempt>,
}

impl AttemptsLog {
    fn new() -> Self {
        AttemptsLog {
            log_entries: Vec::new(),
            start_time: Instant::now(),
        }
    }

    fn add_entry(&mut self, score_str: &str) {
        if self.log_entries.is_empty() {
            self.start_time = Instant::now();
        }

        debug!("score str: {}", &score_str);

        let score = u16::from_str(score_str)
            .expect("this fn should never be called unless this str has been validated");
        if score > 5000 {
            // TODO: add err statement
            return;
        }

        let success = score == 5000;
        let time = HoursMinutesSeconds::from_secs(self.start_time.elapsed().as_secs());

        self.log_entries.push(Attempt {
            time,
            score,
            success,
        });

        info!("added entry: score {score} and success {success}");
    }

    fn print_stats(&self) {
        let (success_count, fail_count, total_score) =
            self.log_entries.iter().fold((0, 0, 0), |acc, x| {
                let success = u16::from(x.success);
                let fail = u16::from(!x.success);
                let score = u32::from(x.score);
                (acc.0 + success, acc.1 + fail, acc.2 + score)
            });
        let time = self.log_entries.last().unwrap().time;

        println!("5k count: {}", success_count);
        println!("Miss count: {}", fail_count);
        println!("Total score: {}", total_score);
        println!("Time (from start to most recent attempt): {:?}", time);
    }

    /// returns Err if there are no entries in the log
    fn fix_prev_entry(&mut self, score_str: &str) -> Result<(), ()> {
        if let Some(last_entry) = self.log_entries.last_mut() {
            let score = u16::from_str(score_str).expect(
                "this fn should never be called unless 'get_cmd_type' has validated this str",
            );
            let success = score == 5000;

            info!("updated entry: score {score} and success {success}");

            last_entry.score = score;
            last_entry.success = success;
            Ok(())
        } else {
            Err(())
        }
    }
}

#[allow(dead_code)] //fix this
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HoursMinutesSeconds {
    hours: u8, //this logger should never run for more than 25 hours
    minutes: u8,
    seconds: u8,
}

impl HoursMinutesSeconds {
    fn from_secs(secs: u64) -> Self {
        let total_mins = secs / 60;
        let seconds = (secs % 60) as u8;
        let minutes = (total_mins % 60) as u8;
        let hours = (total_mins / 60) as u8;
        HoursMinutesSeconds {
            hours,
            minutes,
            seconds,
        }
    }
}

// TODO: add tests
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn hours_mins_secs_converter() {
        let mut rng = rand::thread_rng();
        let hours = rng.gen_range(0..=25);
        let minutes = rng.gen_range(0..60);
        let seconds = rng.gen_range(0..60);

        let total_seconds = ((hours as u64) * 60 * 60) + ((minutes as u64) * 60) + (seconds as u64);

        let correct = HoursMinutesSeconds {
            hours,
            minutes,
            seconds,
        };
        assert_eq!(correct, HoursMinutesSeconds::from_secs(total_seconds));
    }
}
