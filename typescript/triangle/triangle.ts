export class Triangle {
  a: number;
  b: number;
  c: number;
  constructor(a: number, b: number, c: number) {
    this.a = a;
    this.b = b;
    this.c = c;
  }

  get checkSideZero(): boolean {
    return this.a !== 0 && this.b !== 0 && this.c !== 0;
  }

  get isEquilateral(): boolean {
    return this.a === this.b && this.b === this.c && this.checkSideZero;
  }

  get isIsosceles(): boolean {
    return (
      (this.a === this.b || this.b === this.c || this.a === this.c) &&
      this.a + this.b > this.c &&
      this.c + this.b > this.a &&
      this.a + this.c > this.b &&
      this.checkSideZero
    );
  }

  get isScalene(): boolean {
    return (
      this.a !== this.b &&
      this.b !== this.c &&
      this.a !== this.c &&
      this.a + this.b > this.c &&
      this.c + this.b > this.a &&
      this.a + this.c > this.b &&
      this.checkSideZero
    );
  }
}
