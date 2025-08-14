fn n_bottles_str(n: u32) -> String {
  match n {
    0 => format!("no more bottles"),
    1 => format!("1 bottle"),
    _ => format!("{} bottles", n)
  }
}

pub fn verse(n: u32) -> String {
  match n {
    0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
    _ => format!("{0} of beer on the wall, {0} of beer.\nTake {1} down and pass it around, {2} of beer on the wall.\n",
      n_bottles_str(n),
      if n == 1 {"it"} else {"one"},
      n_bottles_str(n - 1)
    )
  }
}

pub fn sing(start: u32, end: u32) -> String {
  (end..=start)
    .rev()
    .map(|n| verse(n))
    .collect::<Vec<String>>()
    .join("\n")
}
