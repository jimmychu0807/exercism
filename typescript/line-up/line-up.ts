export function format(name: string, number: number): string {
  const lastDigit = number % 10; // 最后一位数
  const lastTwoDigits = number % 100; // 最后两位数
  let suffix: string; // 后缀
  switch (true) {
    case lastDigit === 1 && lastTwoDigits !== 11:
      suffix = "st";
      break;
    case lastDigit === 2 && lastTwoDigits !== 12:
      suffix = "nd";
      break;
    case lastDigit === 3 && lastTwoDigits !== 13:
      suffix = "rd";
      break;
    default:
      suffix = "th";
  }
  return `${name}, you are the ${number}${suffix} customer we serve today. Thank you!`;
}
