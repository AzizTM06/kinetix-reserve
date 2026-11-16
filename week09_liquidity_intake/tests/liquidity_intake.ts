import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";

describe("week09_liquidity_intake", () => {
  it("computes virtual-offset proportional receipts accurately", () => {
    const OFFSET = 1000n;
    const managed = 1_200_000n;
    const issued = 1_000_000n;
    const amount = 600_000n;
    const receipts = (amount * (issued + OFFSET)) / (managed + OFFSET);
    expect(receipts > 499_000n && receipts <= 500_000n).to.equal(true);
  });
});
