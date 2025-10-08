use std::{
	cmp::Ordering,
	collections::HashMap,
	fmt::{Display, Error, Formatter},
};

pub enum MatchResult {
	Win,
	Loss,
	Draw,
}

pub struct Results {
	win: u32,
	loss: u32,
	draw: u32,
}

impl Results {
	pub fn new(res: &MatchResult) -> Self {
		match res {
			MatchResult::Win => Self { win: 1, loss: 0, draw: 0 },
			MatchResult::Loss => Self { win: 0, loss: 1, draw: 0 },
			MatchResult::Draw => Self { win: 0, loss: 0, draw: 1 },
		}
	}

	pub fn win(&mut self) {
		self.win += 1;
	}

	pub fn loss(&mut self) {
		self.loss += 1;
	}

	pub fn draw(&mut self) {
		self.draw += 1;
	}

	pub fn mp(&self) -> u32 {
		self.win + self.loss + self.draw
	}

	pub fn p(&self) -> u32 {
		self.win * 3 + self.draw
	}
}

pub struct Scores(HashMap<String, Results>);

impl Scores {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn parse(&mut self, line: &str) {
		let els = line.split(';').collect::<Vec<_>>();

		match els[2] {
			"win" => {
				self.0
					.entry(els[0].into())
					.and_modify(|e| e.win())
					.or_insert(Results::new(&MatchResult::Win));

				self.0
					.entry(els[1].into())
					.and_modify(|e| e.loss())
					.or_insert(Results::new(&MatchResult::Loss));
			}
			"draw" => {
				self.0
					.entry(els[0].into())
					.and_modify(|e| e.draw())
					.or_insert(Results::new(&MatchResult::Draw));

				self.0
					.entry(els[1].into())
					.and_modify(|e| e.draw())
					.or_insert(Results::new(&MatchResult::Draw));
			}
			"loss" => {
				self.0
					.entry(els[0].into())
					.and_modify(|e| e.loss())
					.or_insert(Results::new(&MatchResult::Loss));

				self.0
					.entry(els[1].into())
					.and_modify(|e| e.win())
					.or_insert(Results::new(&MatchResult::Win));
			}
			_ => {
				panic!("should get in here");
			}
		}
	}
}

impl Display for Scores {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		let mut result_vec: Vec<String> = vec![];
		let mut team_scores: Vec<(String, u32)> = vec![];

		result_vec.push(format!(
			"{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
			"Team", "MP", "W", "D", "L", "P"
		));

		for key in self.0.keys() {
			team_scores.push((key.to_string(), self.0.get(key).unwrap().p()));
		}

		team_scores.sort_by(|a, b| match b.1.cmp(&a.1) {
			Ordering::Less => Ordering::Less,
			Ordering::Greater => Ordering::Greater,
			Ordering::Equal => a.0.cmp(&b.0),
		});

		for entry in team_scores.iter() {
			let results = self.0.get(&entry.0).unwrap();
			result_vec.push(format!(
				"{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
				entry.0,
				results.mp(),
				results.win,
				results.draw,
				results.loss,
				results.p()
			));
		}

		let _ = write!(f, "{}", result_vec.join("\n"));
		Ok(())
	}
}

pub fn tally(match_results: &str) -> String {
	let mut scores = Scores::new();

	if !match_results.is_empty() {
		for line in match_results.split('\n') {
			scores.parse(line);
		}
	}

	format!("{scores}")
}
