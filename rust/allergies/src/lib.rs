use enum_iterator::{Sequence, all};

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Sequence, Clone)]
pub enum Allergen {
	Eggs,
	Peanuts,
	Shellfish,
	Strawberries,
	Tomatoes,
	Chocolate,
	Pollen,
	Cats,
}

impl Allergies {
	pub fn new(score: u32) -> Self {
		Self(score)
	}

	pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
		let shift =
			all::<Allergen>().enumerate().find(|(_, a)| a == allergen).map(|(i, _)| i).unwrap();

		(self.0 >> shift) % 2 == 1
	}

	pub fn allergies(&self) -> Vec<Allergen> {
		let mut result: Vec<Allergen> = Vec::new();

		let mut dividend = self.0;
		let all_allergens = all::<Allergen>().collect::<Vec<_>>();

		for allergen in &all_allergens {
			if dividend == 0 {
				break;
			}
			if dividend & 1 == 1 {
				result.push(allergen.clone());
			}
			dividend >>= 1;
		}

		result
	}
}
