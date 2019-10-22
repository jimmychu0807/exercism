extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

pub fn verse(n: u32) -> String {
  let verse_zero = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";

  let verse_generic = "{first} of beer on the wall, {first} of beer.\nTake {pronoun} down and pass it around, {second} of beer on the wall.\n";

  let mut verse_vals = HashMap::new();

  if n == 0 {
    strfmt(verse_zero, &verse_vals).unwrap()
  } else if n == 1 {
    verse_vals.insert("first".to_string(), n.to_string() + " bottle");
    verse_vals.insert("second".to_string(), "no more bottles".to_string());
    verse_vals.insert("pronoun".to_string(), "it".to_string());
    strfmt(verse_generic, &verse_vals).unwrap()
  } else {
    let second_bottle = if n - 1 == 1 {"bottle"} else {"bottles"};
    verse_vals.insert("first".to_string(), n.to_string() + " bottles");
    verse_vals.insert("second".to_string(),
      (n - 1).to_string() + " " + second_bottle);
    verse_vals.insert("pronoun".to_string(), "one".to_string());
    strfmt(verse_generic, &verse_vals).unwrap()
  }
}

pub fn sing(start: u32, end: u32) -> String {
  (end..=start)
    .rev()
    .map(|n| verse(n))
    .collect::<Vec<String>>()
    .join("\n")
}
