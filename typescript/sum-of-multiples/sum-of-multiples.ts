export function sum(factors: number[], limit: number): number {
  const multiples: number[] = []; // 定义一个数组来存储所有的倍数
  for (const factor of factors) {
    if (factor === 0) continue; // 解决 "the only multiple of 0 is 0" 的死循环问题
    for (let m = factor; m < limit; m += factor) {
      // 求小于limit的倍数
      if (!multiples.includes(m)) {
        //避免重复
        multiples.push(m);
      }
    }
  }

  let total = 0;
  for (const multiple of multiples) {
    total += multiple; // 根据题目要求，求倍数之和
  }

  return total;
}
