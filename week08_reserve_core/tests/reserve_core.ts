import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { createMint } from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("week08_reserve_core", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Week08ReserveCore as Program<any>;

  it("initializes ReservePool and MemberLedger PDAs", async () => {
    const curator = (provider.wallet as anchor.Wallet).payer;
    const baseMint = await createMint(provider.connection, curator, curator.publicKey, null, 6);

    const [pool] = PublicKey.findProgramAddressSync(
      [Buffer.from("kinetix_pool"), baseMint.toBuffer()],
      program.programId
    );
    const [reserveAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("reserve_vault"), baseMint.toBuffer()],
      program.programId
    );
    const [receiptMint] = PublicKey.findProgramAddressSync(
      [Buffer.from("receipt_mint"), baseMint.toBuffer()],
      program.programId
    );

    await program.methods
      .bootstrapPool(25)
      .accounts({ pool, reserveAccount, receiptMint, baseMint, curator: curator.publicKey } as any)
      .rpc();

    const poolAcc = await program.account.reservePool.fetch(pool);
    assert.equal(poolAcc.feeBps, 25);

    const [ledger] = PublicKey.findProgramAddressSync(
      [Buffer.from("member_ledger"), pool.toBuffer(), curator.publicKey.toBuffer()],
      program.programId
    );
    await program.methods
      .registerMember()
      .accounts({ pool, ledger, participant: curator.publicKey } as any)
      .rpc();

    const ledgerAcc = await program.account.memberLedger.fetch(ledger);
    assert.equal(ledgerAcc.receiptBalance.toNumber(), 0);
  });
});
