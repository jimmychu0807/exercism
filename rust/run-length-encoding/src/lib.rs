use regex_lite::Regex;

pub fn encode(source: &str) -> String {
	let mut storage: Vec<char> = vec![];
	let mut result: String = "".to_string();

	let dump_storage_out = |storage: &mut Vec<char>| -> String {
		let mut result = String::from("");

		if storage.is_empty() {
			return result;
		}

		let last_char = storage.last().unwrap();
		if storage.len() == 1 {
			result = format!("{}", *last_char);
		} else {
			result = format!("{}{}", storage.len(), *last_char);
		}
		// and clear the storage
		storage.clear();
		result
	};

	for c in source.chars() {
		if !storage.is_empty() && *storage.last().unwrap() != c {
			result = format!("{}{}", result, dump_storage_out(&mut storage));
		}
		storage.push(c);
	}
	result = format!("{}{}", result, dump_storage_out(&mut storage));

	result
}

pub fn decode(source: &str) -> String {
	let regex = Regex::new(r"(\d*)(.)").unwrap();
	let mut result = String::from("");

	for cap in regex.captures_iter(source) {
		if cap[1].is_empty() {
			result.push_str(&cap[2]);
		} else {
			let repeat = cap[1].parse::<u32>().unwrap() as usize;
			result.push_str(&cap[2].repeat(repeat));
		}
	}

	result
}
