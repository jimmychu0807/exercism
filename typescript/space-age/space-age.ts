const EARTH_YEAR_IN_SECONDS = 31557600;

const orbit_ratio = {
  mercury: 0.2408467,
  venus: 0.61519726,
  earth: 1,
  mars: 1.8808158,
  jupiter: 11.862615,
  saturn: 29.447498,
  uranus: 84.016846,
  neptune: 164.79132,
};

export type Planet = keyof typeof orbit_ratio;

export function age(forPlanet: Planet, seconds: number): number {
  if (!(forPlanet in orbit_ratio)) {
    throw new Error(`Unknown planet: ${forPlanet}`);
  }

  const earthYears = seconds / EARTH_YEAR_IN_SECONDS;
  const planetYears = earthYears / orbit_ratio[forPlanet];

  return Number(planetYears.toFixed(2));
}
