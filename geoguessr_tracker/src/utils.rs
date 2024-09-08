pub mod timekeeping {
    pub const TWENTY_FOUR_HOURS_IN_SECS: u64 = 60 * 60 * 24;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HoursMinutesSeconds {
        pub hours: u8, //this logger should never run for more than 25 hours
        pub minutes: u8,
        pub seconds: u8,
    }

    impl HoursMinutesSeconds {
        pub fn from_secs(secs: u64) -> Self {
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
}

// TODO: add more tests
#[cfg(test)]
mod tests {
    use super::timekeeping::HoursMinutesSeconds;
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
