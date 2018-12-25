// use std::cmp::PartialEq;
use std::convert::From;
use std::ops::Add;
use std::ops::Rem;
use std::string::ToString;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

///
/// Mathematical modulus operator (not remaineder)
/// Calculates x = num (mod base)
fn modulus<T>(num: T, base: T) -> T
where
    T: Rem<Output = T> + Add<Output = T> + Copy,
{
    (base + (num % base)) % base
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let min_remainder = minutes % 60;
        let wrapped_min = modulus(min_remainder, 60);
        let min_overflow = (minutes / 60) - ((min_remainder < 0) as i32);
        let wrapped_hour = modulus(hours + min_overflow, 24);

        Clock {
            hours: wrapped_hour as u8,
            minutes: wrapped_min as u8,
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours as i32, self.minutes as i32 + minutes)
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:>02}:{:>02}", self.hours, self.minutes)
    }
}

impl From<Clock> for String {
    fn from(c: Clock) -> String {
        c.to_string()
    }
}
