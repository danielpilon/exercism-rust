use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let carry_hours = minutes.div_euclid(60);
        let minutes = minutes.rem_euclid(60);
        let hours = (hours + carry_hours).rem_euclid(24);
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0width$}:{:0width$}",
            self.hours,
            self.minutes,
            width = 2
        )
    }
}
