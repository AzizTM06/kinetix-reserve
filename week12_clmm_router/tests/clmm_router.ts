import { expect } from "chai";

describe("week12_clmm_router", () => {
  it("verifies slippage threshold computation", () => {
    const amountIn = 5_000_000;
    const maxSlippageBps = 50;
    const minOut = Math.floor((amountIn * (10_000 - maxSlippageBps)) / 10_000);
    expect(minOut).to.equal(4_975_000);
  });
});
