use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
	let mut transformed: BTreeMap<char, i32> = BTreeMap::new();

	for val in h.keys() {
		let vec = h.get(val).unwrap();

		for k in vec.iter() {
			transformed.insert((*k).to_ascii_lowercase(), *val);
		}
	}

	transformed
}
