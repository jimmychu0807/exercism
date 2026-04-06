export class SplitSecondStopwatch {
  _state: "ready" | "running" | "stopped"; // 这题的关键在于不同操作下 state 的状态变化以及对不同 function 的返回影响
  _previousLaps: string[];
  _currentLapMs: number; // 让时间先变为可加进行加减运算的格式，输出时再另外格式化
  _totalMs: number;
  constructor() {
    this._state = "ready";
    this._previousLaps = [];
    this._currentLapMs = 0;
    this._totalMs = 0;
  }

  public get state(): string {
    return this._state;
  }

  public get currentLap(): string {
    const total = this._currentLapMs;
    return timeFormat(total);
  }

  public get total(): string {
    return timeFormat(this._totalMs);
  }

  public get previousLaps(): string[] {
    return this._previousLaps;
  }

  public start(): string {
    if (this._state === "running") {
      throw new Error("cannot start an already running stopwatch");
    }
    return (this._state = "running");
  }

  public stop(): string {
    if (this._state !== "running") {
      throw new Error("cannot stop a stopwatch that is not running");
    }
    this._state = "stopped";
    return this._state;
  }

  public lap(): void {
    if (this._state !== "running") {
      throw new Error("cannot lap a stopwatch that is not running");
    }
    this._previousLaps.push(timeFormat(this._currentLapMs));
    this._currentLapMs = 0;
  }

  public reset(): string {
    if (this._state !== "stopped") {
      throw new Error("cannot reset a stopwatch that is not stopped");
    }
    this._state = "ready";
    this._currentLapMs = 0;
    this._previousLaps = [];
    return this._state;
  }

  public advanceTime(duration: string): void {
    if (this._state === "running") {
      const [h, m, s] = duration.split(":");
      const timeMs = Number(h) * 3600000 + Number(m) * 60000 + Number(s) * 1000;
      this._currentLapMs += timeMs;
      this._totalMs += timeMs;
    }
  }
}

function timeFormat(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor(totalSeconds / 60) % 60;
  const seconds = totalSeconds % 60;

  const hh = String(hours).padStart(2, "0");
  const mm = String(minutes).padStart(2, "0");
  const ss = String(seconds).padStart(2, "0");

  return `${hh}:${mm}:${ss}`; // 参考 clock 那题
}
