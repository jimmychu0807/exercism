export class Clock {
  hour: number
  minute: number
    
  constructor(hour: number, minute: number = 0) {
    this.hour = hour
    this.minute = minute 
    
    while (this.minute >= 60) { 
      this.minute -= 60;
      this.hour += 1;
    }
    while (this.minute < 0) {
      this.minute += 60;
      this.hour -= 1;
    }    
    while (this.hour < 0) {
      this.hour += 24;
    }
    while (this.hour >= 24) {
      this.hour -= 24;
    }    
    // 解决时和分分别超出一圈或者小于0的情况，以统一为24小时和60分钟内的的钟表同等状态
  }

  public toString(): string {
    const h = String(this.hour).padStart(2, '0'); // 两位字符串，用 0 补
    const m = String(this.minute).padStart(2, '0');
    return `${h}:${m}`; // 返回正确的时间格式
  }

  public plus(minutes: number): Clock {
    return new Clock(this.hour, this.minute + minutes); // 增加分钟
  }

  public minus(minutes: number): Clock {
    return new Clock(this.hour, this.minute - minutes); // 减少分钟
  }

  public equals(other: Clock): boolean {
    if (this.hour === other.hour && this.minute === other.minute) {
      return true;   // 判断新旧时针分针位置是否一样，是为真。等值的前提是前边的4个 while 处理
    } else {
      return false;  // 否为假
    }
  }
}
