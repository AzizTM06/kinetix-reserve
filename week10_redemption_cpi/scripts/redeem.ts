import { Keypair, PublicKey } from "@solana/web3.js";

const PROGRAM_ID = new PublicKey("AHQjFFEEtV378q7PbQ2FQ1bzP2FNg8yoUCErba6VG4sk");

function main() {
  const baseMint = Keypair.generate().publicKey;
  const [pool] = PublicKey.findProgramAddressSync(
    [Buffer.from("kinetix_pool"), baseMint.toBuffer()],
    PROGRAM_ID
  );
  const [reserveVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("reserve_vault"), baseMint.toBuffer()],
    PROGRAM_ID
  );
  console.log(`Redemption PDA pool=${pool.toBase58()}, vault=${reserveVault.toBase58()}`);
}

main();
