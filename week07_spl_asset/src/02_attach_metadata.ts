import { PublicKey } from "@solana/web3.js";
import * as fs from "fs";
import {
  MINT_RECORD_FILE,
  explorerLink,
  loadOrCreateOperator,
  loadRpcConnection,
} from "./config";

const METAPLEX_METADATA_PROGRAM = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

async function main() {
  if (!fs.existsSync(MINT_RECORD_FILE)) {
    throw new Error("Run npm run init-mint first.");
  }
  const { mint: mintStr } = JSON.parse(fs.readFileSync(MINT_RECORD_FILE, "utf-8"));
  const mint = new PublicKey(mintStr);
  const connection = loadRpcConnection();
  const operator = loadOrCreateOperator();

  const [metadataPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), METAPLEX_METADATA_PROGRAM.toBuffer(), mint.toBuffer()],
    METAPLEX_METADATA_PROGRAM
  );

  const { createUmi } = await import("@metaplex-foundation/umi-bundle-defaults");
  const { createSignerFromKeypair, signerIdentity, publicKey: umiPk } = await import(
    "@metaplex-foundation/umi"
  );
  const { createMetadataAccountV3, mplTokenMetadata } = await import(
    "@metaplex-foundation/mpl-token-metadata"
  );
  const { fromWeb3JsKeypair } = await import("@metaplex-foundation/umi-web3js-adapters");

  const umi = createUmi(connection.rpcEndpoint).use(mplTokenMetadata());
  const signer = createSignerFromKeypair(umi, fromWeb3JsKeypair(operator));
  umi.use(signerIdentity(signer));

  await createMetadataAccountV3(umi, {
    metadata: umiPk(metadataPda.toBase58()),
    mint: umiPk(mint.toBase58()),
    mintAuthority: signer,
    payer: signer,
    updateAuthority: signer.publicKey,
    data: {
      name: "Kinetix Protocol Token",
      symbol: "KTX",
      uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-syntax/tokens/tokens/.assets/spl-token.json",
      sellerFeeBasisPoints: 0,
      creators: null,
      collection: null,
      uses: null,
    },
    isMutable: true,
    collectionDetails: null,
  }).sendAndConfirm(umi);

  console.log(`Attached Metaplex V3 metadata PDA: ${metadataPda.toBase58()}`);
  console.log(`Explorer: ${explorerLink(mint.toBase58(), "address")}`);
}

main().catch(console.error);
