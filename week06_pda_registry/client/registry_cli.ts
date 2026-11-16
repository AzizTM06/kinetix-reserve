import { PublicKey } from "@solana/web3.js";

function main() {
  const programIdStr = process.argv[2] || "9BDNVsLJfw4ShN4cMqrJJjEDWSEWEN7byMTerjBRAnWN";
  const programId = new PublicKey(programIdStr);
  const sampleWallet = new PublicKey("11111111111111111111111111111111");

  const [passportPda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("node-passport"), sampleWallet.toBuffer()],
    programId
  );

  console.log(`Program ID: ${programId.toBase58()}`);
  console.log(`Derived NodePassport PDA: ${passportPda.toBase58()} (bump=${bump})`);
}

main();
