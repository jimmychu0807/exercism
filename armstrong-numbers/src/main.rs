use armstrong_numbers as lib;

fn main() {
  let val = 5;
  println!("{:?}: {:?}", val, lib::is_armstrong_number(val));
  let val = 10;
  println!("{:?}: {:?}", val, lib::is_armstrong_number(val));
  let val = 153;
  println!("{:?}: {:?}", val, lib::is_armstrong_number(val));
  let val = 100;
  println!("{:?}: {:?}", val, lib::is_armstrong_number(val));
  let val = 9474;
  println!("{:?}: {:?}", val, lib::is_armstrong_number(val));
}
