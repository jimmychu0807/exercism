pub fn abbreviate(phrase: &str) -> String {
	let extract = |t: &str| -> String {
		let up_first = [t[0..1].to_uppercase(), t[1..].to_string()].join("");
		up_first
			.chars()
			.enumerate()
			.filter_map(|(i, c)| {
				if
					// either the 1st char
					i == 0 ||
					// or an uppercase preceded by a lowercase char
					(i > 0 && c.is_ascii_uppercase() && up_first.chars().nth(i-1)?.is_ascii_lowercase())
				{
					Some(c)
				} else {
					None
				}
			})
			.collect::<String>()
	};

	// 1. remove punctuations
	let punctuations = &[',', '_', '\'', '!'];
	let processed = phrase.chars().filter(|c| !punctuations.contains(c)).collect::<String>();

	// 2. split on "-" and space
	processed
		.split(&[' ', '-'][..])
		.filter(|&t| !t.is_empty())
		.map(extract)
		.collect::<Vec<String>>()
		.join("")
}
