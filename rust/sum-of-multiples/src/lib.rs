use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
	let set: HashSet<u32> = factors
		.iter()
		.flat_map(|&base| match base > 0 {
			true => (0..limit).filter(|&v| v % base == 0).collect::<Vec<u32>>(),
			_ => vec![],
		})
		.collect();

	set.into_iter().sum::<u32>()
}
