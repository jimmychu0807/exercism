pub fn is_armstrong_number(num: u32) -> bool {
	let num_str = num.to_string();
	let strlen = num_str.len() as u32;

	let check_summed = num_str
		.chars()
		.map(|c| String::from(c).parse::<u32>().unwrap().pow(strlen))
		.try_fold(0_u32, |acc, val| acc.checked_add(val));

	match check_summed {
		Some(val) => val == num,
		None => false,
	}
}
