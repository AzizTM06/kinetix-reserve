import { Keypair, PublicKey } from "@solana/web3.js";

const PROGRAM_ID = new PublicKey("E4HxS1wXiHxZPDoeJpjWtk8jU3E7YaLwWbzQUhUZMQvg");

function main() {
  const baseMint = Keypair.generate().publicKey;
  const participant = Keypair.generate().publicKey;

  const [pool] = PublicKey.findProgramAddressSync(
    [Buffer.from("kinetix_pool"), baseMint.toBuffer()],
    PROGRAM_ID
  );
  const [reserveVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("reserve_vault"), baseMint.toBuffer()],
    PROGRAM_ID
  );
  const [receiptMint] = PublicKey.findProgramAddressSync(
    [Buffer.from("receipt_mint"), baseMint.toBuffer()],
    PROGRAM_ID
  );
  const [memberLedger] = PublicKey.findProgramAddressSync(
    [Buffer.from("member_ledger"), pool.toBuffer(), participant.toBuffer()],
    PROGRAM_ID
  );

  console.log(`BaseMint:      ${baseMint.toBase58()}`);
  console.log(`ReservePool:   ${pool.toBase58()}`);
  console.log(`ReserveVault:  ${reserveVault.toBase58()}`);
  console.log(`ReceiptMint:   ${receiptMint.toBase58()}`);
  console.log(`MemberLedger:  ${memberLedger.toBase58()}`);
}

main();
