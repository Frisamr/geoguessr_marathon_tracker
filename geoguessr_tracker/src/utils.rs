use std::str::FromStr;

use log::debug;

use timekeeping::*;

pub(crate) fn calculate_countdown(time_since_epoch: u32, epoch_offset_secs: u32) -> String {
    let time_passed = time_since_epoch + epoch_offset_secs;
    let time_remaining = TWENTY_FOUR_HOURS_IN_SECS.saturating_sub(time_passed);

    time_counter(time_remaining)
}

pub(crate) fn time_counter(raw_secs: u32) -> String {
    let time_hms = HoursMinutesSeconds::from_secs(raw_secs);
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

pub(crate) fn score_from_str(input_str: &str) -> Result<u16, String> {
    if input_str.lines().count() < 1 {
        return Err("no lines in input".to_owned());
    }
    let first_line = input_str.lines().next().unwrap(); // string is know to have at least one line
    let mut score_str = String::new();
    for ch in first_line.chars() {
        if ch.is_ascii_digit() {
            score_str.push(ch);
            continue;
        } else if ch.is_whitespace() || ch == ',' {
            continue;
        } else {
            return Err(input_str.to_owned());
        }
    }
    debug!("score str: {}", &score_str);

    match u16::from_str(&score_str) {
        Ok(score) => Ok(score),
        Err(e) => Err(e.to_string()),
    }
}

pub mod timekeeping {
    use std::{num::ParseIntError, str::FromStr};

    pub const TWENTY_FOUR_HOURS_IN_SECS: u32 = 60 * 60 * 24;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HoursMinutesSeconds {
        pub hours: u8, //this logger should never run for more than 25 hours
        pub minutes: u8,
        pub seconds: u8,
    }

    impl HoursMinutesSeconds {
        pub fn from_secs(secs: u32) -> Self {
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

        pub fn from_strs(hours: &str, mins: &str, secs: &str) -> Result<Self, ParseIntError> {
            let hours_int = u8::from_str(&hours)?;
            let mins_int = u8::from_str(&mins)?;
            let secs_int = u8::from_str(&secs)?;
            Ok(HoursMinutesSeconds {
                hours: hours_int,
                minutes: mins_int,
                seconds: secs_int,
            })
        }

        pub fn total_secs(&self) -> u32 {
            ((self.hours as u32) * 60 * 60) + ((self.minutes as u32) * 60) + (self.seconds as u32)
        }
    }
}

// TODO: add more tests
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn countdown_test() {
        let total_time: u32 = (60 * 60 * 22) + (60 * 58) + 59;
        let mut rng = rand::thread_rng();
        let epoch_offset_secs = rng.gen_range(0..=total_time);
        let time_since_epoch = total_time - epoch_offset_secs;

        let res = calculate_countdown(time_since_epoch, epoch_offset_secs);
        assert_eq!(res, "01:01:01");
    }

    #[test]
    fn hours_mins_secs_conv_test() {
        let mut rng = rand::thread_rng();
        let hours = rng.gen_range(0..=25);
        let minutes = rng.gen_range(0..60);
        let seconds = rng.gen_range(0..60);

        let total_seconds = ((hours as u32) * 60 * 60) + ((minutes as u32) * 60) + (seconds as u32);

        let correct = HoursMinutesSeconds {
            hours,
            minutes,
            seconds,
        };
        assert_eq!(correct, HoursMinutesSeconds::from_secs(total_seconds));
    }
}
