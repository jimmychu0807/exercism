use std::collections::HashMap;

pub fn encode(n: u64) -> String {
	let english_map = HashMap::from([
		(0, "zero"), (1, "one"), (2, "two"), (3, "three"), (4, "four"),
		(5, "five"), (6, "six"), (7, "seven"), (8, "eight"), (9, "nine"),
		(10, "ten"), (11, "eleven"), (12, "twelve"), (13, "thirteen"), (14, "fourteen"),
		(15, "fifteen"), (16, "sixteen"), (17, "seventeen"), (18, "eighteen"), (19, "nineteen"),
	]);

	let tenth_eng = [
		"", "", "twenty", "thirty", "forty", "fifty",
		"sixty", "seventy", "eighty", "ninety"];

	let thousandth_eng = [
		"thousand", "million", "billion", "trillion", "quadrillion",
		"quintillion"
	];

	let mut nibbles: Vec<&str> = vec![];
	let mut sol: Vec<String> = vec![];

	let n_as_string = n.to_string();
	let first_nibble_len = n_as_string.len() % 3;

	if first_nibble_len != 0 {
		nibbles.push(&n_as_string[0..first_nibble_len]);
	}

	let mut offset = first_nibble_len;
	while offset < n_as_string.len() {
		nibbles.push(&n_as_string[offset..offset + 3]);
		offset += 3;
	}

	for (idx, nibble) in nibbles.iter().enumerate() {
		let mut num = nibble.parse::<usize>().unwrap();

		// process the hundredth
		if num >= 100 {
			let hundredth = num / 100;
			sol.push(english_map.get(&hundredth).unwrap().to_string());
			sol.push("hundred".to_string());
		}

		// process the last two digits
		num %= 100;

		if num >= 20 {
			let tenth = num / 10;
			let single = num % 10;
			if single > 0 {
				sol.push([tenth_eng[tenth], "-", english_map.get(&single).unwrap()].join(""));
			} else {
				sol.push(tenth_eng[tenth].to_string());
			}
		} else {
			// num < 20
			if num > 0 || (nibbles.len() == 1 && sol.is_empty()) {
				// the second check is to only insert "zero" if this is the only digit
				sol.push(english_map.get(&num).unwrap().to_string());
			}
		}

		// insert the nibble_set_eng
		if num > 0 && nibbles.len() > 1 && nibbles.len() - idx > 1 {
			sol.push(thousandth_eng[nibbles.len() - idx - 2].to_string());
		}
	}

	sol.join(" ")
}
