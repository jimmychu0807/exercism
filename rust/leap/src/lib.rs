pub fn is_leap_year(year: u64) -> bool {
	match year % 4 {
		0 if year % 100 == 0 && year % 400 == 0 => true,
		0 if year % 100 == 0 => false,
		0 => true,
		_ => false,
	}
}
