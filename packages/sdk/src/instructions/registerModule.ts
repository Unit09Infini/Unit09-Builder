import type { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

/**
 * Arguments for registering a new module.
 */
export interface RegisterModuleArgs {
  moduleKey: PublicKey;
  repoKey: PublicKey;
  name: string;
  kind: number;
  tags: string;
  metadataUri?: string;
}

/**
 * Build method call for module registration.
 */
export function buildRegisterModuleInstruction(
  program: Program,
  args: RegisterModuleArgs,
  payer: PublicKey
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
    .registerModule({
      moduleKey: args.moduleKey,
      repoKey: args.repoKey,
      name: args.name,
      kind: args.kind,
      tags: args.tags,
      metadataUri: args.metadataUri ?? null
    })
    .accounts({
      module: modulePda,
      repo: repoPda,
      payer,
      systemProgram: SystemProgram.programId
    });
}
