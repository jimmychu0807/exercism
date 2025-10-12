pub fn number(user_number: &str) -> Option<String> {
	// remove all alphanumeric code
	let mut cleaned: String = user_number.chars().filter(|c| c.is_numeric()).collect();

	// if the first digit is 1, remove it
	if cleaned[0..1] == *"1" {
		cleaned = cleaned[1..].to_string();
	}
	// the remaining digits should be 10 in length. Otherwise reject it.
	if cleaned.len() != 10 {
		return None;
	}
	// 1st digit should be 2-9. Otherwise reject it.
	let first_digit = &cleaned[0..1];
	if first_digit.parse::<u16>().unwrap() <= 1 {
		return None;
	}

	// 4th digit should be 2-9. Otherwise reject it.
	let forth_digit = &cleaned[3..4];
	if forth_digit.parse::<u16>().unwrap() <= 1 {
		return None;
	}

	Some(cleaned)
}
