pub fn build_proverb(list: &[&str]) -> String {
  if list.len() == 0 { return String::new(); }

  let mut word_pair: Vec<(&str, &str)> = Vec::new();
  list.iter().enumerate().for_each(|(i, word)| {
    if i == list.len() - 1 { return; }
    word_pair.push((word, list[i + 1]));
  });

  let mut proverb = word_pair
    .iter()
    .map(|(first, second)| {
      format!("For want of a {} the {} was lost.\n", first, second)
    })
    .collect::<Vec<String>>()
    .join("");
  proverb.push_str(&format!("And all for the want of a {}.", list[0]));
  proverb
}
