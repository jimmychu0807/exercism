use modinverse::{egcd, modinverse};

const ALPHABET_LEN: i32 = 26;
const ASCII_LOWER_A: u8 = 97;
const NIBBLE_LEN: usize = 5;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
	NotCoprime(i32),
}

fn alphabet_to_idx(c: char) -> i32 {
	((c as u8) - ASCII_LOWER_A) as i32
}

fn idx_to_alphabet(c: i32) -> char {
	println!("c: {c}");

	(c as u8 + ASCII_LOWER_A) as char
}

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

fn modsubstract(a: i32, b: i32, modulus: i32) -> i32 {
	let amod = a % modulus;
	let bmod = b % modulus;
	if amod > bmod {
		return amod - bmod;
	}

	amod + modulus - bmod
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
	let (g, _, _) = egcd(a, ALPHABET_LEN);
	if g != 1 {
		return Err(AffineCipherError::NotCoprime(a));
	}

	let res = plaintext
		.to_ascii_lowercase()
		.chars()
		.filter(|c| c.is_alphanumeric())
		.map(|c| match c {
			'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => c,
			_ => idx_to_alphabet((alphabet_to_idx(c) * a + b) % ALPHABET_LEN),
		})
		.collect::<String>();

	Ok(expand(&res))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention ein Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
	let (g, _, _) = egcd(a, ALPHABET_LEN);
	if g != 1 {
		return Err(AffineCipherError::NotCoprime(a));
	}

	let modinverse = modinverse(a, ALPHABET_LEN).unwrap();

	let res = ciphertext
		.chars()
		.filter(|c| c.is_alphanumeric())
		.map(|c| match c {
			'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => c,
			_ => idx_to_alphabet(
				(modsubstract(alphabet_to_idx(c), b, ALPHABET_LEN) * modinverse) % ALPHABET_LEN,
			),
		})
		.collect::<String>();

	Ok(res)
}
