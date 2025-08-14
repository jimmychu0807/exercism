pub struct Raindrop {
  pub factor: u32,
  pub voice: &'static str,
}

impl Raindrop {
  fn factors() -> Vec<Raindrop> {
    vec![
      Raindrop { factor: 3, voice: "Pling" },
      Raindrop { factor: 5, voice: "Plang" },
      Raindrop { factor: 7, voice: "Plong" },
    ]
  }
}

pub fn raindrops(n: u32) -> String {
  let mut res = String::from("");
  for raindrop in Raindrop::factors() {
    if n % raindrop.factor == 0 {
      res.push_str(raindrop.voice);
    }
  }

  if &res == "" { res = n.to_string(); }
  res
}
