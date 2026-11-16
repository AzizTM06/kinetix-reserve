import {
  createMint,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
} from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import {
  INITIAL_SUPPLY,
  TOKEN_DECIMALS,
  TRANSFER_AMOUNT,
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
  const operatorAta = await getOrCreateAssociatedTokenAccount(
    connection,
    operator,
    mint,
    operator.publicKey
  );

  const scale = BigInt(10 ** TOKEN_DECIMALS);
  await mintTo(
    connection,
    operator,
    mint,
    operatorAta.address,
    operator,
    BigInt(INITIAL_SUPPLY) * scale
  );

  const peer = Keypair.generate();
  const peerAta = await getOrCreateAssociatedTokenAccount(
    connection,
    operator,
    mint,
    peer.publicKey
  );
  const txSig = await transfer(
    connection,
    operator,
    operatorAta.address,
    peerAta.address,
    operator,
    BigInt(TRANSFER_AMOUNT) * scale
  );

  const finalOp = await getAccount(connection, operatorAta.address);
  console.log(`KTX Mint: ${explorerLink(mint.toBase58(), "address")}`);
  console.log(`Transfer: ${explorerLink(txSig, "tx")}`);
  console.log(`Remaining operator balance: ${Number(finalOp.amount) / 10 ** TOKEN_DECIMALS} KTX`);
}

main().catch(console.error);
