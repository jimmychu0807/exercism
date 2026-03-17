export class Robot {
  private static allNames: string[] = []; //为了避免随机生成的名字有重复，先定义一个数组用于存放已生成的合规名字

  private _name: string;

  constructor() {
    this._name = this.uniqueName();
  }

  private uniqueName(): string {
    let newName = this.randomName();
    while (Robot.allNames.includes(newName)) {
      newName = this.randomName(); // 如果现存名单里有新名字存在，就重新生成一个新名字
    }
    Robot.allNames.push(newName); // 添加到名单
    return newName; // 返回该名字
  }

  private randomName(): string {
    const letter1 = String.fromCharCode(65 + Math.floor(Math.random() * 26));
    const letter2 = String.fromCharCode(65 + Math.floor(Math.random() * 26));
    const digit1 = Math.floor(Math.random() * 10);
    const digit2 = Math.floor(Math.random() * 10);
    const digit3 = Math.floor(Math.random() * 10);
    return `${letter1}${letter2}${digit1}${digit2}${digit3}`;
  }

  public get name(): string {
    return this._name;
  }

  public resetName(): void {
    this._name = this.uniqueName(); // 生成新名字
  }

  public static releaseNames(): void {
    Robot.allNames = []; // 重置名单
  }
}
