#[derive(Debug)]
pub enum Category {
	Ones,
	Twos,
	Threes,
	Fours,
	Fives,
	Sixes,
	FullHouse,
	FourOfAKind,
	LittleStraight,
	BigStraight,
	Choice,
	Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
	match category {
		Category::Yacht => {
			if dice.iter().all(|&d| d == dice[0]) {
				50
			} else {
				0
			}
		}
		Category::FourOfAKind => {
			if let Some(die) = find_die_with_count(&count(&dice), 4) {
				return die * 4;
			}

			if let Some(die) = find_die_with_count(&count(&dice), 5) {
				return die * 4;
			}

			0
		}
		Category::FullHouse => {
			if find_die_with_count(&count(&dice), 3).is_some() {
				dice.iter().sum()
			} else {
				0
			}
		}
		Category::BigStraight => {
			if (2..=6).all(|v| dice.contains(&(v as u8))) {
				30
			} else {
				0
			}
		}
		Category::LittleStraight => {
			if (1..=5).all(|v| dice.contains(&(v as u8))) {
				30
			} else {
				0
			}
		}
		Category::Choice => dice.iter().sum(),
		Category::Ones => dice.iter().fold(0, |acc, &d| if d == 1 { acc + 1 } else { acc }),
		Category::Twos => dice.iter().fold(0, |acc, &d| if d == 2 { acc + 1 } else { acc }) * 2,
		Category::Threes => dice.iter().fold(0, |acc, &d| if d == 3 { acc + 1 } else { acc }) * 3,
		Category::Fours => dice.iter().fold(0, |acc, &d| if d == 4 { acc + 1 } else { acc }) * 4,
		Category::Fives => dice.iter().fold(0, |acc, &d| if d == 5 { acc + 1 } else { acc }) * 5,
		Category::Sixes => dice.iter().fold(0, |acc, &d| if d == 6 { acc + 1 } else { acc }) * 6,
	}
}

fn count(dice: &[u8]) -> Vec<u8> {
	let mut res = vec![0, 0, 0, 0, 0, 0, 0];

	for d in dice {
		res[*d as usize] += 1;
	}
	res
}

fn find_die_with_count(die_count: &[u8], target_count: u8) -> Option<u8> {
	if target_count == 0 {
		panic!("not expecting 0 in find_die_with_count()");
	}

	for (die, count) in die_count.iter().enumerate() {
		if *count == target_count {
			return Some(die as u8);
		}
	}

	None
}
