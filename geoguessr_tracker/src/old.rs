use inquire::{
    validator::{MaxLengthValidator, Validation},
    CustomUserError, InquireError, Text,
};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    println!("Commands are always given as a single character. Some commands may ask for additional input.");
    println!("Command list:"); // TODO: update cmd list
    println!("- q: quit");
    println!("- s: print stats");
    println!("- d: add a new entry to the list with the provided score");
    println!("- c: add a new entry to the list by calculating the score based on the total score and the previous entries");
    println!("- f: fix the most recent entry to use the provided score");

    let mut attempts_log = AttemptsLog::new();

    let cmd_prompt = Text::new("enter command>")
        .with_validator(cmd_validator)
        .with_formatter(&cmd_formatter);
    let round_score_prompt = Text::new("enter round score>")
        .with_validators(&[
            Box::new(MaxLengthValidator::new(4)),
            Box::new(score_validator),
        ])
        .with_formatter(&score_formatter);
    let total_score_prompt = Text::new("enter total score>")
        .with_validator(score_validator)
        .with_formatter(&score_formatter);

    loop {
        let input_res = cmd_prompt.clone().prompt();
        if input_res.is_err() {
            println!("error getting input");
            continue;
        }
        let cmd_res = CmdType::from_str(&input_res.unwrap()); // input_res is known to not be an err
        if let Err(err_string) = cmd_res {
            println!("invalid command: {err_string}");
            continue;
        }

        match cmd_res.unwrap() {
            //cmd_res is know to not be an err
            CmdType::Quit => break, // TODO: add saving to file on quit
            CmdType::PrintStats => attempts_log.print_stats(),
            CmdType::AddEntryDirect => {
                let round_score_string = get_score(&round_score_prompt);
                let total_score_string = get_score(&total_score_prompt);
                attempts_log.add_entry_direct(&round_score_string, &total_score_string);
            }
            CmdType::AddEntryCalculated => {
                let total_score_string = get_score(&total_score_prompt);
                attempts_log.add_entry_calculated(&total_score_string);
            }
            CmdType::FixPrev => {
                let total_score_string = get_score(&total_score_prompt);
                let res = attempts_log.fix_prev_entry(&total_score_string);
                if res.is_err() {
                    println!(
                        "unable to fix previous entry because there are no entries in the log!"
                    );
                }
            }
        }
    }
}

fn get_score(score_prompt: &Text) -> String {
    loop {
        let round_score_res = score_prompt.clone().prompt();
        if let Err(score_err) = round_score_res {
            match score_err {
                InquireError::InvalidConfiguration(reason) => {
                    println!("invalid score: {reason}");
                    continue;
                }
                _ => {
                    println!("error getting input");
                    continue;
                }
            }
        };
        return round_score_res.unwrap(); //err case handled by guard clause
    }
}

enum CmdType {
    Quit,
    PrintStats,
    AddEntryDirect,
    AddEntryCalculated,
    FixPrev,
}

impl FromStr for CmdType {
    type Err = String; // TODO: impl custom err type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd_char: char = s.chars().next().unwrap(); //unwrap because validator would have caught it

        match cmd_char {
            Self::QUIT_CHAR => Ok(CmdType::Quit),
            Self::PRINT_STATS_CHAR => Ok(CmdType::PrintStats),
            Self::ADD_ENTRY_DIRECT_CHAR => Ok(CmdType::AddEntryDirect),
            Self::ADD_ENTRY_CALCULATED_CHAR => Ok(CmdType::AddEntryCalculated),
            Self::FIX_PREV_CHAR => Ok(CmdType::FixPrev),
            _ => Err("unkown command character".into()),
        }
    }
}

impl CmdType {
    const QUIT_CHAR: char = 'q';
    const PRINT_STATS_CHAR: char = 's';
    const ADD_ENTRY_DIRECT_CHAR: char = 'd';
    const ADD_ENTRY_CALCULATED_CHAR: char = 'c';
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
fn cmd_formatter(input: &str) -> String {
    input.trim_end().to_string()
}
fn score_validator(input: &str) -> Result<Validation, CustomUserError> {
    let trimmed = input.trim_end();
    if (1..5).contains(&trimmed.len()) {
        let commas_removed: String = trimmed.chars().filter(|c| *c != ',').collect();
        let is_all_digits = commas_removed
            .chars()
            .map(|c| c.is_ascii_digit())
            .fold(true, |acc, x| acc && x);

        if is_all_digits {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(
                "score can only contain ASCII digits and commas".into(),
            ))
        }
    } else {
        Ok(Validation::Invalid("input is too long".into()))
    }
}
fn score_formatter(input: &str) -> String {
    input.trim_end().chars().filter(|c| *c != ',').collect()
}

#[allow(dead_code)] // TODO: fix this
struct Attempt {
    time: HoursMinutesSeconds,
    score: u16,
    success: bool,
}

struct AttemptsLog {
    start_time: Instant,
    total_score: u16,
    log_entries: Vec<Attempt>,
}

impl AttemptsLog {
    fn new() -> Self {
        AttemptsLog {
            log_entries: Vec::new(),
            total_score: 0,
            start_time: Instant::now(),
        }
    }

    fn add_entry_direct(&mut self, score_str: &str, total_score_str: &str) {
        if self.log_entries.is_empty() {
            self.start_time = Instant::now();
        }

        let score = u16::from_str(score_str)
            .expect("this fn should never be called unless this str has been validated");
        let success = score == 5000;

        let total_score = u16::from_str(total_score_str)
            .expect("this fn should never be called unless this str has been validated");

        let prev_total_score = self.total_score;
        if total_score >= prev_total_score {
            if score != (total_score - prev_total_score) {
                println!("Whoops! This situation is not accounted for!");
                return;
            }
        } else {
            if !(score == total_score && score <= 5000) {
                println!("Whoops! This situation is not accounted for!");
                return;
            }
        }

        let time = HoursMinutesSeconds::from_secs(self.start_time.elapsed().as_secs());

        self.log_entries.push(Attempt {
            time,
            score,
            success,
        });
        self.total_score = total_score;

        println!("added entry: score {score} and success {success}, and updated total score to {total_score}");
    }

    fn add_entry_calculated(&mut self, total_score_str: &str) {
        if self.log_entries.is_empty() {
            self.start_time = Instant::now();
        }

        let total_score = u16::from_str(total_score_str)
            .expect("this fn should never be called unless this str has been validated");

        let prev_total_score = self.total_score;

        let score = if total_score >= prev_total_score {
            total_score - prev_total_score
        } else if total_score <= 5000 {
            total_score
        } else {
            println!("Whoops! This situation is not accounted for!");
            return;
        };
        let success = score == 5000;

        let time = HoursMinutesSeconds::from_secs(self.start_time.elapsed().as_secs());

        self.log_entries.push(Attempt {
            time,
            score,
            success,
        });
        self.total_score = total_score;

        println!("added entry: score {score} and success {success}, and updated total score to {total_score}");
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

            println!("added entry: score {score} and success {success}");

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
