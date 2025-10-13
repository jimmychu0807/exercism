use rand::{Rng, rng};

const ASCII_LC_A: u8 = 97;
const ENG_CHAR_NUM: u8 = 26;

pub fn encode(key: &str, s: &str) -> Option<String> {
	rotate_with_key(key, s, true)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
	rotate_with_key(key, s, false)
}

pub fn encode_random(s: &str) -> (String, String) {
	let key = build_key(100);
	let ct = rotate_with_key(&key, s, true).unwrap();

	(key, ct)
}

fn build_key(len: u32) -> String {
	let charset: Vec<char> = ('a'..='z').collect();
	let mut rng = rng();

	(0..len).map(|_| charset[rng.random_range(0..charset.len())]).collect()
}

fn rotate_with_key(key: &str, s: &str, encode: bool) -> Option<String> {
	if key.chars().any(|c| !c.is_ascii_lowercase()) || key.is_empty() {
		return None;
	}

	let lengthened_key =
		if s.len() > key.len() { key.repeat(s.len() / key.len() + 1) } else { key.to_string() };

	let res = s
		.chars()
		.zip(lengthened_key.chars())
		.map(|(c, k)| rotate(c, (k as u8) - ASCII_LC_A, encode))
		.collect::<String>();

	Some(res)
}

fn rotate(c: char, offset: u8, encode: bool) -> char {
	if !c.is_ascii_lowercase() {
		panic!("not expecting the string has non-lowercase char");
	}

	let multiplier: i16 = if encode { 1 } else { -1 };

	(add_mod(c as u8 - ASCII_LC_A, offset as i16 * multiplier, ENG_CHAR_NUM) + ASCII_LC_A) as char
}

fn add_mod(val1: u8, val2: i16, base: u8) -> u8 {
	let mut intermediate: i16 = val1 as i16 + val2;

	if intermediate >= 0 {
		return intermediate as u8 % base;
	}

	while intermediate < 0 {
		intermediate += base as i16;
	}

	intermediate as u8
}
