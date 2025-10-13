use std::fmt::{Display, Formatter, Result};

pub struct Roman {
	num: u32,
}

impl Display for Roman {
	fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
		let mapping = [
			(1000, "M"),
			(900, "CM"),
			(500, "D"),
			(400, "CD"),
			(100, "C"),
			(90, "XC"),
			(50, "L"),
			(40, "XL"),
			(10, "X"),
			(9, "IX"),
			(5, "V"),
			(4, "IV"),
			(1, "I"),
		];

		let mut remaining = self.num;
		let mut result: String = "".to_string();

		while remaining > 0 {
			for (val, numeral) in mapping {
				if remaining >= val {
					result.push_str(numeral);
					remaining -= val;
					break;
				}
			}
		}

		write!(_f, "{result}")
	}
}

impl From<u32> for Roman {
	fn from(num: u32) -> Self {
		Self { num }
	}
}
