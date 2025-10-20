use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
	stack: Vec<Value>,
	functions: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	DivisionByZero,
	StackUnderflow,
	UnknownWord,
	InvalidWord,
}

const KEYWORDS: &[&str] = &["dup", "drop", "swap", "over"];

impl Forth {
	pub fn new() -> Self {
		Self { stack: Vec::new(), functions: HashMap::new() }
	}

	pub fn stack(&self) -> &[Value] {
		&self.stack
	}

	pub fn eval(&mut self, input: &str) -> Result {

		let input = input.to_lowercase();

		// Determined this is a line defining a function
		if input.split(" ").next().unwrap() == ":" {
			let tokens = input.split(" ").collect::<Vec<&str>>();
			let key = tokens[1];
			let content = tokens[2..(tokens.len() - 1)].join(" ");
			self.functions.insert(key.to_string(), content);

			return Ok(());
		}

		// The 1st scan is to substitute unknown word as function, if existed.
		let input = input.split(" ").try_fold(String::new(), |mut acc, tok| {
			if let Ok(_) = tok.parse::<Value>() {
				acc.push_str(&(String::from(" ") + tok));
			} else if tok == "+" || tok == "-" || tok == "*" || tok == "/" || KEYWORDS.contains(&tok) {
				if self.functions.contains_key(tok) {
					acc.push_str(&(String::from(" ") + self.functions.get(tok).unwrap()));
				} else {
					acc.push_str(&(String::from(" ") + tok));
				}
			} else if self.functions.contains_key(tok) {
				acc.push_str(&(String::from(" ") + self.functions.get(tok).unwrap()));
			} else {
				return Err(Error::UnknownWord);
			}
			Ok(acc)
		})?;

		let input = &input[1..];

		println!("input: {input}");

		// The 2nd scan is to process the token
		for tok in input.split(" ") {
			if let Ok(num) = tok.parse::<Value>() {
				self.stack.push(num);
				continue;
			}

			match tok {
				"+" | "-" | "*" | "/" => {
					let operand2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
					let operand1 = self.stack.pop().ok_or(Error::StackUnderflow)?;

					match tok {
						"+" => self.stack.push(operand1 + operand2),
						"-" => self.stack.push(operand1 - operand2),
						"*" => self.stack.push(operand1 * operand2),
						"/" => {
							if operand2 == 0 {
								return Err(Error::DivisionByZero);
							}
							self.stack.push(operand1 / operand2)
						}
						_ => panic!("unexpected token"),
					}
				}
				"dup" => {
					let operand = self.stack.pop().ok_or(Error::StackUnderflow)?;
					self.stack.push(operand);
					self.stack.push(operand);
				}
				"drop" => {
					let _ = self.stack.pop().ok_or(Error::StackUnderflow)?;
				}
				"swap" => {
					let operand2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
					let operand1 = self.stack.pop().ok_or(Error::StackUnderflow)?;

					self.stack.push(operand2);
					self.stack.push(operand1);
				}
				"over" => {
					if self.stack.len() < 2 {
						return Err(Error::StackUnderflow);
					}
					let second_last = self.stack[self.stack.len() - 2];
					self.stack.push(second_last);
				}
				_ => panic!("unexpected operation"),
			}
		}

		Ok(())
	}
}
