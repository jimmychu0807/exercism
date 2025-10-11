const A_ASCII_CODE: u8 = 65;

pub fn get_diamond(c: char) -> Vec<String> {
	let gen_row = |i: u8, size: u8| -> String {
		let chr = get_char_from_offset(i);
		(0..size)
			.map(|idx| if idx == size / 2 + i || idx == size / 2 - i { chr } else { ' ' })
			.collect()
	};

	let mut sol_vec: Vec<String> = vec![];
	let char_offset = (c as u8) - A_ASCII_CODE;
	let size = char_offset * 2 + 1;

	for i in 0..=char_offset {
		sol_vec.push(gen_row(i, size));
	}

	for i in (0..char_offset).rev() {
		sol_vec.push(gen_row(i, size));
	}

	sol_vec
}

fn get_char_from_offset(offset: u8) -> char {
	(offset + A_ASCII_CODE) as char
}
