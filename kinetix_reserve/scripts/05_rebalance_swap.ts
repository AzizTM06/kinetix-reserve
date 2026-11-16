import { RAYDIUM_CLMM_DEVNET_ID } from "./common";

function main() {
  const amountIn = 5_000_000;
  const slippageBps = 50; // 0.50%
  const minOut = Math.floor((amountIn * (10_000 - slippageBps)) / 10_000);
  const sqrtPriceLimitX64 = "18262276632972456100"; // ~0.9801 lower price guard

  console.log(`CLMM Program:        ${RAYDIUM_CLMM_DEVNET_ID.toBase58()}`);
  console.log(`Swap amountIn:       ${amountIn}`);
  console.log(`Min amountOut:       ${minOut} (slippage=${slippageBps} bps)`);
  console.log(`sqrtPriceLimitX64:   ${sqrtPriceLimitX64}`);
}

main();
