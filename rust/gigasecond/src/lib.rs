use time::{Duration, PrimitiveDateTime as DateTime};

const GIGA_SECONDS: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
	start.checked_add(Duration::seconds(GIGA_SECONDS)).unwrap()
}
