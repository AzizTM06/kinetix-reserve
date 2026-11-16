import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("node_registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NodeRegistry as Program<any>;
  const owner = provider.wallet.publicKey;

  const [passportPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("node-passport"), owner.toBuffer()],
    program.programId
  );

  it("issues a deterministic PDA passport", async () => {
    await program.methods
      .issuePassport("ktx-alpha-01", "eu-central", 2)
      .accounts({
        passport: passportPda,
        owner,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const record = await program.account.nodePassport.fetch(passportPda);
    expect(record.callsign).to.equal("ktx-alpha-01");
    expect(record.region).to.equal("eu-central");
    expect(record.roleTier).to.equal(2);
    expect(record.reputation).to.equal(100);
  });

  it("upgrades passport region and reputation", async () => {
    await program.methods
      .upgradePassport("ap-southeast", 3, 50)
      .accounts({
        passport: passportPda,
        owner,
      })
      .rpc();

    const updated = await program.account.nodePassport.fetch(passportPda);
    expect(updated.region).to.equal("ap-southeast");
    expect(updated.roleTier).to.equal(3);
    expect(updated.reputation).to.equal(150);
  });

  it("revokes passport and reclaims rent", async () => {
    const balanceBefore = await provider.connection.getBalance(owner);
    await program.methods
      .revokePassport()
      .accounts({
        passport: passportPda,
        owner,
      })
      .rpc();

    const closed = await provider.connection.getAccountInfo(passportPda);
    expect(closed).to.be.null;
    const balanceAfter = await provider.connection.getBalance(owner);
    expect(balanceAfter).to.be.greaterThan(balanceBefore - 50000);
  });
});
