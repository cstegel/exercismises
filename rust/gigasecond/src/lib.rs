extern crate chrono;
use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let duration = Duration::seconds(1000000000);
    let end = start + duration;
    return end;
}
