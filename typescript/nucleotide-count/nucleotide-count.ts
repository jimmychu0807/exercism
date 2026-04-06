const dict = ["A", "C", "G", "T"];
function isInDict(str: string): boolean {
  for (const letter of str.toUpperCase()) {
    // 遍历每个字符，为保险起见我统一为大写
    if (!dict.includes(letter)) {
      // 如果发现有字母不在 dict 里就报错
      return false;
    }
  }
  return true;
}
export function nucleotideCounts(text: string): { A: number; C: number; G: number; T: number } {
  const count: { A: number; C: number; G: number; T: number } = {
    A: 0,
    C: 0,
    G: 0,
    T: 0,
  };

  if (isInDict(text)) {
    for (const char of text) {
      if (char === "A") {
        count.A = count.A + 1;
      }
      if (char === "C") {
        count.C = count.C + 1;
      }
      if (char === "G") {
        count.G = count.G + 1;
      }
      if (char === "T") {
        count.T = count.T + 1;
      }
    }
  } else {
    throw new Error("Invalid nucleotide in strand");
  }

  return count;
}
