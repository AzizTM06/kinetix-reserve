import { Connection, Keypair, clusterApiUrl } from "@solana/web3.js";
import { createAccount, createMint, mintTo } from "@solana/spl-token";
import * as dotenv from "dotenv";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";

dotenv.config({ path: path.join(__dirname, "../.env") });
dotenv.config({ path: path.join(__dirname, "../../.env") });

const RAYDIUM_CLMM_DEVNET = "devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH";

function loadKeypair(): Keypair {
  const kpPath = path.join(os.homedir(), ".config", "solana", "id.json");
  if (fs.existsSync(kpPath)) {
    return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(kpPath, "utf-8"))));
  }
  return Keypair.generate();
}

async function main() {
  const rpc = process.env.HELIUS_RPC_URL?.trim() || clusterApiUrl("devnet");
  const connection = new Connection(rpc, "confirmed");
  const payer = loadKeypair();

  const ktxA = await createMint(connection, payer, payer.publicKey, null, 6);
  const ktxB = await createMint(connection, payer, payer.publicKey, null, 6);

  const [mint0, mint1] = Buffer.compare(ktxA.toBuffer(), ktxB.toBuffer()) < 0
    ? [ktxA, ktxB]
    : [ktxB, ktxA];

  const ata0 = await createAccount(connection, payer, mint0, payer.publicKey);
  const ata1 = await createAccount(connection, payer, mint1, payer.publicKey);
  await mintTo(connection, payer, mint0, ata0, payer, 25_000 * 1_000_000);
  await mintTo(connection, payer, mint1, ata1, payer, 25_000 * 1_000_000);

  console.log(`CLMM Program (Devnet): ${RAYDIUM_CLMM_DEVNET}`);
  console.log(`Sorted mint_0:         ${mint0.toBase58()}`);
  console.log(`Sorted mint_1:         ${mint1.toBase58()}`);
  console.log("Pool Config:           tick_spacing=60, range=[-600, +600], sqrt_price_x64=2^64");
}

main().catch(console.error);
