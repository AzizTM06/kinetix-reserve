import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";
import * as fs from "fs";
import {
  MINT_RECORD_FILE,
  TOKEN_DECIMALS,
  TRANSFER_AMOUNT,
  explorerLink,
  loadOrCreateOperator,
  loadRpcConnection,
} from "./config";

async function main() {
  const record = JSON.parse(fs.readFileSync(MINT_RECORD_FILE, "utf-8"));
  const mint = new PublicKey(record.mint);
  const connection = loadRpcConnection();
  const operator = loadOrCreateOperator();
  const counterparty = Keypair.generate();

  const sourceAta = await getOrCreateAssociatedTokenAccount(
    connection,
    operator,
    mint,
    operator.publicKey
  );
  const targetAta = await getOrCreateAssociatedTokenAccount(
    connection,
    operator,
    mint,
    counterparty.publicKey
  );

  const rawTransfer = BigInt(TRANSFER_AMOUNT) * BigInt(10 ** TOKEN_DECIMALS);
  const sig = await transfer(
    connection,
    operator,
    sourceAta.address,
    targetAta.address,
    operator,
    rawTransfer
  );

  record.counterparty = counterparty.publicKey.toBase58();
  fs.writeFileSync(MINT_RECORD_FILE, JSON.stringify(record, null, 2));

  console.log(`Transferred ${TRANSFER_AMOUNT} KTX to ${counterparty.publicKey.toBase58()}`);
  console.log(`Tx: ${explorerLink(sig, "tx")}`);
}

main().catch(console.error);
