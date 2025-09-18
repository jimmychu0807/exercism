/// Determines whether the supplied string is a valid ISBN number
const ISBN_LEN: usize = 10;

pub fn is_valid_isbn(isbn: &str) -> bool {
	// remove all `-`
	// check the remaining char are valid
	//   L for first 9 chars, 0-9, the last char is 0-9 or X. If not, then false

	// calculate the checksum mod 11 == 0

	let isbn_string = String::from(isbn).replace("-", "");
	if isbn_string.len() != ISBN_LEN {
		return false;
	}
	for (idx, chr) in isbn_string.chars().enumerate() {
		if idx < ISBN_LEN - 1 && !chr.is_ascii_digit() {
			return false;
		}
		if idx == ISBN_LEN - 1 && !chr.is_ascii_digit() && chr != 'X' {
			return false;
		}
	}

	let checksum = isbn_string.chars().enumerate().fold(0, |acc, (idx, chr)| {
		acc + match chr {
			'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
				chr.to_digit(10).unwrap() * (ISBN_LEN - idx) as u32
			}
			'X' => 10 * (ISBN_LEN - idx) as u32,
			_ => panic!("unexpected character"),
		}
	});

	checksum % 11 == 0
}
