use std::time::Instant;

use log::info;

#[derive(Debug)]
struct LogEntries {
    scores: Vec<u16>,
    times: Vec<u32>,
}

pub(crate) struct MarathonLog {
    pub(crate) marathon_duration_secs: u32, // this timer should not run for 136 years
    pub(crate) current_epoch: Option<Instant>,
    pub(crate) epoch_offset_secs: u32,
    pub(crate) total_5ks: u16,
    log_entries: LogEntries,
}

pub(crate) enum AddEntryResult {
    Ok,
    TimerPaused,
    ImpossibleScore { score: u16 },
}

impl MarathonLog {
    pub(crate) fn new(duration: u32) -> Self {
        MarathonLog {
            marathon_duration_secs: duration,
            current_epoch: None,
            epoch_offset_secs: 0,
            total_5ks: 0,
            log_entries: LogEntries {
                scores: Vec::new(),
                times: Vec::new(),
            },
        }
    }

    pub(crate) fn try_add_entry(&mut self, score: u16) -> AddEntryResult {
        if score > 5000 {
            return AddEntryResult::ImpossibleScore { score };
        }

        let mut res = AddEntryResult::Ok;
        let time_since_epoch = match self.current_epoch {
            Some(epoch) => epoch.elapsed().as_secs() as u32,
            None => {
                res = AddEntryResult::TimerPaused;
                0
            }
        };
        let time_seconds = time_since_epoch + self.epoch_offset_secs;

        self.log_entries.scores.push(score);
        self.log_entries.times.push(time_seconds);
        if score == 5000 {
            self.total_5ks += 1;
        }

        info!("added entry: score {score} at time {time_seconds}");
        res
    }

    pub(crate) fn estimate_pace(&self) -> Option<u32> {
        let time_since_epoch = match self.current_epoch {
            Some(epoch) => epoch.elapsed().as_secs() as u32,
            None => 0,
        };
        let current_time = self.epoch_offset_secs + time_since_epoch;
        if current_time == 0 || self.total_5ks == 0 {
            return None;
        };

        let pace = f64::from(self.total_5ks) / f64::from(current_time);
        let remaining_duration = f64::from(self.marathon_duration_secs - current_time);
        let remaining_estimate = pace * remaining_duration;
        let remaining_estimate = remaining_estimate.trunc() as u32;
        Some(u32::from(self.total_5ks) + remaining_estimate)
    }

    pub(crate) fn time_since_last_5k(&self) -> Option<u32> {
        let time_since_epoch = match self.current_epoch {
            Some(epoch) => epoch.elapsed().as_secs() as u32,
            None => 0,
        };
        let current_time = self.epoch_offset_secs + time_since_epoch;
        assert_eq!(self.log_entries.scores.len(), self.log_entries.times.len());
        for i in (0..self.log_entries.times.len()).rev() {
            if self.log_entries.scores[i] == 5000 {
                return Some(current_time - self.log_entries.times[i]);
            }
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn print_entries(&self) {
        assert_eq!(self.log_entries.scores.len(), self.log_entries.times.len());
        for i in 0..self.log_entries.times.len() {
            println!(
                "entry: score {} and time {}",
                self.log_entries.scores[i], self.log_entries.times[i]
            );
        }
    }

    /* pub(crate) fn print_stats(&self) {
        let (success_count, miss_count, total_score) =
            self.log_entries.iter().fold((0, 0, 0), |acc, x| {
                let score = u32::from(x.score);
                let success = score == 5000;
                let fail = u16::from(!success);
                let success = u16::from(success);
                (acc.0 + success, acc.1 + fail, acc.2 + score)
            });
        let time = self
            .log_entries
            .last()
            .map_or(0, |entry| entry.time_seconds);

        if success_count != self.total_5ks {
            error!(
                "success_count ({success_count}) is desynced with success in log ({})",
                self.total_5ks
            );
        }

        println!();
        println!("5k count: {success_count}");
        println!("Miss count: {miss_count}");
        println!("Total score: {total_score}");
        println!("Time (in seconds, from start to most recent attempt): {time}");
        println!();
    } */
}
