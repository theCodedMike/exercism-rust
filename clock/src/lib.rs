#![allow(unused)]
use std::fmt::{Display, Formatter};

const MINUTES_PER_HOUR: i64 = 60;
const MINUTES_PER_DAY: i64 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        let minutes = (((hours * MINUTES_PER_HOUR + minutes) % MINUTES_PER_DAY) + MINUTES_PER_DAY)
            % MINUTES_PER_DAY;

        Clock { minutes }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_PER_HOUR,
            self.minutes % MINUTES_PER_HOUR
        )
    }
}
