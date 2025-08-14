use matching_brackets as mb;

fn main() {
  println!("{}", mb::brackets_are_balanced("]"));
  println!("{}", mb::brackets_are_balanced("["));
  println!("{}", mb::brackets_are_balanced("[)"));
  println!("{}", mb::brackets_are_balanced("[]"));
  println!("{}", mb::brackets_are_balanced("{[]}"));
  println!("{}", mb::brackets_are_balanced("{([])}"));
  println!("{}", mb::brackets_are_balanced("{[}]"));
}
