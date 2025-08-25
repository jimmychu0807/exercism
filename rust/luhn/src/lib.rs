/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
	let luhn_transform = |i: usize, c: char| -> u32 {
		let num = String::from(c).parse::<u32>().unwrap();
		match i % 2 {
			0 => num,
			1 if num * 2 < 10 => num * 2,
			_ => num * 2 - 9,
		}
	};

	// stripped out all space
	let stripped: Vec<char> = code.chars().filter(|c| *c != ' ').collect::<Vec<char>>();

	// cannot be 1 char or less
	if stripped.len() <= 1 {
		return false;
	}

	// only digits and space are allowed
	if stripped.iter().any(|c| *c < '0' || *c > '9') {
		return false;
	}

	stripped
		.into_iter()
		.rev()
		.enumerate()
		.map(|(i, c)| luhn_transform(i, c))
		.sum::<u32>()
		% 10 == 0
}
