import type { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

/**
 * Arguments for creating a fork.
 */
export interface CreateForkArgs {
  forkKey: PublicKey;
  parent?: PublicKey | null;
  label: string;
  tags: string;
  metadataUri?: string;
}

/**
 * Build method call for creating a fork.
 */
export function buildCreateForkInstruction(
  program: Program,
  args: CreateForkArgs,
  payer: PublicKey
) {
  const [forkPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("fork"), args.forkKey.toBuffer()],
    program.programId
  );
  const parentKey = args.parent ?? null;

  return program.methods
    .createFork({
      forkKey: args.forkKey,
      parent: parentKey,
      label: args.label,
      tags: args.tags,
      metadataUri: args.metadataUri ?? null
    })
    .accounts({
      fork: forkPda,
      payer,
      systemProgram: SystemProgram.programId
    });
}
