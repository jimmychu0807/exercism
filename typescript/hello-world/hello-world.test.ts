import { hello } from "./hello-world.ts";
import { describe, it, expect } from "@jest/globals";

describe("Hello World", () => {
  it("says hello world", () => {
    expect(hello()).toEqual("Hello, World!");
  });
});
