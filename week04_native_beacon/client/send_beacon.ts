import {
  Connection,
  PublicKey,
  Keypair,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
  SystemProgram,
} from "@solana/web3.js";
import * as borsh from "borsh";
import * as fs from "fs";
import * as path from "path";

const BEACON_ACCOUNT_BYTES = 8 + 2 + 4 + 48;

const PingInstructionSchema = {
  struct: {
    variant: "u8",
    status_code: "u16",
    tag: "string",
  },
};

function resolveRpcEndpoint(): string {
  if (process.env.HELIUS_RPC_URL) return process.env.HELIUS_RPC_URL;
  const candidates = [path.join(__dirname, ".env"), path.join(__dirname, "../../.env")];
  for (const file of candidates) {
    if (fs.existsSync(file)) {
      for (const row of fs.readFileSync(file, "utf-8").split("\n")) {
        const line = row.trim();
        if (line.startsWith("HELIUS_RPC_URL=")) {
          const val = line.slice("HELIUS_RPC_URL=".length).trim();
          if (val) return val;
        }
      }
    }
  }
  return clusterApiUrl("devnet");
}

async function main() {
  const rpcUrl = resolveRpcEndpoint();
  const connection = new Connection(rpcUrl, "confirmed");
  const version = await connection.getVersion();
  console.log(`Connected to Devnet node solana-core ${version["solana-core"]}`);

  const payer = Keypair.generate();
  const programIdArg = process.argv[2];
  const programId = programIdArg
    ? new PublicKey(programIdArg)
    : SystemProgram.programId;

  if (!programIdArg) {
    console.log("Pass deployed Program ID as CLI arg: npx ts-node send_beacon.ts <PROGRAM_ID>");
  }

  try {
    const sig = await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL);
    const bh = await connection.getLatestBlockhash();
    await connection.confirmTransaction({ signature: sig, ...bh });
  } catch {
    console.log("Devnet faucet rate-limited; transaction construction continues.");
  }

  const beaconAcc = Keypair.generate();
  const rentLamports = await connection.getMinimumBalanceForRentExemption(BEACON_ACCOUNT_BYTES);

  const createIx = SystemProgram.createAccount({
    fromPubkey: payer.publicKey,
    newAccountPubkey: beaconAcc.publicKey,
    lamports: rentLamports,
    space: BEACON_ACCOUNT_BYTES,
    programId,
  });

  const payload = borsh.serialize(PingInstructionSchema, {
    variant: 0,
    status_code: 200,
    tag: "kinetix-beacon-v1",
  });

  const pingIx = new TransactionInstruction({
    keys: [{ pubkey: beaconAcc.publicKey, isSigner: false, isWritable: true }],
    programId,
    data: Buffer.from(payload),
  });

  console.log(`Beacon account: ${beaconAcc.publicKey.toBase58()}`);
  console.log(`Payload bytes:  ${Buffer.from(payload).toString("hex")}`);

  if (programIdArg) {
    const tx = new Transaction().add(createIx, pingIx);
    const sig = await sendAndConfirmTransaction(connection, tx, [payer, beaconAcc]);
    console.log(`Confirmed: https://explorer.solana.com/tx/${sig}?cluster=devnet`);
  }
}

main().catch(console.error);
