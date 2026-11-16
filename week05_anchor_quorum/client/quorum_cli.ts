import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram, clusterApiUrl } from "@solana/web3.js";
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

const QUORUM_IDL = {
  version: "0.1.0",
  name: "kinetix_quorum",
  instructions: [
    {
      name: "createTally",
      accounts: [
        { name: "tally", isMut: true, isSigner: true },
        { name: "curator", isMut: true, isSigner: true },
        { name: "systemProgram", isMut: false, isSigner: false },
      ],
      args: [
        { name: "proposalId", type: "u64" },
        { name: "quorumTarget", type: "u64" },
      ],
    },
    {
      name: "castVote",
      accounts: [{ name: "tally", isMut: true, isSigner: false }],
      args: [
        { name: "support", type: "bool" },
        { name: "weight", type: "u64" },
      ],
    },
    {
      name: "sealTally",
      accounts: [
        { name: "tally", isMut: true, isSigner: false },
        { name: "curator", isMut: false, isSigner: true },
      ],
      args: [],
    },
  ],
  accounts: [
    {
      name: "ProposalTally",
      type: {
        kind: "struct",
        fields: [
          { name: "curator", type: "publicKey" },
          { name: "proposalId", type: "u64" },
          { name: "yesVotes", type: "u64" },
          { name: "noVotes", type: "u64" },
          { name: "quorumTarget", type: "u64" },
          { name: "isSealed", type: "bool" },
        ],
      },
    },
  ],
};

async function main() {
  const programIdStr = process.argv[2] || "5EWB2t83QvN9MMDB6fgCLatRZQBxHW1LSi1aYzSBpEu4";
  const programId = new PublicKey(programIdStr);
  const connection = new Connection(loadRpc(), "confirmed");
  const curator = Keypair.generate();
  const wallet = new anchor.Wallet(curator);
  const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  const program = new anchor.Program(QUORUM_IDL as any, programId, provider);

  const tally = Keypair.generate();
  console.log(`Program ID: ${programId.toBase58()}`);
  console.log(`Tally account: ${tally.publicKey.toBase58()}`);

  const ix = await program.methods
    .createTally(new anchor.BN(7), new anchor.BN(100))
    .accounts({
      tally: tally.publicKey,
      curator: curator.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .instruction();

  console.log(`Built createTally instruction (${ix.data.length} bytes).`);
}

main().catch(console.error);
