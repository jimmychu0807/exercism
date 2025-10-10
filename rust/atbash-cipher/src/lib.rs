const NIBBLE_LEN: usize = 5;

fn expand(str: &str) -> String {
	let mut res = String::from("");
	for (idx, c) in str.chars().enumerate() {
		if idx != 0 && idx % NIBBLE_LEN == 0 {
			res.push(' ');
		}
		res.push(c);
	}

	res
}

fn to_atbash_cipher(target: char) -> char {
	let chars: Vec<char> = ('a'..='z').collect();
	let rev_chars: Vec<char> = ('a'..='z').rev().collect();

	rev_chars[chars.iter().position(|&c| c == target).unwrap()]
}

fn from_atbash_cipher(target: char) -> char {
	let chars: Vec<char> = ('a'..='z').collect();
	let rev_chars: Vec<char> = ('a'..='z').rev().collect();

	chars[rev_chars.iter().position(|&c| c == target).unwrap()]
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
	let res = plain
		.to_ascii_lowercase()
		.chars()
		.filter(|c| c.is_alphanumeric())
		.map(|c| match c {
			'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => c,
			_ => to_atbash_cipher(c),
		})
		.collect::<String>();

	expand(&res)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
	cipher
		.chars()
		.filter(|c| c.is_alphanumeric())
		.map(|c| match c {
			'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => c,
			_ => from_atbash_cipher(c),
		})
		.collect::<String>()
}
