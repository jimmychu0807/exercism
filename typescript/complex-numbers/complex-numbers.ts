export class ComplexNumber {
  private realPart: number;
  private imagPart: number;
  constructor(real: number, imaginary: number) {
    this.realPart = real;
    this.imagPart = imaginary;
  }

  public get real(): number {
    return this.realPart;
  }

  public get imag(): number {
    return this.imagPart;
  }

  public add(other: ComplexNumber): ComplexNumber {
    const newReal = this.realPart + other.realPart;
    const newImag = this.imagPart + other.imagPart;
    return new ComplexNumber(newReal, newImag);
  }

  public sub(other: ComplexNumber): ComplexNumber {
    const newReal = this.realPart - other.realPart;
    const newImag = this.imagPart - other.imagPart;
    return new ComplexNumber(newReal, newImag);
  }

  public div(other: ComplexNumber): ComplexNumber {
    const denominator = other.realPart * other.realPart + other.imagPart * other.imagPart;
    if (denominator === 0) {
      throw new Error("Cannot divide by 0");
    }
    const newReal = (this.realPart * other.realPart + this.imagPart * other.imagPart) / denominator;
    const newImag = (this.imagPart * other.realPart - this.realPart * other.imagPart) / denominator;
    return new ComplexNumber(newReal, newImag);
  }

  public mul(other: ComplexNumber): ComplexNumber {
    const newReal = this.realPart * other.realPart - this.imagPart * other.imagPart;
    const newImag = this.imagPart * other.realPart + this.realPart * other.imagPart;
    return new ComplexNumber(newReal, newImag);
  }

  public get abs(): number {
    return Math.sqrt(this.realPart * this.realPart + this.imagPart * this.imagPart);
  }

  public get conj(): ComplexNumber {
    if (this.imagPart === 0) {
      return new ComplexNumber(this.realPart, this.imagPart);
    } else {
      return new ComplexNumber(this.realPart, -this.imagPart);
    }
  }

  public get exp(): ComplexNumber {
    const ea = Math.exp(this.realPart);
    return new ComplexNumber(ea * Math.cos(this.imagPart), ea * Math.sin(this.imagPart));
  }
}
