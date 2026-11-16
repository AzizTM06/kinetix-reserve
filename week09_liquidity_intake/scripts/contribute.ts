import { Keypair, PublicKey } from "@solana/web3.js";

const PROGRAM_ID = new PublicKey("U1Z9MA1VEJJmwLcR5LkrXpVwH9KKpKB4oAtKawCTpMW");

function main() {
  const baseMint = Keypair.generate().publicKey;
  const participant = Keypair.generate().publicKey;
  const [pool] = PublicKey.findProgramAddressSync(
    [Buffer.from("kinetix_pool"), baseMint.toBuffer()],
    PROGRAM_ID
  );
  const [ledger] = PublicKey.findProgramAddressSync(
    [Buffer.from("member_ledger"), pool.toBuffer(), participant.toBuffer()],
    PROGRAM_ID
  );
  console.log(`Contribute target pool: ${pool.toBase58()}, ledger: ${ledger.toBase58()}`);
}

main();
