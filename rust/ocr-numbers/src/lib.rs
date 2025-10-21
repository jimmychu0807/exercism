use std::collections::HashMap;

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	InvalidRowCount(usize),
	InvalidColumnCount(usize),
}

const ROW_MULTIPLE: usize = 4;
const COL_MULTIPLE: usize = 3;

pub fn convert(input: &str) -> Result<String, Error> {
	// check the dimension
	let rows = input.lines().count();
	if rows % ROW_MULTIPLE != 0 {
		return Err(Error::InvalidRowCount(rows));
	}

	for row in input.lines() {
		if row.len() % COL_MULTIPLE != 0 {
			return Err(Error::InvalidColumnCount(row.len()));
		}
	}

	let mut remaining = input.to_string();
	let mut res = String::new();
	let digit_map = get_digit_map();

	while !remaining.trim().is_empty() {
		let block: String;
		(block, remaining) = take_block(&remaining);
		res.push_str(recognize_block(&block, &digit_map).unwrap_or("?"));

		//if the first 4 char are line break, remove them from remaining and append a "," in the res,
		if remaining.starts_with("\n\n\n\n") {
			remaining = remaining[4..].to_string();
			res.push(',');
		}
	}

	Ok(res)
}

fn take_block(input: &str) -> (String, String) {
	let mut block = String::new();
	let mut remaining = String::new();

	for (row, line) in input.lines().enumerate() {
		if row < ROW_MULTIPLE {
			block.push_str(&format!("{}\n", &line[..COL_MULTIPLE]));
			remaining.push_str(&format!("{}\n", &line[COL_MULTIPLE..]));
		} else {
			remaining.push_str(&format!("{line}\n"));
		}
	}

	// remove the last line break
	let _ = block.pop();
	let _ = remaining.pop();

	(block, remaining)
}

fn recognize_block<'a>(
	block: &'a str,
	digit_map: &'a HashMap<&'a str, &'a str>,
) -> Option<&'a str> {
	digit_map.get(block).copied()
}

fn get_digit_map<'a>() -> HashMap<&'a str, &'a str> {
	let mut map = HashMap::new();

	map.insert(" _ \n| |\n|_|\n   ", "0");
	map.insert("   \n  |\n  |\n   ", "1");
	map.insert(" _ \n _|\n|_ \n   ", "2");
	map.insert(" _ \n _|\n _|\n   ", "3");
	map.insert("   \n|_|\n  |\n   ", "4");
	map.insert(" _ \n|_ \n _|\n   ", "5");
	map.insert(" _ \n|_ \n|_|\n   ", "6");
	map.insert(" _ \n  |\n  |\n   ", "7");
	map.insert(" _ \n|_|\n|_|\n   ", "8");
	map.insert(" _ \n|_|\n _|\n   ", "9");
	map
}
