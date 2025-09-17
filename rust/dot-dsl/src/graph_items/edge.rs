use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Edge {
	from: String,
	to: String,
	attrs: HashMap<String, String>,
}

impl Edge {
	pub fn new(from: &str, to: &str) -> Self {
		Self { from: from.to_string(), to: to.to_string(), attrs: HashMap::new() }
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
