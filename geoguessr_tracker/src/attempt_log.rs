use super::{error, info, warn, Instant};

pub(crate) struct Attempt {
    time_seconds: u64,
    score: u16,
}

pub(crate) struct AttemptsLog {
    pub(crate) current_epoch: Option<Instant>,
    pub(crate) epoch_offset_secs: u64,
    pub(crate) total_5ks: u16,
    pub(crate) log_entries: Vec<Attempt>,
}

impl AttemptsLog {
    pub(crate) fn new() -> Self {
        AttemptsLog {
            current_epoch: None,
            epoch_offset_secs: 0,
            total_5ks: 0,
            log_entries: Vec::new(),
        }
    }

    pub(crate) fn add_entry(&mut self, score: u16) {
        if score > 5000 {
            // TODO: add err statement
            error!("impossible score: {score}");
            return;
        }

        let time_since_epoch = match self.current_epoch {
            Some(epoch) => epoch.elapsed().as_secs(),
            None => {
                warn!("entries should NOT be added while the timer is paused!");
                0
            }
        };
        let time_seconds = time_since_epoch + self.epoch_offset_secs;

        self.log_entries.push(Attempt {
            time_seconds,
            score,
        });
        if score == 5000 {
            self.total_5ks += 1;
        }

        info!("added entry: score {score} at time {time_seconds}");
    }

    pub(crate) fn print_stats(&self) {
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
    }
}
