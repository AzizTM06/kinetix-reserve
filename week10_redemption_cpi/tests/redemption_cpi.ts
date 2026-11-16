import { expect } from "chai";

describe("week10_redemption_cpi", () => {
  it("calculates net redemption output after exit fee retention", () => {
    const OFFSET = 1000n;
    const receipts = 500_000n;
    const managed = 1_000_000n;
    const issued = 1_000_000n;
    const feeBps = 50n;

    const gross = (receipts * (managed + OFFSET)) / (issued + OFFSET);
    const fee = (gross * feeBps) / 10_000n;
    const net = gross - fee;
    expect(net).to.equal(497_500n);
  });
});
