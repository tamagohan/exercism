use std::fmt;

const HOUR: i32 = 60;
const DAY: i32 = HOUR * 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hour = self.minutes / HOUR;
        let minute = self.minutes % HOUR;
        write!(f, "{:0>2}:{:0>2}", hour, minute)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: Self::extract_minutes(hours * 60 + minutes),
        }
    }

    fn extract_minutes(minutes: i32) -> i32 {
        ((minutes % DAY) + DAY) % DAY
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Self {
            minutes: Self::extract_minutes(self.minutes + minutes),
        };
    }
}
