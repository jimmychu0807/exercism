use std::string::ToString;

pub struct Luhn<T> {
	data: T,
}

impl<T: ToString> Luhn<T> {
	pub fn is_valid(&self) -> bool {
		let luhn_transform = |i: usize, c: char| -> u32 {
			let num = String::from(c).parse::<u32>().unwrap();
			match i % 2 {
				0 => num,
				1 if num * 2 < 10 => num * 2,
				_ => num * 2 - 9,
			}
		};

		// stripped out all space
		let stripped: Vec<char> =
			self.data.to_string().chars().filter(|c| *c != ' ').collect::<Vec<char>>();

		// cannot be 1 char or less
		if stripped.len() <= 1 {
			return false;
		}

		// only digits and space are allowed
		if stripped.iter().any(|c| *c < '0' || *c > '9') {
			return false;
		}

		stripped.into_iter().rev().enumerate().map(|(i, c)| luhn_transform(i, c)).sum::<u32>() % 10
			== 0
	}
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn<T> {
	fn from(input: T) -> Self {
		Self { data: input }
	}
}
