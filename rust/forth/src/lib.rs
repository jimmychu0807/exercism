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

	pub fn new_with_functions(functions: &HashMap<String, String>) -> Self {
		Self { stack: Vec::new(), functions: functions.clone() }
	}

	// input will be mutated
	fn override_substitution(
		&self,
		input: &str,
		b_inner_eval: bool,
	) -> std::result::Result<String, Error> {
		let mut substituted;
		let mut res = input.to_string();

		// Keep subsituting until no more substition is performed
		loop {
			substituted = false;

			res = res.split(" ").try_fold(String::new(), |mut acc, tok| {
				if tok.parse::<Value>().is_ok() {
					acc.push_str(&(String::from(" ") + tok));
				} else if tok == "+"
					|| tok == "-" || tok == "*"
					|| tok == "/" || KEYWORDS.contains(&tok)
				{
					if self.functions.contains_key(tok) {
						// overrided, read from functions map
						acc.push_str(&(String::from(" ") + self.functions.get(tok).unwrap()));
						substituted = true;
					} else {
						// default functionality
						acc.push_str(&(String::from(" ") + tok));
					}
				} else if self.functions.contains_key(tok) {
					// overrided, read from functions map
					acc.push_str(&(String::from(" ") + self.functions.get(tok).unwrap()));
					substituted = true;
				} else {
					return Err(Error::UnknownWord);
				}

				// remove the empty space before
				Ok(acc.trim().to_string())
			})?;

			if !substituted || res.is_empty() {
				break;
			}
		}

		// evaluate the inner expression
		if b_inner_eval && !res.is_empty() {
			let mut mac = Forth::new_with_functions(&self.functions);
			let res_copy = res.clone();

			if mac.inner_eval(&res).is_ok() {
				res = mac.stack().iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
			} else {
				res = res_copy;
			}
		}

		Ok(res)
	}

	pub fn eval(&mut self, input: &str) -> Result {
		let mut input = input.to_lowercase();

		// Determined this is a line defining a function
		if input.split(" ").next().unwrap() == ":" {
			let tokens = input.split(" ").collect::<Vec<&str>>();
			let key = tokens[1];

			// Do not allow overriding number
			if key.parse::<Value>().is_ok() {
				return Err(Error::InvalidWord);
			}

			let mut content = tokens[2..(tokens.len() - 1)].join(" ");
			content = self.override_substitution(&content, true)?;

			self.functions.insert(key.to_string(), content);

			return Ok(());
		}

		input = self.override_substitution(&input, false)?;
		self.inner_eval(&input)
	}

	pub fn inner_eval(&mut self, input: &str) -> Result {
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
