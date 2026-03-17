const dict: { [key: string]: string } = {
  A: "Z",
  B: "Y",
  C: "X",
  D: "W",
  E: "V",
  F: "U",
  G: "T",
  H: "S",
  I: "R",
  J: "Q",
  K: "P",
  L: "O",
  M: "N",
  N: "M",
  O: "L",
  P: "K",
  Q: "J",
  R: "I",
  S: "H",
  T: "G",
  U: "F",
  V: "E",
  W: "D",
  X: "C",
  Y: "B",
  Z: "A",
  a: "z",
  b: "y",
  c: "x",
  d: "w",
  e: "v",
  f: "u",
  g: "t",
  h: "s",
  i: "r",
  j: "q",
  k: "p",
  l: "o",
  m: "n",
  n: "m",
  o: "l",
  p: "k",
  q: "j",
  r: "i",
  s: "h",
  t: "g",
  u: "f",
  v: "e",
  w: "d",
  x: "c",
  y: "b",
  z: "a",
};

export function encode(plainText: string): string {
  const charts = plainText
    .split("")
    .filter((char) => /[a-zA-Z0-9]/.test(char)) // 只保留字母和数字（否则第4题开始报错）
    .map((char) => {
      const convertedChar = dict[char];
      if (convertedChar) {
        return convertedChar.toLowerCase(); // 题目给出的输出都是小写
      } else {
        return char; // 如果输入的内容在“字典”里不存在，那就直接输出
      }
    });

  let finalText = "";
  let count = 0;

  for (let i = 0; i < charts.length; i++) {
    finalText += charts[i];
    count++;
    if (count === 5) {
      finalText += " "; // 就在后面加一个空格
      count = 0; // 然后把计数器归零，准备数下一组
    }
  }

  return finalText.trimEnd(); // 最后那一位不能是空格
}

export function decode(cipherText: string): string {
  return encode(cipherText).split(" ").join(""); // 因为字典里的字母是一一对应的关系，所以用 encode() 就行，但是结果会多出来空格，需要把空格两边的字符取出再合并就行了
}
