export enum Colors {
  black = 0,
  brown = 1,
  red = 2,
  orange = 3,
  yellow = 4,
  green = 5,
  blue = 6,
  violet = 7,
  grey = 8,
  white = 9,
}

type Color = keyof typeof Colors;

export function decodedResistorValue(colors: Color[]): string {
  if (colors.length < 3) {
    throw new Error('At least three color bands are required');
  }

  const [color1, color2, color3] = colors;

  const base = Colors[color1] * 10 + Colors[color2];
  const exp = Colors[color3]; 

  const totalOhms = base * Math.pow(10, exp);

  if (totalOhms >= 1_000_000_000 && totalOhms % 1_000_000_000 === 0) {
    return `${totalOhms / 1_000_000_000} gigaohms`;
  }
  if (totalOhms >= 1_000_000 && totalOhms % 1_000_000 === 0) {
    return `${totalOhms / 1_000_000} megaohms`;
  }
  if (totalOhms >= 1_000 && totalOhms % 1_000 === 0) {
    return `${totalOhms / 1_000} kiloohms`;
  }

  return `${totalOhms} ohms`;
}