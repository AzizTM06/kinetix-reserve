import { PublicKey } from "@solana/web3.js";
import { KINETIX_PROGRAM_ID, derivePoolPdas, loadSigner } from "./common";

function main() {
  const baseMint = new PublicKey(
    process.argv[2] || "So11111111111111111111111111111111111111112"
  );
  const participant = loadSigner();
  const { pool, receiptMint } = derivePoolPdas(baseMint);
  const [ledger] = PublicKey.findProgramAddressSync(
    [Buffer.from("member_ledger"), pool.toBuffer(), participant.publicKey.toBuffer()],
    KINETIX_PROGRAM_ID
  );

  console.log(`Pool PDA:          ${pool.toBase58()}`);
  console.log(`Receipt Mint PDA:  ${receiptMint.toBase58()}`);
  console.log(`MemberLedger PDA:  ${ledger.toBase58()}`);
}

main();
