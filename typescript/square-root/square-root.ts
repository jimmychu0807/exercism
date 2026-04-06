function isqrt(n: number): number {
  if (n < 0) throw new Error("必须为正整数");

  let x = n; // 我上一个写法想着找一个比被开方数要小的随机数来测，然后这个随机数据比平方根还小而报错，还是用回被开方数开始测比较好安全

  while (true) {
    const next = Math.floor((x + Math.floor(n / x)) / 2); // Heron's method https://en.wikipedia.org/wiki/Square_root_algorithms#Heron's_method
    if (next >= x) break; // 根据公式，每次迭代，下一步的猜测值，应该比当前猜测值小，所当下一轮大于当前猜测值，说明当前轮次最接近结果，于是停止
    x = next;
  }

  return x;
}

export function squareRoot(radicand: number): number {
  return isqrt(radicand);
}
