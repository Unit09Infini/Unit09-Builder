import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

/**
 * Arguments for updating fork metadata.
 */
export interface UpdateForkArgs {
  forkKey: PublicKey;
  label?: string;
  tags?: string;
  metadataUri?: string;
  isActive?: boolean;
}

/**
 * Build method call for updating a fork.
 */
export function buildUpdateForkInstruction(
  program: Program,
  args: UpdateForkArgs,
  authority: PublicKey
) {
  const [forkPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("fork"), args.forkKey.toBuffer()],
    program.programId
  );

  return program.methods
    .updateFork({
      label: args.label ?? null,
      tags: args.tags ?? null,
      metadataUri: args.metadataUri ?? null,
      isActive: args.isActive ?? null
    })
    .accounts({
      fork: forkPda,
      authority
    });
}
