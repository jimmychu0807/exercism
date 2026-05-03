export function classify(num: number): string {
  // 真因数之和
  // https://zhuanlan.zhihu.com/p/717046843
  // 6 = 1+2+3
  let sum = 0;
  for (let i = 1; i < num; i++) {
    if (num % i === 0) sum += i;
  }
  if (num <= 0) {
    throw new Error("Classification is only possible for natural numbers.");
  }
  if (sum === num) {
    return "perfect";
  }
  if (sum < num) {
    return "deficient";
  } else {
    return "abundant";
  }
}
