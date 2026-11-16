import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, clusterApiUrl } from "@solana/web3.js";
import * as dotenv from "dotenv";
import bs58 from "bs58";
import * as fs from "fs";
import * as path from "path";
import * as os from "os";

dotenv.config();
dotenv.config({ path: path.resolve(__dirname, "../.env") });
dotenv.config({ path: path.resolve(__dirname, "../../.env") });

export function createDevnetConnection(): Connection {
  const apiKey = process.env.HELIUS_API_KEY?.trim();
  const endpoint =
    process.env.HELIUS_RPC_URL?.trim() ||
    (apiKey ? `https://devnet.helius-rpc.com/?api-key=${apiKey}` : clusterApiUrl("devnet"));
  return new Connection(endpoint, "confirmed");
}

export function resolvePayerKeypair(): Keypair {
  const secret = process.env.SECRET_KEY?.trim();
  if (secret) {
    try {
      return Keypair.fromSecretKey(bs58.decode(secret));
    } catch {
      console.warn("Invalid SECRET_KEY format; falling back to local keypair.");
    }
  }

  const cliKeyPath = path.join(os.homedir(), ".config", "solana", "id.json");
  if (fs.existsSync(cliKeyPath)) {
    const raw = JSON.parse(fs.readFileSync(cliKeyPath, "utf-8"));
    return Keypair.fromSecretKey(new Uint8Array(raw));
  }

  const localPath = path.join(__dirname, "../wallet.json");
  if (fs.existsSync(localPath)) {
    const raw = JSON.parse(fs.readFileSync(localPath, "utf-8"));
    return Keypair.fromSecretKey(new Uint8Array(raw));
  }

  const fresh = Keypair.generate();
  fs.writeFileSync(localPath, JSON.stringify(Array.from(fresh.secretKey)));
  return fresh;
}

export async function ensureMinimumSol(
  connection: Connection,
  pubkey: PublicKey,
  minSol: number
): Promise<void> {
  const lamports = await connection.getBalance(pubkey);
  if (lamports / LAMPORTS_PER_SOL >= minSol) return;
  try {
    const sig = await connection.requestAirdrop(pubkey, LAMPORTS_PER_SOL);
    const bh = await connection.getLatestBlockhash();
    await connection.confirmTransaction({ signature: sig, ...bh });
  } catch {
    console.warn(`Airdrop rate-limited for ${pubkey.toBase58()}`);
  }
}
