export class Rational {
  numerator: number; // 分子
  denominator: number; // 分母
  constructor(numerator: number, denominator: number) {
    if (denominator === 0) {
      throw new Error("Denominator != 0");
    }
    this.numerator = numerator;
    this.denominator = denominator;
    this.reduce();
  }

  add(other: Rational): Rational {
    return new Rational(
      this.numerator * other.denominator + other.numerator * this.denominator,
      this.denominator * other.denominator,
    ); // (a₁ * b₂ + a₂ * b₁) / (b₁ * b₂)
  }

  sub(other: Rational): Rational {
    return new Rational(
      this.numerator * other.denominator - other.numerator * this.denominator,
      this.denominator * other.denominator,
    ); // (a₁ * b₂ - a₂ * b₁) / (b₁ * b₂)
  }

  mul(other: Rational): Rational {
    return new Rational(this.numerator * other.numerator, this.denominator * other.denominator); // (a₁ * a₂) / (b₁ * b₂)
  }

  div(other: Rational): Rational {
    return new Rational(this.numerator * other.denominator, other.numerator * this.denominator); // (a₁ * b₂) / (a₂ * b₁)
  }

  abs(): Rational {
    return new Rational(Math.abs(this.numerator), Math.abs(this.denominator));
  }

  exprational(n: number): Rational {
    if (n === 0) {
      return new Rational(1, 1);
    }
    if (n < 0) {
      // (a/b)^(-n) = (b/a)^n
      return new Rational(this.denominator ** Math.abs(n), this.numerator ** Math.abs(n));
    }
    return new Rational(this.numerator ** n, this.denominator ** n);
  }

  expreal(x: number): number {
    return x ** (this.numerator / this.denominator);
  }

  reduce(): Rational {
    if (this.numerator === 0) {
      // 处理分子为 0 的情况
      this.numerator = 0;
      this.denominator = 1;
      return this;
    }

    let num = this.numerator;
    let den = this.denominator;

    const gcd = this.findGCD(num, den);
    num = num / gcd;
    den = den / gcd;

    if (den < 0) {
      // 处理分母为负数的情况
      num = -num;
      den = -den;
    }

    this.numerator = num;
    this.denominator = den;

    return this; // 返回自己才能被上边调用
  }

  private findGCD(a: number, b: number): number {
    // 找最大公约数
    // 先转成正数
    a = Math.abs(a);
    b = Math.abs(b);

    const smaller = a < b ? a : b; // 先找到比较小的那个数

    for (let i = smaller; i >= 1; i--) {
      if (a % i === 0 && b % i === 0) {
        return i; // 从小的那个数开始一直找，最先能同时被整除的那个数就是最大公约数
      }
    }
    return 1; // 如果找不到其他，那么最大公约数是1
  }
}
