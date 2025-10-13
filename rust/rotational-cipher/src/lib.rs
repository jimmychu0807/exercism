use std::collections::HashMap;

const ASCII_UC_A: u32 = 65;
const ASCII_LC_A: u32 = 97;
const ENG_CHAR_NUM: u32 = 26;

pub fn rotate(input: &str, key: u8) -> String {
	let lookup_table = build_lookup_table(key);

	input
		.chars()
		.map(|c| if let Some(v) = lookup_table.get(&c) { *v } else { c })
		.collect::<String>()
}

fn build_lookup_table(key: u8) -> HashMap<char, char> {
	let rotate_iter = ('A'..='Z').chain('a'..='z');
	let mut lookup = HashMap::new();

	for c in rotate_iter {
		let offset = if c.is_uppercase() { ASCII_UC_A } else { ASCII_LC_A };

		let idx = ((c as u32) - offset + key as u32) % ENG_CHAR_NUM;
		lookup.insert(c, ((idx + offset) as u8) as char);
	}

	lookup
}
