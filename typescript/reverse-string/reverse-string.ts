export function reverse(str: string): string {
  const arr = str.split("");
  const reverseArr = arr.reverse();
  return reverseArr.join("");
}
