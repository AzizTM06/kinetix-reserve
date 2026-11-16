import { createMint } from "@solana/spl-token";
import { createConnection, derivePoolPdas, loadSigner } from "./common";

async function main() {
  const connection = createConnection();
  const curator = loadSigner();
  console.log(`Curator: ${curator.publicKey.toBase58()}`);

  const baseMint = await createMint(connection, curator, curator.publicKey, null, 6);
  const pdas = derivePoolPdas(baseMint);

  console.log(`Base Mint:     ${baseMint.toBase58()}`);
  console.log(`ReservePool:   ${pdas.pool.toBase58()}`);
  console.log(`ReserveVault:  ${pdas.reserveVault.toBase58()}`);
  console.log(`ReceiptMint:   ${pdas.receiptMint.toBase58()}`);
}

main().catch(console.error);
