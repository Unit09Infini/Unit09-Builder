import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { fetchRepoAccount } from "../accounts/repoAccount";

/**
 * Fetch repo account by a base58 public key string.
 */
export async function getRepoById(program: Program, repoKey: string): Promise<any | null> {
  const key = new PublicKey(repoKey);
  const acc = await fetchRepoAccount(program, key);
  return acc ? acc.data : null;
}
