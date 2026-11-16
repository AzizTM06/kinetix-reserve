import { expect } from "chai";

describe("week13_clmm_positions", () => {
  it("computes Impermanent Loss across price shifts", () => {
    const ilAt2x = (2 * Math.sqrt(2)) / (1 + 2) - 1;
    expect(ilAt2x).to.be.closeTo(-0.05719, 1e-4);
  });
});
