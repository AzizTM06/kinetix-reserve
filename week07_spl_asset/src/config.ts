import { Connection, Keypair, LAMPORTS_PER_SOL, clusterApiUrl } from "@solana/web3.js";
import * as fs from "fs";
import * as path from "path";

export const TOKEN_DECIMALS = 6;
export const INITIAL_SUPPLY = 250_000;
export const TRANSFER_AMOUNT = 4_250;

const WALLET_FILE = path.join(__dirname, "../wallet.json");
export const MINT_RECORD_FILE = path.join(__dirname, "../mint_record.json");

export function loadRpcConnection(): Connection {
  let endpoint = process.env.HELIUS_RPC_URL || process.env.SOLANA_RPC_URL;
  if (!endpoint) {
    for (const candidate of [path.join(__dirname, "../.env"), path.join(__dirname, "../../.env")]) {
      if (!fs.existsSync(candidate)) continue;
      for (const line of fs.readFileSync(candidate, "utf-8").split("\n")) {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith("#")) continue;
        const idx = trimmed.indexOf("=");
        if (idx === -1) continue;
        const k = trimmed.slice(0, idx).trim();
        const v = trimmed.slice(idx + 1).trim();
        if ((k === "HELIUS_RPC_URL" || k === "SOLANA_RPC_URL") && v.startsWith("http")) {
          endpoint = v;
          break;
        }
      }
      if (endpoint) break;
    }
  }
  return new Connection(endpoint || clusterApiUrl("devnet"), "confirmed");
}

export function loadOrCreateOperator(): Keypair {
  if (fs.existsSync(WALLET_FILE)) {
    const raw = JSON.parse(fs.readFileSync(WALLET_FILE, "utf-8"));
    return Keypair.fromSecretKey(new Uint8Array(raw));
  }
  const kp = Keypair.generate();
  fs.writeFileSync(WALLET_FILE, JSON.stringify(Array.from(kp.secretKey)));
  return kp;
}

export async function ensureSolBalance(connection: Connection, signer: Keypair, minSol = 0.25) {
  const bal = await connection.getBalance(signer.publicKey);
  if (bal < minSol * LAMPORTS_PER_SOL) {
    try {
      const sig = await connection.requestAirdrop(signer.publicKey, LAMPORTS_PER_SOL);
      const bh = await connection.getLatestBlockhash();
      await connection.confirmTransaction({ signature: sig, ...bh });
    } catch {
      console.warn(`Devnet faucet rate-limited for ${signer.publicKey.toBase58()}`);
    }
  }
}

export function explorerLink(id: string, kind: "tx" | "address"): string {
  return `https://explorer.solana.com/${kind}/${id}?cluster=devnet`;
}
