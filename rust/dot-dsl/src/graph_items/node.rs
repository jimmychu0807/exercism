use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Node {
	pub name: String,
	attrs: HashMap<String, String>,
}

impl Node {
	pub fn new(name: &str) -> Self {
		Self { name: String::from(name), attrs: HashMap::new() }
	}

	pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
		for (key, val) in attrs {
			self.attrs.insert(key.to_string(), val.to_string());
		}

		self
	}

	pub fn attr(&self, key: &str) -> Option<&str> {
		self.attrs.get(key).map(|v| v.as_str())
	}
}
