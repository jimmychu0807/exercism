use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let mut result = HashSet::<&'a str>::new();
  let lower_word = word.to_lowercase();

  for test in possible_anagrams {
    let lower_test = test.to_lowercase();
    if lower_word == lower_test || !is_anagram(&lower_word, &lower_test) { continue }
    result.insert(*test);
  }
  result
}

fn is_anagram(word1: &str, word2: &str) -> bool {
  let mut chars_cnt = HashMap::<char, u32>::new();
  word1.chars().for_each(|c| {
    match chars_cnt.contains_key(&c) {
      true => chars_cnt.insert(c, chars_cnt.get(&c).unwrap() + 1),
      false => chars_cnt.insert(c, 1),
    };
  });

  for cw2 in word2.chars() {
    match chars_cnt.get(&cw2) {
      None => return false,
      Some(cw2_cnt) if *cw2_cnt <= 0 => return false,
      Some(cw2_cnt) => chars_cnt.insert(cw2, cw2_cnt - 1),
    };
  }

  chars_cnt.iter().filter(|(_, v)| **v > 0).count() == 0
}
