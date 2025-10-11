const ZERO_ASCII_CODE: u8 = 48;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	SpanTooLong,
	InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
	if string_digits.len() < span {
		return Err(Error::SpanTooLong);
	}
	if let Some(chr) = string_digits.chars().find(|c| !c.is_numeric()) {
		return Err(Error::InvalidDigit(chr));
	}

	let max: u64 = (0..=(string_digits.len() - span)).fold(0, |max, start_offset| {
		let digits = &string_digits[start_offset..(start_offset + span)];
		let product =
			digits.chars().fold(1_u64, |prod, chr| prod * (chr as u8 - ZERO_ASCII_CODE) as u64);
		max.max(product)
	});

	Ok(max)
}
