use std::collections::HashMap;

pub struct CodonsInfo<'a> {
  rna_map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
  pub fn name_for(&self, codon: &str) -> Option<&'a str> {
    self.rna_map.get(codon).map(|val| *val)
  }

  pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
    let mut result: Vec<&'a str> = Vec::new();



    while rna.len() > 0 {
      let three_chars = if
    }
  }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
  let mut info = CodonsInfo{ rna_map: HashMap::new() };
  for (codon, name) in pairs {
    info.rna_map.insert(codon, name);
  }

  info
}
