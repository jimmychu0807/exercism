export const answer = (str: string): number => {
  str = str.replace("?", "").replace("zero", "0"); // 去掉问号和替换zero

  const words = str.split(" ");
  const validWords = ["What", "is", "plus", "minus", "multiplied", "divided", "by"];
  for (const word of words) {
    if (/^-?\d+$/.test(word)) continue; // 跳过数字
    if (word === "") continue; // 跳过空字符串
    if (!validWords.includes(word)) {
      throw new Error("Unknown operation"); // 如果不包含 validWords 里的词，报错
    }
  }

  const nums = str.match(/-?\d+/g);
  if (!nums || nums.length === 0) {
    throw new Error("Syntax error"); // 检查是否包含数字
  }

  // 检查每个数字后面不能直接跟另一个数字
  for (let i = 0; i < words.length; i++) {
    if (/^-?\d+$/.test(words[i])) {
      let nextIndex = i + 1;
      while (nextIndex < words.length && (words[nextIndex] === "" || words[nextIndex] === "by")) {
        nextIndex++;
      }
      if (nextIndex < words.length) {
        const nextWord = words[nextIndex];
        if (/^-?\d+$/.test(nextWord)) {
          throw new Error("Syntax error"); // 数字后面如果还是数字就报错
        }
      }
    }
  }

  const operators: string[] = [];
  for (const word of words) {
    if (word === "plus") operators.push("+");
    else if (word === "minus") operators.push("-");
    else if (word === "multiplied") operators.push("*");
    else if (word === "divided") operators.push("/");
  }

  if (operators.length !== nums.length - 1) {
    throw new Error("Syntax error");
  }

  let result = parseInt(nums[0]);

  for (let i = 0; i < operators.length; i++) {
    const nextNum = parseInt(nums[i + 1]);

    // 按顺序进行运算
    if (operators[i] === "+") {
      result = result + nextNum;
    } else if (operators[i] === "-") {
      result = result - nextNum;
    } else if (operators[i] === "*") {
      result = result * nextNum;
    } else if (operators[i] === "/") {
      result = result / nextNum;
    }
  }

  return result;
};
