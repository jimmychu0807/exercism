use std::convert::TryInto;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
	let ub_usize: usize = upper_bound.try_into().unwrap();
	let mut marking = vec![false; ub_usize + 1];

	for i in 2..=ub_usize {
		if marking[i] {
			continue;
		}
		swipe_true(&mut marking, i as u64);
	}

	marking
		.into_iter()
		.enumerate()
		.filter_map(|(ind, val)| if ind > 1 && !val { Some(ind as u64) } else { None })
		.collect::<Vec<u64>>()
}

fn swipe_true(marking: &mut [bool], el: u64) {
	let el_usize = el as usize;
	let marking_len = marking.len();
	(2..marking_len)
		.filter_map(|n| if n * el_usize < marking_len { Some(n * el_usize) } else { None })
		.for_each(|n| marking[n] = true);
}
