pub fn encrypt(input: &str) -> String {
	let processed: String =
		input.to_ascii_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();

	if processed.len() <= 1 {
		return processed.to_string();
	}

	let (_, c) = find_r_c(&processed);

	encoding(&processed, c)
}

fn find_r_c(string: &str) -> (usize, usize) {
	let mut r = 1;
	let mut c = 1;
	while r * c < string.len() {
		if r == c {
			c += 1;
		} else {
			r += 1;
		}
	}

	(r, c)
}

fn encoding(string: &str, cols: usize) -> String {
	let mut blocks: Vec<String> = vec![];
	let mut sol_vec: Vec<String> = vec![];

	let mut offset = 0_usize;
	while offset < string.len() {
		let ending = (offset + cols).min(string.len());
		blocks.push(format!("{:1$}", &string[offset..ending], cols));

		offset = ending;
	}

	for c in 0..cols {
		sol_vec
			.push(blocks.iter().map(|phrase| phrase.chars().nth(c).unwrap()).collect::<String>());
	}

	sol_vec.join(" ")
}
