import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createAccount,
  createMint,
  getAccount,
  mintTo,
} from "@solana/spl-token";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { assert } from "chai";

async function airdropFromCurator(
  provider: anchor.AnchorProvider,
  curator: Keypair,
  target: PublicKey,
  lamports = 0.05 * LAMPORTS_PER_SOL
) {
  const tx = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: curator.publicKey,
      toPubkey: target,
      lamports,
    })
  );
  await provider.sendAndConfirm(tx, [curator]);
}

describe("kinetix_reserve", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = (anchor.workspace.KinetixReserve ||
    anchor.workspace.kinetix_reserve) as Program<any>;
  const curator = (provider.wallet as anchor.Wallet).payer;

  const participantA = Keypair.generate();
  const participantB = Keypair.generate();
  const unregisteredUser = Keypair.generate();

  let baseMint: PublicKey;
  let poolPda: PublicKey;
  let reserveVaultPda: PublicKey;
  let receiptMintPda: PublicKey;

  const baseAccounts: Record<string, PublicKey> = {};
  const receiptAccounts: Record<string, PublicKey> = {};
  const DECIMALS_UNIT = 1_000_000;

  const ledgerPda = (user: PublicKey) =>
    PublicKey.findProgramAddressSync(
      [Buffer.from("member_ledger"), poolPda.toBuffer(), user.toBuffer()],
      program.programId
    )[0];

  before(async () => {
    for (const kp of [participantA, participantB, unregisteredUser]) {
      await airdropFromCurator(provider, curator, kp.publicKey);
    }

    baseMint = await createMint(provider.connection, curator, curator.publicKey, null, 6);

    [poolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("kinetix_pool"), baseMint.toBuffer()],
      program.programId
    );
    [reserveVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("reserve_vault"), baseMint.toBuffer()],
      program.programId
    );
    [receiptMintPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("receipt_mint"), baseMint.toBuffer()],
      program.programId
    );
  });

  it("bootstraps ReservePool with 50 bps exit fee", async () => {
    await program.methods
      .bootstrapPool(50)
      .accounts({
        pool: poolPda,
        reserveAccount: reserveVaultPda,
        receiptMint: receiptMintPda,
        baseMint,
        curator: curator.publicKey,
      } as any)
      .rpc();

    const pool = await program.account.reservePool.fetch(poolPda);
    assert.equal(pool.curator.toBase58(), curator.publicKey.toBase58());
    assert.equal(pool.feeBps, 50);
    assert.equal(pool.paused, false);
    assert.equal(pool.managedAssets.toNumber(), 0);
  });

  it("registers MemberLedger PDAs for participants", async () => {
    for (const kp of [participantA, participantB]) {
      await program.methods
        .registerMember()
        .accounts({
          pool: poolPda,
          ledger: ledgerPda(kp.publicKey),
          participant: kp.publicKey,
        } as any)
        .signers([kp])
        .rpc();

      const ledger = await program.account.memberLedger.fetch(ledgerPda(kp.publicKey));
      assert.equal(ledger.participant.toBase58(), kp.publicKey.toBase58());
      assert.equal(ledger.receiptBalance.toNumber(), 0);
    }
  });

  it("contributes liquidity and mints proportional receipt tokens", async () => {
    for (const kp of [participantA, participantB, unregisteredUser]) {
      const k = kp.publicKey.toBase58();
      baseAccounts[k] = await createAccount(provider.connection, curator, baseMint, kp.publicKey);
      receiptAccounts[k] = await createAccount(
        provider.connection,
        curator,
        receiptMintPda,
        kp.publicKey
      );
      await mintTo(provider.connection, curator, baseMint, baseAccounts[k], curator, 500 * DECIMALS_UNIT);
    }

    await program.methods
      .contributeLiquidity(new anchor.BN(100 * DECIMALS_UNIT))
      .accounts({
        pool: poolPda,
        ledger: ledgerPda(participantA.publicKey),
        reserveAccount: reserveVaultPda,
        receiptMint: receiptMintPda,
        participantTokenAccount: baseAccounts[participantA.publicKey.toBase58()],
        participantReceiptAccount: receiptAccounts[participantA.publicKey.toBase58()],
        participant: participantA.publicKey,
      } as any)
      .signers([participantA])
      .rpc();

    const rAcc = await getAccount(
      provider.connection,
      receiptAccounts[participantA.publicKey.toBase58()]
    );
    assert.equal(rAcc.amount, BigInt(100 * DECIMALS_UNIT));
  });

  it("redeems liquidity via CPI burn and PDA transfer with exit fee", async () => {
    const before = await getAccount(
      provider.connection,
      baseAccounts[participantA.publicKey.toBase58()]
    );

    await program.methods
      .redeemLiquidity(new anchor.BN(40 * DECIMALS_UNIT))
      .accounts({
        pool: poolPda,
        ledger: ledgerPda(participantA.publicKey),
        reserveAccount: reserveVaultPda,
        receiptMint: receiptMintPda,
        participantTokenAccount: baseAccounts[participantA.publicKey.toBase58()],
        participantReceiptAccount: receiptAccounts[participantA.publicKey.toBase58()],
        participant: participantA.publicKey,
      } as any)
      .signers([participantA])
      .rpc();

    const after = await getAccount(
      provider.connection,
      baseAccounts[participantA.publicKey.toBase58()]
    );
    // 40.0 tokens gross minus 0.5% (50 bps) = 39.8 tokens
    assert.equal(after.amount - before.amount, BigInt(39_800_000));
  });

  it("enforces curator authorization on rebalance swap and position deployment", async () => {
    let swapRejected = false;
    try {
      await program.methods
        .executeRebalanceSwap(new anchor.BN(1_000_000), new anchor.BN(950_000), new anchor.BN(0))
        .accounts({
          curator: participantA.publicKey,
          pool: poolPda,
        } as any)
        .signers([participantA])
        .rpc();
    } catch {
      swapRejected = true;
    }
    assert.isTrue(swapRejected);

    let tickRejected = false;
    try {
      await program.methods
        .deployRangePosition(120, -120, 0, 0, new anchor.BN(500), new anchor.BN(100), new anchor.BN(100))
        .accounts({
          curator: curator.publicKey,
          pool: poolPda,
        } as any)
        .rpc();
    } catch {
      tickRejected = true;
    }
    assert.isTrue(tickRejected);
  });
});
