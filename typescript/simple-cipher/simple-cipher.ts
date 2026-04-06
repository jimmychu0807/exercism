const chars = "abcdefghijklmnopqrstuvwxyz";

export class SimpleCipher {
  key: string;
  constructor(key: string | undefined = undefined) {
    if (key) {
      this.key = key;
    } else {
      this.key = randomStr(100);
    }
  }
  encode(text: string): string {
    let result = "";
    for (let i = 0; i < text.length; i++) {
      const textPos = chars.indexOf(text[i]); // 假设 aaa
      const keyPos = chars.indexOf(this.key[i % this.key.length]); // 随机，假设 xyz，如果位数不够时循环使用
      const encoded = (textPos + keyPos) % 26; // Vigenère cipher, 余数是超出了整组循环后的位置
      result += chars[encoded];
    }
    return result;
  }

  decode(text: string): string {
    let result = "";
    for (let i = 0; i < text.length; i++) {
      const textPos = chars.indexOf(text[i]);
      const keyPos = chars.indexOf(this.key[i % this.key.length]);
      const decoded = (textPos - keyPos + 26) % 26; // 如果不 +26，有负数可能，is reversible 报错
      result += chars[decoded];
    }
    return result;
  }
}

function randomStr(length: number): string {
  let str = "";
  for (let i = 0; i < length; i++) {
    const index = getRandomInt(chars.length);
    str += chars[index]; // 从 chars 里取该位置的字母
  }
  return str;
}

function getRandomInt(max: number): number {
  return Math.floor(Math.random() * max);
}
