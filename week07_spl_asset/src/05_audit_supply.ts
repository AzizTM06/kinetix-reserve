import { getAccount, getAssociatedTokenAddressSync, getMint } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import * as fs from "fs";
import {
  MINT_RECORD_FILE,
  TOKEN_DECIMALS,
  loadOrCreateOperator,
  loadRpcConnection,
} from "./config";

async function main() {
  const record = JSON.parse(fs.readFileSync(MINT_RECORD_FILE, "utf-8"));
  const mint = new PublicKey(record.mint);
  const connection = loadRpcConnection();
  const operator = loadOrCreateOperator();

  const mintInfo = await getMint(connection, mint);
  const operatorAta = getAssociatedTokenAddressSync(mint, operator.publicKey);
  const operatorAcc = await getAccount(connection, operatorAta);

  const scale = 10 ** TOKEN_DECIMALS;
  console.log(`Mint Supply:      ${Number(mintInfo.supply) / scale} KTX`);
  console.log(`Operator Balance: ${Number(operatorAcc.amount) / scale} KTX`);

  if (record.counterparty) {
    const peerAta = getAssociatedTokenAddressSync(mint, new PublicKey(record.counterparty));
    const peerAcc = await getAccount(connection, peerAta);
    console.log(`Peer Balance:     ${Number(peerAcc.amount) / scale} KTX`);
  }
}

main().catch(console.error);
