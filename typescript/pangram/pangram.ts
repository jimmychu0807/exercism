export function isPangram(text: string): boolean {
  if(text === ''){
    return false;
  }
  const alphabet = 'abcdefghijklmnopqrstuvwxyz'; // 根据意思需要让输入的句子包含所有字母，所以先定一个字母表。
  const testText = text.toLowerCase(); // 统一用小写来进行比对

  for (let i = 0; i < alphabet.length; i++) { 
    const char = alphabet[i]; // 从字母表里抽取每一个字母
    if (!testText.includes(char)) { //看看该字母是否被测试文本包含
      return false;
    }
  }
  return true;
}
