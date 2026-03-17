export class Squares {
  count: number;
  constructor(count: number) {
    this.count = count; // 构造指定数字
  }

  get sumOfSquares(): number {
    let sum = 0;
    for (let n = 1; n < this.count + 1; n++) {
      sum += n * n; // 从1开始到指定数字的平方和
    }
    return sum;
  }

  get squareOfSum(): number {
    let sum = 0;
    for (let n = 1; n < this.count + 1; n++) {
      sum += n; // 自然数的和
    }
    return sum * sum; // 和的平方
  }

  get difference(): number {
    return this.squareOfSum - this.sumOfSquares; // 求差
  }
}
