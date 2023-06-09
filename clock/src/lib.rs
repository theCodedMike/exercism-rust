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

        if hours >= 0 && minutes >= 0 {
            res_minutes = minutes % MINUTES_PER_HOUR;
            res_hours = ((minutes / MINUTES_PER_HOUR) + hours) % HOURS_PER_DAY;
        } else if hours >= 0 && minutes < 0 {
            res_minutes = minutes % MINUTES_PER_HOUR;
            if res_minutes != 0 {
                res_minutes += MINUTES_PER_HOUR;
            }

            res_hours = minutes / MINUTES_PER_HOUR;
            if res_minutes != 0 {
                res_hours -= 1;
            }
            res_hours = (res_hours % HOURS_PER_DAY) + (hours % HOURS_PER_DAY);
            if res_hours < 0 {
                res_hours += HOURS_PER_DAY;
            }
        } else if hours < 0 && minutes >= 0 {
            res_minutes = minutes % MINUTES_PER_HOUR;
            res_hours = hours % HOURS_PER_DAY + minutes / MINUTES_PER_HOUR;
            if res_hours < 0 {
                res_hours += HOURS_PER_DAY;
            }
        } else {
            res_minutes = minutes % MINUTES_PER_HOUR;
            if res_minutes != 0 {
                res_minutes += MINUTES_PER_HOUR;
            }
            res_hours = minutes / MINUTES_PER_HOUR;
            if res_minutes != 0 {
                res_hours -= 1;
            }

            res_hours = ((res_hours % HOURS_PER_DAY) + (hours % HOURS_PER_DAY)) % HOURS_PER_DAY;
            if res_hours < 0 {
                res_hours += HOURS_PER_DAY;
            }
        }

        Clock {
            hours: res_hours,
            minutes: res_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.minutes + minutes;
        let mut res_minutes = 0;
        let mut res_hours = 0;

        if total_minutes >= 0 {
            res_minutes = total_minutes % MINUTES_PER_HOUR;
            res_hours = ((total_minutes / MINUTES_PER_HOUR) + self.hours) % HOURS_PER_DAY;
        } else {
            res_minutes = total_minutes % MINUTES_PER_HOUR;
            if res_minutes != 0 {
                res_minutes += MINUTES_PER_HOUR;
            }

            res_hours = total_minutes / MINUTES_PER_HOUR;
            if res_minutes != 0 {
                res_hours -= 1;
            }
            let total_hours = self.hours + res_hours;
            if total_hours >= 0 {
                res_hours = total_hours % HOURS_PER_DAY;
            } else {
                res_hours = total_hours % HOURS_PER_DAY;
                if res_hours != 0 {
                    res_hours += HOURS_PER_DAY;
                }
            }
        }

        Clock {
            hours: res_hours,
            minutes: res_minutes,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
