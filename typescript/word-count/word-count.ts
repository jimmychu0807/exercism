export function count(str: string): Map<string, number> {
  const words = str
    .toLowerCase() // 后边的 test 大小写统一统计，所以用了 toLocaleLowerCase()
    .split(/[^a-z0-9']+/) // 用数字和小写字母以外的其他字符比如空格和标点符号进行分割
    .map((w) => w.replace(/^'+|'+$/g, "")) // 把数组里开头结尾包含引号的元素的引号去掉用于统计
    .filter((w) => w.length > 0); // 过滤空字符

  const counts = new Map() as Map<string, number>; // 之前直接 new Map()，test 通过但是编辑器报错，查得这样写可以修复
  for (const word of words) {
    if (counts.has(word)) {
      counts.set(word, (counts.get(word) ?? 0) + 1); // 如果当前单词已经存在，它的数量在原有读取数量的基础上+1。
    } else {
      counts.set(word, 1); // 否则直接设为1作为初始数值。
    }
  }
  return counts;
}
