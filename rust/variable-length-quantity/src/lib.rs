#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
	values.iter().fold(Vec::<u8>::new(), |mut acc, val| {
		acc.extend(&base128_encode(*val));
		acc
	})
}

fn base128_encode(value: u32) -> Vec<u8> {
	let mut sol: Vec<u8> = vec![];
	let mut remaining = value;
	let mut first = true;

	if value == 0 {
		return vec![0_u8];
	}

	while remaining > 0 {
		let shifted = (remaining & 0b1111111) as u8;
		if first {
			sol.push(shifted);
			first = false;
		} else {
			sol.push(shifted | 0b10000000);
		}

		remaining >>= 7;
	}

	sol.reverse();
	sol
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
	// check the last bytes to see if it is an error
	let last_byte = bytes[bytes.len() - 1];
	if last_byte & 0b10000000 > 1 {
		return Err(Error::IncompleteNumber);
	}

	let mut sol: Vec<u32> = vec![];
	let mut start_offset = 0;

	while start_offset < bytes.len() {
		let mut end_offset = start_offset;
		while bytes[end_offset] & 0b10000000 > 1 {
			end_offset += 1;
		}
		sol.push(base128_decode(&bytes[start_offset..=end_offset]));
		start_offset = end_offset + 1;
	}

	Ok(sol)
}

fn base128_decode(bytes: &[u8]) -> u32 {
	bytes.iter().fold(0u32, |acc, val| {
		let masked = val & 0b01111111;
		acc << 7 | (masked as u32)
	})
}
