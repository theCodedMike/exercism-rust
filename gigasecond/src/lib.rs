use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let interval = 1_000_000_000;

    let duration = Duration::new(interval, 0);

    start + duration
}
