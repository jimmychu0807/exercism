export function transform(oldData: { [key: string]: string[] }): { [key: string]: number } {
  const result: { [key: string]: number } = {};

  for (const scoreKey in oldData) {
    const score = Number(scoreKey);
    for (const letter of oldData[scoreKey]) {
      result[letter.toLowerCase()] = score; // 把数据填到 { [key: string]: number } = {}; 里
    }
  }

  return result;
}
