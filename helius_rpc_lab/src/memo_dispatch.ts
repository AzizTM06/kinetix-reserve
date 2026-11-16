import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { createDevnetConnection, ensureMinimumSol, resolvePayerKeypair } from "./rpc";

const SPL_MEMO_PROGRAM_ID = new PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

async function main() {
  const connection = createDevnetConnection();
  const sender = resolvePayerKeypair();
  const receiver = Keypair.generate();

  await ensureMinimumSol(connection, sender.publicKey, 0.02);

  const memoTag = "KTX_SETTLEMENT::epoch_42::batch_01";
  const tx = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: sender.publicKey,
      toPubkey: receiver.publicKey,
      lamports: Math.round(0.015 * LAMPORTS_PER_SOL),
    }),
    new TransactionInstruction({
      keys: [{ pubkey: sender.publicKey, isSigner: true, isWritable: false }],
      programId: SPL_MEMO_PROGRAM_ID,
      data: Buffer.from(memoTag, "utf-8"),
    })
  );

  const sig = await sendAndConfirmTransaction(connection, tx, [sender]);
  const info = await connection.getTransaction(sig, {
    commitment: "confirmed",
    maxSupportedTransactionVersion: 0,
  });

  console.log(`Sender:    ${sender.publicKey.toBase58()}`);
  console.log(`Receiver:  ${receiver.publicKey.toBase58()}`);
  console.log(`Slot:      ${info?.slot ?? "n/a"}`);
  console.log(`Explorer:  https://explorer.solana.com/tx/${sig}?cluster=devnet`);
}

main().catch(console.error);
