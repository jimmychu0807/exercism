type GameState = {
  status: "finished" | "loop";
  cards: number;
  tricks: number;
};

function toState(cards: string[]): string {
  const result: string[] = [];
  for (let i = 0; i < cards.length; i++) {
    if ("AKQJ".includes(cards[i])) {
      result.push(cards[i]);
    } else {
      result.push("#");
    }
  }
  return result.join(",");
}

export const simulateGame = (playerA: string[], playerB: string[]): GameState => {
  const deskCards: string[] = []; // 牌桌上的牌
  let totalCards = 0;
  let totalTricks = 0;
  let currentPlayer = playerA; // 轮到出牌者，从 A 开始
  let penaltyNum = 0; // 要交的罚牌
  let lastPaymentCardPlayer: string[] | null = null; // 谁出最后点数牌
  let currentStatus: "finished" | "loop" = "finished"; // 默认正常结束游戏状态
  const pastStates = new Set<string>(); // 用来记录过去状态，用于后边对比后判断 status；改为 Set 尝试提高性能

  const isLoop = (): boolean => {
    const currentState = `${toState(playerA)}|${toState(playerB)}|${toState(deskCards)}|${penaltyNum}|${currentPlayer === playerA ? "A" : "B"}`;
    if (pastStates.has(currentState)) {
      return true;
    }
    pastStates.add(currentState);
    return false;
  };

  while (true) {
    if (currentPlayer.length === 0) {
      break;
    }

    if (isLoop()) {
      currentStatus = "loop";
      break;
    }

    const card = currentPlayer.shift() as string; // 当前玩家取出一张牌
    deskCards.push(card);
    totalCards++;

    let currentPenalty = 0;

    if (card === "A") {
      currentPenalty = 4;
    } else if (card === "K") {
      currentPenalty = 3;
    } else if (card === "Q") {
      currentPenalty = 2;
    } else if (card === "J") {
      currentPenalty = 1;
    } // 看当前的牌，判断对应的罚多少张

    if (currentPenalty > 0) {
      // 这种情况，说正在 penalty 的时候抽到点数牌，局势逆转
      penaltyNum = currentPenalty; // 这是接下来出牌的人（对方）要出的拍的数量，即当下我出到点数牌的数量
      lastPaymentCardPlayer = currentPlayer; // 标记暂时的赢家

      if (currentPlayer === playerA) {
        currentPlayer = playerB;
      } else {
        currentPlayer = playerA;
      }
    } else if (penaltyNum > 0) {
      // 正在罚恶普通牌
      penaltyNum--; // 每次出牌，罚牌数少1
      if (penaltyNum === 0) {
        //这个时候交完罚牌，游戏结束

        if (lastPaymentCardPlayer) {
          for (let i = 0; i < deskCards.length; i++) {
            lastPaymentCardPlayer.push(deskCards[i]); // 根据规则，桌上的牌最后归发点数牌的玩家
          }
          currentPlayer = lastPaymentCardPlayer; // 这一局的胜者下一局先手
        }

        deskCards.length = 0; // 清空牌桌，试过没这步运行很久，因为不做这一步牌桌上的牌会越来越多，遍历次数也会越来越多。
        totalTricks++;
        lastPaymentCardPlayer = null;

        // 结算后检查对方是否手牌为空，若空则游戏结束
        const opponent = currentPlayer === playerA ? playerB : playerA;
        if (opponent.length === 0) {
          break;
        }
      }
      // penaltyNum > 0 时：不换手，继续还债
    } else {
      // 都出普通牌，无 penalty：换手
      if (currentPlayer === playerA) {
        currentPlayer = playerB;
      } else {
        currentPlayer = playerA;
      }
    }
  }

  if (currentStatus === "finished" && deskCards.length > 0) {
    totalTricks++; // 累计回合数
  }

  return {
    status: currentStatus,
    cards: totalCards,
    tricks: totalTricks,
  };
};
