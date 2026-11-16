import { PublicKey } from "@solana/web3.js";
import { derivePoolPdas, loadSigner } from "./common";

function main() {
  const baseMint = new PublicKey(
    process.argv[2] || "So11111111111111111111111111111111111111112"
  );
  const receiptsToBurn = Number(process.argv[3] || 50_000_000);
  const participant = loadSigner();
  const { pool, reserveVault, receiptMint } = derivePoolPdas(baseMint);

  console.log(`Participant:   ${participant.publicKey.toBase58()}`);
  console.log(`Redeeming:     ${receiptsToBurn} raw receipts`);
  console.log(`ReservePool:   ${pool.toBase58()}`);
  console.log(`ReserveVault:  ${reserveVault.toBase58()}`);
  console.log(`ReceiptMint:   ${receiptMint.toBase58()}`);
}

main();
