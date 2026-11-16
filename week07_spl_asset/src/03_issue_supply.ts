import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import * as fs from "fs";
import {
  INITIAL_SUPPLY,
  MINT_RECORD_FILE,
  TOKEN_DECIMALS,
  explorerLink,
  loadOrCreateOperator,
  loadRpcConnection,
} from "./config";

async function main() {
  const { mint: mintStr } = JSON.parse(fs.readFileSync(MINT_RECORD_FILE, "utf-8"));
  const mint = new PublicKey(mintStr);
  const connection = loadRpcConnection();
  const operator = loadOrCreateOperator();

  const ata = await getOrCreateAssociatedTokenAccount(
    connection,
    operator,
    mint,
    operator.publicKey
  );

  const rawAmount = BigInt(INITIAL_SUPPLY) * BigInt(10 ** TOKEN_DECIMALS);
  const sig = await mintTo(connection, operator, mint, ata.address, operator, rawAmount);

  console.log(`Minted ${INITIAL_SUPPLY} KTX to ATA ${ata.address.toBase58()}`);
  console.log(`Tx: ${explorerLink(sig, "tx")}`);
}

main().catch(console.error);
