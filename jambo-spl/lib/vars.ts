import dotenv from "dotenv";

import { clusterApiUrl } from "@solana/web3.js";
import { loadKeypairFromFile, loadOrGenerateKeypair } from "./helpers";

// load the env variables from file
dotenv.config();

/**
 * Load the `payer` keypair from the local file system, or load/generate a new
 * one and storing it within the local directory
 */
export const payer = process.env?.LOCAL_PAYER_JSON_ABSPATH
  ? loadKeypairFromFile(process.env?.LOCAL_PAYER_JSON_ABSPATH)
  : loadOrGenerateKeypair("payer");

/**
 * Load the `owner` keypair from the local file system, or load/generate a new
 * one and storing it within the local directory
 */
export const owner = process.env?.LOCAL_OWNER_JSON_ABSPATH
  ? loadKeypairFromFile(process.env?.LOCAL_OWNER_JSON_ABSPATH)
  : loadOrGenerateKeypair("owner");

// load the env variables and store the cluster RPC url
export const CLUSTER_URL = process.env?.RPC_URL ?? clusterApiUrl("devnet");
