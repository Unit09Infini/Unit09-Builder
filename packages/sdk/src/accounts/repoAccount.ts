import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

/**
 * Fetch a single repo account by its repo key (public key).
 */
export async function fetchRepoAccount(
  program: Program,
  repoKey: PublicKey
): Promise<{ pubkey: PublicKey; data: any } | null> {
  const [repoPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("repo"), repoKey.toBuffer()],
    program.programId
  );
  try {
    const data = await program.account.repo.fetch(repoPda);
    return { pubkey: repoPda, data };
  } catch {
    return null;
  }
}
