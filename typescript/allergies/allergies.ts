const allergiesDict = {
  eggs: 1,
  peanuts: 2,
  shellfish: 4,
  strawberries: 8,
  tomatoes: 16,
  chocolate: 32,
  pollen: 64,
  cats: 128,
};

export class Allergies {
  allergenIndex: number; // 过敏原指数，声明属性

  constructor(allergenIndex: number) {
    this.allergenIndex = allergenIndex; // 赋予 test 里 Allergies() 传入的数值
  }

  public list(): string[] {
    const allergens: string[] = [];
    for (const allergen in allergiesDict) {
      if (this.allergicTo(allergen)) {
        allergens.push(allergen); // 如果过敏则加入过敏原列表
      }
    }
    return allergens;
  }

  public allergicTo(allergen: string): boolean {
    const allergenValue = allergiesDict[allergen as keyof typeof allergiesDict]; // 获取过敏原对应的数值
    return (this.allergenIndex & allergenValue) !== 0; // 判断是否过敏
  }
}
