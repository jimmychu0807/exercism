pub fn actions(n: u8) -> Vec<&'static str> {
	let words = ["wink", "double blink", "close your eyes", "jump"];
	let mut res: Vec<&str> = vec![];

	if n == 0 {
		return res;
	}

	for i in 0..5 {
		let mask = 1 << i;
		if n & mask > 0 {
			if i < 4 {
				res.push(words[i as usize]);
			} else {
				res = res.into_iter().rev().collect();
			}
		}
	}

	res
}
