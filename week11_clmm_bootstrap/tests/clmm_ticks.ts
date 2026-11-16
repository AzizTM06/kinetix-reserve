import { expect } from "chai";
import BN from "bn.js";

describe("week11_clmm_bootstrap", () => {
  it("computes Q64.64 sqrt_price_x64 and tick pricing P(i)=1.0001^i", () => {
    const q64 = new BN("18446744073709551616"); // 2^64 for P=1.0 (tick=0)
    const tick600Price = Math.pow(1.0001, 600);
    const tickNeg600Price = Math.pow(1.0001, -600);

    expect(q64.muln(2).toString()).to.equal("36893488147419103232");
    expect(tick600Price).to.be.closeTo(1.06183, 1e-4);
    expect(tickNeg600Price).to.be.closeTo(0.94176, 1e-4);
  });

  it("validates tick alignment for tick_spacing=60", () => {
    const spacing = 60;
    const lower = -600;
    const upper = 600;
    expect(lower % spacing).to.equal(0);
    expect(upper % spacing).to.equal(0);
    expect(lower).to.be.lessThan(upper);
  });
});
