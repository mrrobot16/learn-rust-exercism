use time::PrimitiveDateTime as DateTime;
use std::time::{Duration};

/// Create a datetime from the given numeric point in time.
///
/// Panics if any field is invalid.
pub fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    use time::{Date, Time};

    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // unimplemented!("What time is a gigasecond later than {}", start);
    gigasecond_after(start)
}

fn gigasecond_after(start: DateTime) -> DateTime {
    let start_date = start;
    let target_after_date = dt(2043, 1, 1, 1, 46, 40);
    let GIGASECOND = 1_000_000_000;
    let gigasecond_duration = Duration::from_secs(GIGASECOND);
    let after_date = start_date + gigasecond_duration;
    after_date
}