export class Gigasecond {
  inputDate: Date; // 构造 Date 这个实例
  constructor(input: Date) {
    this.inputDate = input;
  }
  public date(): Date {
    const gigasecond = 1000000000; // 十亿秒
    const milliseconds = this.inputDate.getTime(); // 获取毫秒数
    const newMilliseconds = milliseconds + gigasecond * 1000; // 统一单位
    return new Date(newMilliseconds);
  }
}
