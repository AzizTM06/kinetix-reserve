import { Keypair } from "@solana/web3.js";
import { RAYDIUM_CLMM_DEVNET_ID } from "./common";

function main() {
  const mintA = Keypair.generate().publicKey;
  const mintB = Keypair.generate().publicKey;
  const [mint0, mint1] = Buffer.compare(mintA.toBuffer(), mintB.toBuffer()) < 0
    ? [mintA, mintB]
    : [mintB, mintA];

  const tickSpacing = 60;
  const tickLower = -600;
  const tickUpper = 600;
  const sqrtPriceX64 = 18446744073709551616n; // 2^64 for P = 1.0

  console.log(`Raydium CLMM Devnet: ${RAYDIUM_CLMM_DEVNET_ID.toBase58()}`);
  console.log(`Canonical token_0:   ${mint0.toBase58()}`);
  console.log(`Canonical token_1:   ${mint1.toBase58()}`);
  console.log(`Tick range:          [${tickLower}, ${tickUpper}] (spacing=${tickSpacing})`);
  console.log(`Initial sqrtPriceX64:${sqrtPriceX64.toString()}`);
}

main();
