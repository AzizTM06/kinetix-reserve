function impermanentLoss(priceRatio: number): number {
  return (2 * Math.sqrt(priceRatio)) / (1 + priceRatio) - 1;
}

function main() {
  const scenarios = [1.1, 1.25, 1.5, 2.0];
  console.log("CLMM Range Position Impermanent Loss Reference:");
  for (const k of scenarios) {
    const ilPct = (impermanentLoss(k) * 100).toFixed(3);
    console.log(`  priceRatio=${k.toFixed(2)}x -> AMM v2 IL=${ilPct}%`);
  }
}

main();
