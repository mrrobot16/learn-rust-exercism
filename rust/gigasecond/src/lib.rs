use time::PrimitiveDateTime as DateTime;
use std::time::{Duration};

// Returns a DateTime one billion seconds after start.
// pub fn after(start: DateTime) -> DateTime {
//     unimplemented!("What time is a gigasecond later than {}", start);
//     gigasecond_after(start)
// }

fn gigasecond_after(start: DateTime) -> DateTime {
    let start_date = start;
    const GIGA_SECOND: u64 = 1_000_000_000;
    let gigasecond_duration = Duration::from_secs(GIGA_SECOND);
    let after_date = start_date + gigasecond_duration;
    after_date
}

// Cleaner community solution
use time::ext::NumericalDuration;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + 1e9.seconds()
}
