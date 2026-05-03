function checkInScore(arr: string[], val: string): boolean {
  return arr.some((arrVal) => val === arrVal);
} // https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Array/some#判断数组元素中是否存在某个值

export function score(word: string | undefined): number {
  const score1 = ["A", "E", "I", "O", "U", "L", "N", "R", "S", "T"];
  const score2 = ["D", "G"];
  const score3 = ["B", "C", "M", "P"];
  const score4 = ["F", "H", "V", "W", "Y"];
  const score5 = ["K"];
  const score8 = ["J", "X"];
  const score10 = ["Q", "Z"];

  const scores = [
    { score: score1, value: 1 },
    { score: score2, value: 2 },
    { score: score3, value: 3 },
    { score: score4, value: 4 },
    { score: score5, value: 5 },
    { score: score8, value: 8 },
    { score: score10, value: 10 },
  ];

  let valNum = 0;

  if (word === undefined) {
    return 0;
  }

  for (const x of word) {
    const upperX = x.toUpperCase();
    for (const item of scores) {
      if (checkInScore(item.score, upperX)) {
        valNum += item.value;
        break;
      }
    }
  }
  return valNum;
}
