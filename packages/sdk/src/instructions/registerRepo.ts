import type { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

/**
 * Arguments for `register_repo`-style instruction.
 * Names must match the IDL exactly in a real implementation.
 */
export interface RegisterRepoArgs {
  repoKey: PublicKey;
  name: string;
  url: string;
  tags: string;
  allowObservation: boolean;
}

/**
 * Build method call for repo registration.
 */
export function buildRegisterRepoInstruction(
  program: Program,
  args: RegisterRepoArgs,
  payer: PublicKey
) {
  const [repoPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("repo"), args.repoKey.toBuffer()],
    program.programId
  );

  return program.methods
    .registerRepo({
      repoKey: args.repoKey,
      name: args.name,
      url: args.url,
      tags: args.tags,
      allowObservation: args.allowObservation
    })
    .accounts({
      repo: repoPda,
      payer,
      systemProgram: SystemProgram.programId
    });
}
