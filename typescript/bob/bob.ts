// function isAllUpperCase(text: string): boolean {
//   for (let i = 0; i < text.length; i++) {
//     const char = text[i];
//     if (char >= 'A' && char <= 'Z') {
//       return true;
//     }
//   }
//   return false;
// }



export function hey(message: string): string {
  const hasLetter = /[a-zA-Z]/.test(message); //包含字母
  const upperCase = message.toUpperCase(); 
  const lastText = message.trim().at(-1);
  if(hasLetter && (upperCase === message) && (lastText === "?")){
    return "Calm down, I know what I'm doing!"
  } //包含字母并且全大写并以问号结尾的情况。
  if(message.trim().at(-1) === "?"){
    return "Sure."
  } // 选出清空 string 里的空格后，最后一位是问号。
  if(message.trim() === ""){
    return "Fine. Be that way!"
  }  //选出只留空格的。 
  if(hasLetter && (upperCase === message)){
    return "Whoa, chill out!"
  } //如果本身=大写后的本身，说明本身全是大写。
  return "Whatever."
}
