import { reverse } from "./reverse-string.ts";
import { describe, it, expect } from "@jest/globals";

describe("Reverse String", () => {
  it("an empty string", () => {
    const expected = "";
    expect(reverse("")).toEqual(expected);
  });

  it("a word", () => {
    const expected = "tobor";
    expect(reverse("robot")).toEqual(expected);
  });

  it("a capitalized word", () => {
    const expected = "nemaR";
    expect(reverse("Ramen")).toEqual(expected);
  });

  it("a sentence with punctuation", () => {
    const expected = `!yrgnuh m'I`;
    expect(reverse(`I'm hungry!`)).toEqual(expected);
  });

  it("a palindrome", () => {
    const expected = "racecar";
    expect(reverse("racecar")).toEqual(expected);
  });

  it("an even-sized word", () => {
    const expected = "reward";
    expect(reverse("drawer")).toEqual(expected);
  });

  it("wide characters", () => {
    const expected = "猫子";
    expect(reverse("子猫")).toEqual(expected);
  });
});
