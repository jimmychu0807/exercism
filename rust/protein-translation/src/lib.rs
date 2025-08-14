use std::collections::HashMap;

const STOP_STR: &str = "stop codon";

pub struct CodonsInfo<'a> {
  rna_map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
  pub fn name_for(&self, codon: &str) -> Option<&'a str> {
    self.rna_map.get(codon).copied()
  }

  pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
    let mut result: Vec<&'a str> = Vec::new();
    let mut rna = rna;

    while !rna.is_empty() {
      let three_chars: &str = if rna.len() >= 3 { &rna[0..3] } else { &rna[0..] };
      rna = &rna[three_chars.len()..];
      if !self.rna_map.contains_key(three_chars) { return None }
      let val = self.rna_map.get(three_chars).unwrap();
      if *val == STOP_STR { return Some(result) }
      result.push(*val);
    }
    Some(result)
  }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
  let mut info = CodonsInfo{ rna_map: HashMap::new() };
  for (codon, name) in pairs {
    info.rna_map.insert(codon, name);
  }

  info
}
