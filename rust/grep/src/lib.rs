use anyhow::Error;
use std::{fs::File, io::Read};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
pub struct Flags {
	b_linenum: bool,
	b_filename: bool,
	b_case_insensitive: bool,
	b_invert: bool,
	b_entire: bool,
}

impl Flags {
	pub fn new(flags: &[&str]) -> Self {
		let mut res = Self {
			b_linenum: false,
			b_filename: false,
			b_case_insensitive: false,
			b_invert: false,
			b_entire: false,
		};

		for &flag in flags {
			match flag {
				"-n" => {
					res.b_linenum = true;
				}
				"-l" => {
					res.b_filename = true;
				}
				"-i" => {
					res.b_case_insensitive = true;
				}
				"-v" => {
					res.b_invert = true;
				}
				"-x" => {
					res.b_entire = true;
				}
				_ => panic!("unknown flag"),
			}
		}

		res
	}
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
	let mut res = Vec::new();

	for file_name in files {
		let mut handle = File::open(file_name)?;
		let mut contents = String::new();
		let _ = handle.read_to_string(&mut contents)?;

		for (line_idx, line) in contents.lines().enumerate() {
			if filter_line(line, pattern, flags) {
				output_line(&mut res, file_name, line_idx, line, flags);
			}
		}
	}

	Ok(res)
}

fn filter_line(line: &str, pattern: &str, flags: &Flags) -> bool {
	let mut line = line.to_string();
	let mut pattern = pattern.to_string();

	if flags.b_case_insensitive {
		line = line.to_uppercase();
		pattern = pattern.to_uppercase();
	}

	if flags.b_entire { line == pattern } else { line.contains(&pattern) }
}

fn output_line(res: &mut Vec<String>, file_name: &str, line_idx: usize, line: &str, flags: &Flags) {
	if flags.b_linenum {
		res.push(format!("{}:{}", line_idx + 1, line));
	} else if flags.b_filename {
		res.push(file_name.to_string());
	} else {
		res.push(line.to_string());
	}
}
