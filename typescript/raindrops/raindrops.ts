export function convert(num: number): string {
  let result = "";
  if (num === 1) {
    result = "1";
  }
  if (num % 3 === 0) {
    result += "Pling";
  }
  if (num % 5 === 0) {
    result += "Plang";
  }
  if (num % 7 === 0) {
    result += "Plong";
  }
  if (result === "") {
    result = num.toString(); // 当以上情况都不满足时，result依然为空，这时候直接返回 num, 但是类型上必须是 string
  }
  return result;
}
