import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

/**
 * Fetch a single fork account by its fork key (public key).
 */
export async function fetchForkAccount(
  program: Program,
  forkKey: PublicKey
): Promise<{ pubkey: PublicKey; data: any } | null> {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("fork"), forkKey.toBuffer()],
    program.programId
  );
  try {
    const data = await program.account.fork.fetch(pda);
    return { pubkey: pda, data };
  } catch {
    return null;
  }
}
