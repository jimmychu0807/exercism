use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
	let mut set = HashSet::new();

	for i in 1..=(sum / 3) {
		for j in (i + 1)..((sum - i) / 2 + i) {
			let k = sum - i - j;

			if i >= j || j >= k {
				continue;
			}

			if i.pow(2) + j.pow(2) == k.pow(2) {
				set.insert([i, j, k]);
			}
		}
	}

	set
}
