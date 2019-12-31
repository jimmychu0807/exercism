// use poker::{self, PokerHand};
use poker::{ self, Hand as PokerHand };
use core::cmp::{ Ordering };

fn main() -> Result<(), String> {
  let poker1 = PokerHand::new("4S 5S 7H 8D JC")?;
  // Nothing(11, 8, 7, 5, 4)
  println!("Hand: {:?} | Pattern: {:?}", poker1, poker1.pattern());

  // let poker1a = PokerHand::new("4S 5S 6H 8D JC");
  // let poker1b = PokerHand::new("4S 5S 6H 9D JC");

  let poker2 = PokerHand::new("4D 5D 7D 8D 6D")?;
  // StraightFlush(8)
  println!("Hand: {:?} | Pattern: {:?}", poker2, poker2.pattern());

  let poker3 = PokerHand::new("JD QD AD KD 10D")?;
  // StraightFlush(14)
  println!("Hand: {:?} | Pattern: {:?}", poker3, poker3.pattern());

  let poker3a = PokerHand::new("2D 3C 4D AD 5D")?;
  // Straight(5)
  println!("Hand: {:?} | Pattern: {:?}", poker3a, poker3a.pattern());

  let poker4 = PokerHand::new("2D JD QD KD AD")?;
  // Flush(14, 13, 12, 11, 2)
  println!("Hand: {:?} | Pattern: {:?}", poker4, poker4.pattern());

  let poker5 = PokerHand::new("2S 2H 2C 2D AD")?;
  // FourOfAKind(2, 14)
  println!("Hand: {:?} | Pattern: {:?}", poker5, poker5.pattern());

  let poker6 = PokerHand::new("5C 2S 2H 2C 5D")?;
  // FullHouse(2, 5)
  println!("Hand: {:?} | Pattern: {:?}", poker6, poker6.pattern());

  let poker7 = PokerHand::new("2S 3D 4D 2H 2C")?;
  // ThreeOfAKind(2, 4, 3)
  println!("Hand: {:?} | Pattern: {:?}", poker7, poker7.pattern());

  let poker8 = PokerHand::new("2S 2H 3C 3D 4D")?;
  // TwoPair(3, 2, 4)
  println!("Hand: {:?} | Pattern: {:?}", poker8, poker8.pattern());

  let poker9 = PokerHand::new("2S 2H 3C 4D 5D")?;
  // TwoPair(2, 5, 4, 3)
  println!("Hand: {:?} | Pattern: {:?}", poker9, poker9.pattern());

  // assert_eq!(poker1.eq(&poker1), true);
  // assert_eq!(poker1.partial_cmp(&poker1a), Some(Ordering::Greater));
  // assert_eq!(poker1.partial_cmp(&poker1b), Some(Ordering::Less));
  Ok(())
}
