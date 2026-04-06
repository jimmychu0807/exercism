export function compute(left: string, right: string): number {
  if (left.length !== right.length) {
    throw new Error("DNA strands must be of equal length.");
  }
  // 把字符串的每个字母抽出来组成两个数组用于每一位进行比较
  const leftArr = left.split("");
  const rightArr = right.split("");
  return leftArr.filter((val, i) => val !== rightArr[i]).length; // 把对应的某一位的不一样的字母拿出来组成一个新数组然后看看它有多少元素
}
