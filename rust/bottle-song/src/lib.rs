use std::collections::HashMap;
use strfmt::strfmt;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
	let num_eng = ["no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];

	let get_num_eng = |num: u32, up: bool| -> String {
		match up {
			true => num_eng[num as usize]
				.chars()
				.enumerate()
				.map(|(idx, c)| if idx == 0 { c.to_uppercase().next().unwrap() } else { c })
				.collect::<String>(),
			false => num_eng[num as usize].to_string(),
		}
	};

	let get_bottle_str = |num: u32| -> String {
		match num {
			1 => "bottle",
			_ => "bottles",
		}
		.to_string()
	};

	let one_verse = |bottles: u32| -> String {
		let tpl = concat!(
			"{up_num_eng} green {bottle_maybe_plural} hanging on the wall,\n",
			"{up_num_eng} green {bottle_maybe_plural} hanging on the wall,\n",
			"And if one green bottle should accidentally fall,\n",
			"There'll be {low_num_eng} green {bottle_minusone_maybe_plural} hanging on the wall."
		)
		.to_string();

		let mut vars: HashMap<String, String> = HashMap::new();
		vars.insert("up_num_eng".to_string(), get_num_eng(bottles, true));
		vars.insert("low_num_eng".to_string(), get_num_eng(bottles - 1, false));
		vars.insert("bottle_maybe_plural".to_string(), get_bottle_str(bottles));
		vars.insert("bottle_minusone_maybe_plural".to_string(), get_bottle_str(bottles - 1));

		strfmt(&tpl, &vars).expect("should be a valid string")
	};

	(0..take_down)
		.enumerate()
		.map(|(idx, _)| one_verse(start_bottles - idx as u32))
		.collect::<Vec<String>>()
		.join("\n\n")
}
