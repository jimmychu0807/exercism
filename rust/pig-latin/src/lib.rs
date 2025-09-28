use regex_lite::Regex;

pub fn translate(input: &str) -> String {
	input
		.split(' ')
		.map(|word| {
			// rule 1
			if rule1_satisfy(word) {
				// add an "ay" sound to the end of the word.
				return format!("{word}ay");
			}

			// rule 3
			// If a word starts with zero or more consonants followed by "qu",
			// first move those consonants (if any) and the "qu" part to the end of the word,
			//   and then add an "ay" sound to the end of the word.
			let rule3_regex = Regex::new(r"^([^aeiou]*qu)(.*)").unwrap();
			if let Some(caps) = rule3_regex.captures(word) {
				return format!(
					"{}{}ay",
					caps.get(2).unwrap().as_str(),
					caps.get(1).unwrap().as_str()
				);
			}
			// rule 4
			// If a word starts with one or more consonants followed by "y",
			// first move the consonants preceding the "y"to the end of the word,
			//   and then add an "ay" sound to the end of the word.
			let rule4_regex = Regex::new(r"^(([^aeiou]+)y)(.*)").unwrap();
			if let Some(caps) = rule4_regex.captures(word) {
				return format!(
					"y{}{}ay",
					caps.get(3).unwrap().as_str(),
					caps.get(2).unwrap().as_str()
				);
			}

			// rule 2
			// If a word begins with one or more consonants
			let rule2_regex = Regex::new(r"^([^aeiou]+)(.*)").unwrap();
			if let Some(caps) = rule2_regex.captures(word) {
				// First move those consonants to the end of the word and then add an "ay" sound
				//   to the end of the word.
				return format!(
					"{}{}ay",
					caps.get(2).unwrap().as_str(),
					caps.get(1).unwrap().as_str()
				);
			}

			word.to_string()
		})
		.collect::<Vec<String>>()
		.join(" ")
}

fn rule1_satisfy(word: &str) -> bool {
	// If a word begins with a vowel, or starts with "xr" or "yt":
	let vowels_regex = Regex::new(r"^[aeiou]").unwrap();
	let start_regex = Regex::new(r"^((xr)|(yt))").unwrap();

	vowels_regex.is_match(word) || start_regex.is_match(word)
}
