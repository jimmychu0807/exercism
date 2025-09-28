#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
	seq: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
	seq: String,
}

const DNA_CHARS: &str = "ACGT";
const RNA_CHARS: &str = "ACGU";

impl Dna {
	pub fn new(dna: &str) -> Result<Dna, usize> {
		for (idx, c) in dna.chars().enumerate() {
			if !DNA_CHARS.contains(c) {
				return Err(idx);
			}
		}

		Ok(Dna { seq: dna.into() })
	}

	pub fn into_rna(self) -> Rna {
		let rna_seq = self
			.seq
			.chars()
			.map(|c| match c {
				'G' => 'C',
				'C' => 'G',
				'T' => 'A',
				'A' => 'U',
				_ => panic!("unexpected char"),
			})
			.collect::<String>();

		Rna { seq: rna_seq }
	}
}

impl Rna {
	pub fn new(rna: &str) -> Result<Rna, usize> {
		for (idx, c) in rna.chars().enumerate() {
			if !RNA_CHARS.contains(c) {
				return Err(idx);
			}
		}

		Ok(Rna { seq: rna.into() })
	}
}
