export function score(x: number, y: number): number {
  const distSquared = x * x + y * y;
  if (distSquared <= 1) return 10;
  if (distSquared <= 25) return 5;
  if (distSquared <= 100) return 1;
  return 0;
}
