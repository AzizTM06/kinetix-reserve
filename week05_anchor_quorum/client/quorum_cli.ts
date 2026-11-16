import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram, clusterApiUrl } from "@solana/web3.js";
import * as crypto from "crypto";
import * as fs from "fs";
import * as path from "path";

function loadRpc(): string {
  if (process.env.HELIUS_RPC_URL) return process.env.HELIUS_RPC_URL;
  const envFile = path.join(__dirname, "../../.env");
  if (fs.existsSync(envFile)) {
    for (const line of fs.readFileSync(envFile, "utf-8").split("\n")) {
      const t = line.trim();
      if (t.startsWith("HELIUS_RPC_URL=")) {
        const v = t.slice("HELIUS_RPC_URL=".length).trim();
        if (v) return v;
      }
    }
  }
  return clusterApiUrl("devnet");
}

function anchorDiscriminator(name: string): number[] {
  return Array.from(crypto.createHash("sha256").update(`global:${name}`).digest().subarray(0, 8));
}

function buildIdl(programId: string): anchor.Idl {
  return {
    address: programId,
    metadata: {
      name: "kinetix_quorum",
      version: "0.1.0",
      spec: "0.1.0",
    },
    instructions: [
      {
        name: "create_tally",
        discriminator: anchorDiscriminator("create_tally"),
        accounts: [
          { name: "tally", writable: true, signer: true },
          { name: "curator", writable: true, signer: true },
          { name: "system_program", address: SystemProgram.programId.toBase58() },
        ],
        args: [
          { name: "proposal_id", type: "u64" },
          { name: "quorum_target", type: "u64" },
        ],
      },
      {
        name: "cast_vote",
        discriminator: anchorDiscriminator("cast_vote"),
        accounts: [{ name: "tally", writable: true }],
        args: [
          { name: "support", type: "bool" },
          { name: "weight", type: "u64" },
        ],
      },
      {
        name: "seal_tally",
        discriminator: anchorDiscriminator("seal_tally"),
        accounts: [
          { name: "tally", writable: true },
          { name: "curator", signer: true },
        ],
        args: [],
      },
    ],
  } as anchor.Idl;
}

async function main() {
  const programIdStr = process.argv[2] || "5EWB2t83QvN9MMDB6fgCLatRZQBxHW1LSi1aYzSBpEu4";
  const programId = new PublicKey(programIdStr);
  const connection = new Connection(loadRpc(), "confirmed");
  const curator = Keypair.generate();
  const wallet = new anchor.Wallet(curator);
  const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  const program = new anchor.Program(buildIdl(programId.toBase58()), provider);

  const tally = Keypair.generate();
  console.log(`Program ID: ${programId.toBase58()}`);
  console.log(`Tally account: ${tally.publicKey.toBase58()}`);

  const ix = await program.methods
    .createTally(new anchor.BN(7), new anchor.BN(100))
    .accounts({
      tally: tally.publicKey,
      curator: curator.publicKey,
    })
    .instruction();

  console.log(`Built createTally instruction (${ix.data.length} bytes).`);
}

main().catch(console.error);
