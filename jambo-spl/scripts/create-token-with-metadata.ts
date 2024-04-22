import {
  mplTokenMetadata,
  createAndMint,
  TokenStandard,
} from "@metaplex-foundation/mpl-token-metadata";
import { mplCandyMachine } from "@metaplex-foundation/mpl-candy-machine";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  signerIdentity,
  signerPayer,
  generateSigner,
  createSignerFromKeypair,
  percentAmount,
} from "@metaplex-foundation/umi";
import "@solana/web3.js";
import bs58 from "bs58";

import metadata from "./metadata";
import { CLUSTER_URL, payer, owner } from "../lib/vars";
import { explorerURL, printConsoleSeparator } from "../lib/helpers";

(async () => {
  const umi = createUmi(CLUSTER_URL);

  console.log("\n");
  console.log("Jambo Solana Token Creator");

  printConsoleSeparator();

  // generate a new keypair to be used for our mint
  const mint = generateSigner(umi);
  console.log("Token address:", mint.publicKey);

  //////////////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////////////

  console.log("Owner address:", owner.publicKey.toBase58());
  console.log("Payer address:", payer.publicKey.toBase58());

  //////////////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////////////

  const ownerKeypair = umi.eddsa.createKeypairFromSecretKey(owner.secretKey);
  const payerKeypair = umi.eddsa.createKeypairFromSecretKey(payer.secretKey);

  //////////////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////////////

  // metaplex plugins
  umi.use(mplTokenMetadata());
  umi.use(mplCandyMachine());

  umi.use(signerIdentity(createSignerFromKeypair(umi, ownerKeypair), false));
  umi.use(signerPayer(createSignerFromKeypair(umi, payerKeypair)));

  //////////////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////////////

  printConsoleSeparator();

  const tx = createAndMint(umi, {
    mint,
    authority: umi.identity,
    name: metadata.name,
    symbol: metadata.symbol,
    uri: metadata.uri,
    sellerFeeBasisPoints: percentAmount(0),
    decimals: metadata.decimals,
    amount: metadata.supply * Math.pow(10, metadata.decimals),
    tokenOwner: umi.identity.publicKey,
    tokenStandard: TokenStandard.Fungible,
  });

  try {
    const sig = await tx.sendAndConfirm(umi);
    console.log("Transaction completed.");
    console.log(explorerURL({ txSignature: bs58.encode(sig.signature) }));
  } catch (error) {
    console.error("Failed to send transaction:");
    console.error(error);
  }
})();
