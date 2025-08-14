// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

type Second = u64;

#[derive(Debug)]
pub struct Duration(Second);

impl From<Second> for Duration {
  fn from(s: Second) -> Self {
    Duration(s)
  }
}

pub trait Planet {
  const PERIOD_TO_EARTH_YR: f64;
  const EARTH_YR_SEC: u64 = 31557600;

  fn years_during(d: &Duration) -> f64 {
    (d.0 as f64) / (Self::EARTH_YR_SEC as f64 * Self::PERIOD_TO_EARTH_YR)
  }
}

#[macro_export]
macro_rules! implement_planet {
  ($planet: ident, $ratio: expr) => {
    pub struct $planet;
    impl Planet for $planet {
      const PERIOD_TO_EARTH_YR: f64 = $ratio;
    }
  };
}

implement_planet!(Earth, 1.0);
implement_planet!(Mercury, 0.2408467);
implement_planet!(Venus, 0.61519726);
implement_planet!(Mars, 1.8808158);
implement_planet!(Jupiter, 11.862615);
implement_planet!(Saturn, 29.447498);
implement_planet!(Uranus, 84.016846);
implement_planet!(Neptune, 164.79132);

