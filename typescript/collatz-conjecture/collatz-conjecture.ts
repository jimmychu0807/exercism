export function steps(count: number): number {
  let total = 0;
  if (count === 0 || count < 0 || !Number.isInteger(count)) {
    throw new Error("Only positive integers are allowed");
  }
  while (count !== 1) {
    if (count % 2 === 0) {
      count = count / 2;
    } else {
      count = count * 3 + 1;
    }
    total++;
  }
  return total;
}
