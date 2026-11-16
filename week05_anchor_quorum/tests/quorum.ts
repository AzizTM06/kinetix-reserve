import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("kinetix_quorum", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.KinetixQuorum as Program<any>;
  const tallyAccount = Keypair.generate();

  it("initializes a proposal tally", async () => {
    await program.methods
      .createTally(new anchor.BN(1), new anchor.BN(50))
      .accounts({
        tally: tallyAccount.publicKey,
        curator: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([tallyAccount])
      .rpc();

    const state = await program.account.proposalTally.fetch(tallyAccount.publicKey);
    expect(state.proposalId.toNumber()).to.equal(1);
    expect(state.quorumTarget.toNumber()).to.equal(50);
    expect(state.isSealed).to.equal(false);
  });

  it("casts and revokes votes with underflow guard", async () => {
    await program.methods
      .castVote(true, new anchor.BN(30))
      .accounts({ tally: tallyAccount.publicKey })
      .rpc();

    await program.methods
      .revokeVote(true, new anchor.BN(10))
      .accounts({ tally: tallyAccount.publicKey })
      .rpc();

    const state = await program.account.proposalTally.fetch(tallyAccount.publicKey);
    expect(state.yesVotes.toNumber()).to.equal(20);

    let underflowed = false;
    try {
      await program.methods
        .revokeVote(true, new anchor.BN(999))
        .accounts({ tally: tallyAccount.publicKey })
        .rpc();
    } catch {
      underflowed = true;
    }
    expect(underflowed).to.equal(true);
  });

  it("rejects sealing from non-curator and allows curator", async () => {
    const intruder = Keypair.generate();
    let denied = false;
    try {
      await program.methods
        .sealTally()
        .accounts({ tally: tallyAccount.publicKey, curator: intruder.publicKey })
        .signers([intruder])
        .rpc();
    } catch {
      denied = true;
    }
    expect(denied).to.equal(true);

    await program.methods
      .sealTally()
      .accounts({ tally: tallyAccount.publicKey, curator: provider.wallet.publicKey })
      .rpc();

    const sealed = await program.account.proposalTally.fetch(tallyAccount.publicKey);
    expect(sealed.isSealed).to.equal(true);
  });
});
