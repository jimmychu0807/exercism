pub fn brackets_are_balanced(string: &str) -> bool {
  let mut stack: Vec<char> = Vec::new();
  for b in string.chars() {
    if b == '(' || b == '[' || b == '{' {
      stack.push(b);
      continue;
    }

    if b == ')' || b == ']' || b == '}' {
      let last = match stack.pop() {
        Some(val) => val,
        None => return false,
      };
      let match_bracket = match last {
        last if last == '(' => b == ')',
        last if last == '[' => b == ']',
        last if last == '{' => b == '}',
        _ => return false,
      };

      if !match_bracket { return false; }
    }
  }
  stack.is_empty()
}
