pub fn is_armstrong_number(num: u32) -> bool {
  let num_str = num.to_string();
  let strlen = num_str.len() as u32;
  let summed = num_str
    .chars()
    .map(|char| char.to_digit(10).unwrap().pow(strlen))
    .fold(0, |mem, v| mem + v);
  summed == num
}
