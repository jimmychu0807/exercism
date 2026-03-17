export class Matrix {
  private text: string;
  constructor(text: string) {
    this.text = text;
  }

  get rows(): number[][] {
    const lines = this.text.split("\n"); // 将字符串风格成行

    const result: number[][] = []; // 定义保存下来的 matrix 格式

    for (const line of lines) {
      const stringNumbers = line.split(" "); // 把每行的字符串内容进行分割
      const rowOfNumbers: number[] = []; // 定义每一行的数组
      for (const s of stringNumbers) {
        const num = parseInt(s, 10); // 把每行数组里的字符串改为数字
        rowOfNumbers.push(num); // 填充每行数组的数字内容
      }
      result.push(rowOfNumbers); // 把所有数组添加到大数组里
    }

    return result;
  }

  get columns(): number[][] {
    const allRows = this.rows;
    const result: number[][] = [];

    if (allRows.length === 0) return []; // 没有数据时直接返回

    const rowCount = allRows.length; // 行数
    const columnCount = allRows[0].length; // 用第一行的字数确定列数

    for (let i = 0; i < columnCount; i++) {
      // 循环每一列
      const currentColumn: number[] = [];

      for (let j = 0; j < rowCount; j++) {
        const targetNumber = allRows[j][i];
        currentColumn.push(targetNumber); // 获取每一列里各行数字，组成的新数组
      }

      result.push(currentColumn); // 把数组编程一个大数组
    }

    return result;
  }
}
