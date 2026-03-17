type Point = {
  x: number;
  y: number;
  angle: number;
};

type Prism = {
  id: number;
  x: number;
  y: number;
  angle: number;
};

function normalizeAngle(angle: number): number {
  angle = angle % 360; // 转一圈的角度相等
  while (angle < 0) {
    angle += 360; // 如果为负数转到正数，角度一样
  }
  return angle;
}

function isClose(a: number, b: number): boolean {
  const diff = Math.abs(a - b) % 360; // 后边用它来判断不同的横纵坐标差值，来判断是不是同样的线路
  return diff < 0.5 || diff > 359.5; // 处理 0度和360度相等的情况
}

export function findSequence(start: Point, prisms: Prism[]): number[] {
  const result: number[] = [];

  if (prisms.length === 0) {
    return []; // 空数组的情况（第一个 test ）
  }

  let candidates: Prism[] = [];

  let currentX = start.x;
  let currentY = start.y;
  let angle = normalizeAngle(start.angle);

  while (true) {
    candidates = prisms.filter((prism) => {
      if (isClose(prism.x, currentX) && isClose(prism.y, currentY)) return false; // 排除重复棱镜
      let angleToPrism = (Math.atan2(prism.y - currentY, prism.x - currentX) * 180) / Math.PI; // 弧度转角度
      //我开始的写法比较简单，但是原本的写法已经应付不了后边几个test, 才了解到这个方法
      angleToPrism = normalizeAngle(angleToPrism);
      return isClose(angleToPrism, angle);
    });

    if (candidates.length === 0) {
      break;
    }

    candidates.sort((a, b) => {
      const distA = Math.sqrt((a.x - currentX) ** 2 + (a.y - currentY) ** 2);
      const distB = Math.sqrt((b.x - currentX) ** 2 + (b.y - currentY) ** 2);
      return distA - distB;
      // 欧几里得距离公式，试错多次查到的有效排序方法
    });

    const nearest = candidates[0];
    result.push(nearest.id);

    currentX = nearest.x;
    currentY = nearest.y;
    angle = normalizeAngle(angle + nearest.angle); // 更新方向，原方向+棱镜折射角
  }

  return result;
}
