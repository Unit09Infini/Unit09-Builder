import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

/**
 * Fetch a single module account by its module key (public key).
 */
export async function fetchModuleAccount(
  program: Program,
  moduleKey: PublicKey
): Promise<{ pubkey: PublicKey; data: any } | null> {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("module"), moduleKey.toBuffer()],
    program.programId
  );
  try {
    const data = await program.account.module.fetch(pda);
    return { pubkey: pda, data };
  } catch {
    return null;
  }
}
