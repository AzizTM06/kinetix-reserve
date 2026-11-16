import { PublicKey } from "@solana/web3.js";

const ROUTER_PROGRAM = new PublicKey("FTLgYnW8feoh9CsZz1s7CvBs7bL3y1Go5GXRNNZxHcsx");
const RAYDIUM_CLMM_ID = new PublicKey("devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH");

function main() {
  const amountIn = 2_500_000;
  const minOut = 2_475_000; // 1% slippage cap
  console.log(`Router: ${ROUTER_PROGRAM.toBase58()} -> CLMM: ${RAYDIUM_CLMM_ID.toBase58()}`);
  console.log(`Params: amount_in=${amountIn}, min_amount_out=${minOut}`);
}

main();
