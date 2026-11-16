import {
  burn,
  createMint,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
} from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";
import { createDevnetConnection, ensureMinimumSol, resolvePayerKeypair } from "./rpc";

const DECIMALS = 8;
const UNIT = 10n ** BigInt(DECIMALS);

function loadPeerRecipients(): PublicKey[] {
  const configured = [process.env.RECIPIENT_1, process.env.RECIPIENT_2, process.env.RECIPIENT_3]
    .map((x) => x?.trim())
    .filter((x): x is string => Boolean(x && x.length > 30))
    .map((x) => new PublicKey(x));

  if (configured.length === 3) return configured;
  return [Keypair.generate().publicKey, Keypair.generate().publicKey, Keypair.generate().publicKey];
}

async function main() {
  const connection = createDevnetConnection();
  const payer = resolvePayerKeypair();
  await ensureMinimumSol(connection, payer.publicKey, 0.05);

  const mint = await createMint(connection, payer, payer.publicKey, payer.publicKey, DECIMALS);
  const payerAta = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mint,
    payer.publicKey
  );

  await mintTo(connection, payer, mint, payerAta.address, payer, 50_000n * UNIT);

  const peers = loadPeerRecipients();
  for (const peer of peers) {
    const peerAta = await getOrCreateAssociatedTokenAccount(connection, payer, mint, peer);
    await transfer(connection, payer, payerAta.address, peerAta.address, payer, 250n * UNIT);
  }

  const burnSig = await burn(connection, payer, payerAta.address, mint, payer, 125n * UNIT);
  const finalAcc = await getAccount(connection, payerAta.address);

  console.log(`KTXLAB Mint:      https://explorer.solana.com/address/${mint.toBase58()}?cluster=devnet`);
  console.log(`Burn Tx:          https://explorer.solana.com/tx/${burnSig}?cluster=devnet`);
  console.log(`Final Balance:    ${Number(finalAcc.amount) / 10 ** DECIMALS} KTXLAB (expected 49125)`);
}

main().catch(console.error);
