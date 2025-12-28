import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

/**
 * Arguments for linking a module to a repo or another module.
 */
export interface LinkModuleArgs {
  moduleKey: PublicKey;
  repoKey: PublicKey;
}

/**
 * Build method call for link-module instruction.
 */
export function buildLinkModuleInstruction(
  program: Program,
  args: LinkModuleArgs,
  authority: PublicKey
) {
  const [modulePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("module"), args.moduleKey.toBuffer()],
    program.programId
  );
  const [repoPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("repo"), args.repoKey.toBuffer()],
    program.programId
  );

  return program.methods
    .linkModule({
      // If the IDL expects a struct, adapt here.
    })
    .accounts({
      module: modulePda,
      repo: repoPda,
      authority
    });
}
