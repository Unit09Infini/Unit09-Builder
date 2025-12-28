import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

/**
 * Fetch the global config account if it exists.
 * Returns null if the account is not found.
 */
export async function fetchConfigAccount(program: Program): Promise<any | null> {
  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  try {
    const account = await program.account.config.fetch(configPda);
    return {
      pubkey: configPda,
      data: account
    };
  } catch {
    return null;
  }
}
