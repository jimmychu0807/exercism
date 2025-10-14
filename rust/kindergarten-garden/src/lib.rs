use std::collections::HashMap;

const STUDENTS: [&str; 12] = [
	"Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
	"Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
	let plant_map = [('G', "grass"), ('C', "clover"), ('R', "radishes"), ('V', "violets")]
		.into_iter()
		.collect::<HashMap<_, _>>();

	let data = diagram
		.split('\n')
		.map(|row| row.chars().collect::<Vec<char>>())
		.collect::<Vec<Vec<char>>>();

	let plant_idx =
		STUDENTS.iter().enumerate().find(|&(_, &s)| s == student).map(|(idx, _)| idx).unwrap();

	[
		data[0][2 * plant_idx],
		data[0][2 * plant_idx + 1],
		data[1][2 * plant_idx],
		data[1][2 * plant_idx + 1],
	]
	.into_iter()
	.map(|char| *plant_map.get(&char).unwrap())
	.collect::<Vec<_>>()
}
