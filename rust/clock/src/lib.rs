use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour, self.minute)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let sum_minutes = Self::extract_minutes(hours * 60 + minutes);

        Self {
            hour: sum_minutes / 60,
            minute: sum_minutes % 60,
        }
    }

    fn extract_minutes(minutes: i32) -> i32 {
        let rest = if minutes > 0 {
            minutes % (60 * 24)
        } else {
            60 * 24 + (minutes % (60 * 24))
        };

        if rest == 60 * 24 {
            0
        } else {
            rest
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let sum_minutes = self.hour * 60 + self.minute + minutes;
        let extracted = Self::extract_minutes(sum_minutes);

        return Self {
            hour: extracted / 60,
            minute: extracted % 60,
        };
    }
}
