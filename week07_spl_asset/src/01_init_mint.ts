import { createMint } from "@solana/spl-token";
import * as fs from "fs";
import {
  MINT_RECORD_FILE,
  TOKEN_DECIMALS,
  ensureSolBalance,
  explorerLink,
  loadOrCreateOperator,
  loadRpcConnection,
} from "./config";

async function main() {
  const connection = loadRpcConnection();
  const operator = loadOrCreateOperator();
  await ensureSolBalance(connection, operator);

  const mint = await createMint(
    connection,
    operator,
    operator.publicKey,
    operator.publicKey,
    TOKEN_DECIMALS
  );

  fs.writeFileSync(
    MINT_RECORD_FILE,
    JSON.stringify({ mint: mint.toBase58(), decimals: TOKEN_DECIMALS }, null, 2)
  );
  console.log(`Initialized KTX Mint: ${mint.toBase58()}`);
  console.log(`Explorer: ${explorerLink(mint.toBase58(), "address")}`);
}

main().catch(console.error);
