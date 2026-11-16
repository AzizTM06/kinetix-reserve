import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import * as dotenv from "dotenv";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";

dotenv.config({ path: path.join(__dirname, "../.env") });
dotenv.config({ path: path.join(__dirname, "../../.env") });

export const KINETIX_PROGRAM_ID = new PublicKey(
  "32SMYfjH3en7X99UydFab6gWFcRY4eLPRNNsrzLicFQ1"
);
export const RAYDIUM_CLMM_DEVNET_ID = new PublicKey(
  "devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH"
);

export function createConnection(): Connection {
  const url = process.env.HELIUS_RPC_URL?.trim() || clusterApiUrl("devnet");
  return new Connection(url, "confirmed");
}

export function loadSigner(): Keypair {
  const defaultPath = path.join(os.homedir(), ".config", "solana", "id.json");
  if (fs.existsSync(defaultPath)) {
    const raw = JSON.parse(fs.readFileSync(defaultPath, "utf-8"));
    return Keypair.fromSecretKey(Uint8Array.from(raw));
  }
  return Keypair.generate();
}

export function derivePoolPdas(baseMint: PublicKey, programId = KINETIX_PROGRAM_ID) {
  const [pool] = PublicKey.findProgramAddressSync(
    [Buffer.from("kinetix_pool"), baseMint.toBuffer()],
    programId
  );
  const [reserveVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("reserve_vault"), baseMint.toBuffer()],
    programId
  );
  const [receiptMint] = PublicKey.findProgramAddressSync(
    [Buffer.from("receipt_mint"), baseMint.toBuffer()],
    programId
  );
  return { pool, reserveVault, receiptMint };
}
