import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

/**
 * Arguments for updating module metadata.
 */
export interface UpdateModuleArgs {
  moduleKey: PublicKey;
  name?: string;
  tags?: string;
  metadataUri?: string;
  isActive?: boolean;
}

/**
 * Build method call for updating a module.
 */
export function buildUpdateModuleInstruction(
  program: Program,
  args: UpdateModuleArgs,
  authority: PublicKey
) {
  const [modulePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("module"), args.moduleKey.toBuffer()],
    program.programId
  );

  return program.methods
    .updateModule({
      name: args.name ?? null,
      tags: args.tags ?? null,
      metadataUri: args.metadataUri ?? null,
      isActive: args.isActive ?? null
    })
    .accounts({
      module: modulePda,
      authority
    });
}
