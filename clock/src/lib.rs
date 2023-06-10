#![allow(unused)]
use std::fmt::{Display, Formatter};

const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut res_hours = 0;
        let mut res_minutes = 0;

        res_minutes = minutes % MINUTES_PER_HOUR;
        if res_minutes < 0 {
            res_minutes += MINUTES_PER_HOUR;
            // res_minutes < 0，说明还需要向res_hours再借1位
            res_hours -= 1;
        }

        res_hours = (res_hours + minutes / MINUTES_PER_HOUR + hours) % HOURS_PER_DAY;
        if res_hours < 0 {
            res_hours += HOURS_PER_DAY;
        }

        Clock {
            hours: res_hours,
            minutes: res_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.hours * MINUTES_PER_HOUR + self.minutes + minutes;

        Clock::new(0, total_minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
