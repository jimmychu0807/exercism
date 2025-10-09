use rand::{Rng, distr::Alphabetic};
use std::{
	collections::HashSet,
	sync::{LazyLock, Mutex},
};

pub struct Robot {
	name: String,
}

static ROBOT_NAMES: LazyLock<Mutex<HashSet<String>>> =
	LazyLock::new(|| Mutex::new(HashSet::<String>::new()));

impl Robot {
	pub fn new() -> Self {
		let mut name;
		let mut set = ROBOT_NAMES.lock().unwrap();

		loop {
			name = generate_rand_name();

			if !set.contains(&name) {
				set.insert(name.clone());
				break;
			}
		}

		Robot { name }
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn reset_name(&mut self) {
		let mut set = ROBOT_NAMES.lock().unwrap();
		set.remove(&self.name);

		let mut name;
		loop {
			name = generate_rand_name();

			if !set.contains(&name) {
				set.insert(name.clone());
				break;
			}
		}

		self.name = name;
	}
}

fn generate_rand_name() -> String {
	let mut rng = rand::rng();

	let chars: String =
		(0..2).map(|_| (rng.sample(Alphabetic) as char).to_ascii_uppercase()).collect();
	let digits = format!("{:03}", rng.random_range(0..1000));
	format!("{chars}{digits}")
}
