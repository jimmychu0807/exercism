export function find(haystack: number[], needle: number): number {
  let start = 0;
  let end = haystack.length -1;

  while (start <= end) { 
    
    const mid = Math.floor((start + end) / 2); // 之前用 parseInt 取整有警告，处理数字用 Math.floor ，通过这个方式获取中间值，方便将数组分两半从而快速判断目标数字所在区间

    if (haystack[mid] < needle) {
      start = mid + 1; // 中间数字比目标数字小，说明目标数字在数组右边，从中间数字后一位开始组成新数组
    }
    else if (haystack[mid] > needle) {
      end = mid - 1; // 同上，但是在左边
    }
    else {
      return mid; // 这时候中间数字 = 目标数字，中间数字的位置 mid 就是要找的
    }

  }

  throw new Error('Value not in array'); // 结束循环，按指定要求报错

}