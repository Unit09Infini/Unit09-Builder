import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

/**
 * Arguments for a repo update instruction.
 */
export interface UpdateRepoArgs {
  repoKey: PublicKey;
  name?: string;
  url?: string;
  tags?: string;
  allowObservation?: boolean;
  isActive?: boolean;
}

/**
 * Build method call for updating repo metadata.
 */
export function buildUpdateRepoInstruction(
  program: Program,
  args: UpdateRepoArgs,
  authority: PublicKey
) {
  const [repoPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("repo"), args.repoKey.toBuffer()],
    program.programId
  );

  return program.methods
    .updateRepo({
      name: args.name ?? null,
      url: args.url ?? null,
      tags: args.tags ?? null,
      allowObservation: args.allowObservation ?? null,
      isActive: args.isActive ?? null
    })
    .accounts({
      repo: repoPda,
      authority
    });
}
