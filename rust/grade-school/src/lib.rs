use std::collections::{BTreeMap, HashMap, btree_map::Entry};
pub struct School {
	grades: BTreeMap<u32, Vec<String>>,
	students: HashMap<String, u32>,
}

impl School {
	pub fn new() -> Self {
		Self { grades: BTreeMap::new(), students: HashMap::new() }
	}

	pub fn add(&mut self, grade: u32, student: &str) {
		// check if the student has been added
		if self.students.contains_key(student) {
			return;
		}

		let grade_mod = if let Entry::Vacant(e) = self.grades.entry(grade) {
			e.insert(Vec::new());
			self.grades.get_mut(&grade).unwrap()
		} else {
			self.grades.get_mut(&grade).unwrap()
		};

		grade_mod.push(student.to_string());
		grade_mod.sort();
		self.students.insert(student.to_string(), grade);
	}

	pub fn grades(&self) -> Vec<u32> {
		let mut grades: Vec<_> = self.grades.keys().cloned().collect();
		grades.sort();
		grades
	}

	// If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
	// internally to lend out. By returning an owned vector of owned `String`s instead,
	// the internal structure can be completely arbitrary. The tradeoff is that some data
	// must be copied each time `grade` is called.
	pub fn grade(&self, grade: u32) -> Vec<String> {
		self.grades.get(&grade).cloned().unwrap_or(Vec::new())
	}
}
