export function commands(num : number): string[] {
  const binary = num.toString(2).padStart(5, '0'); // 把10进制数字改为2进制并补齐5位数以满足原题目的逻辑
  const actions: string[] = [];

  if (binary[binary.length - 1] === '1') actions.push('wink');               
  if (binary[binary.length - 2] === '1') actions.push('double blink');       
  if (binary[binary.length - 3] === '1') actions.push('close your eyes');    
  if (binary[binary.length - 4] === '1') actions.push('jump');               
  if (binary[binary.length - 5] === '1') actions.reverse();                  
  
  return actions; // 以上是原题目的逻辑，根据不同位数为1的数不同，把指定的字符组合加入到数组里。

}
