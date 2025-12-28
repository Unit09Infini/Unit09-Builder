import type { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

/**
 * Arguments for the on-chain `initialize` instruction.
 *
 * The exact shape must correspond to the IDL for the Unit09 program.
 * This is a suggested structure for a typical config init flow.
 */
export interface InitializeArgs {
  admin: PublicKey;
  feeBps: number;
  maxModulesPerRepo: number;
}

/**
 * Build an Anchor method call for the `initialize` instruction.
 * The caller is responsible for sending the transaction.
 */
export function buildInitializeInstruction(
  program: Program,
  args: InitializeArgs,
  payer: PublicKey
) {
  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );
  const [metricsPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("metrics")],
    program.programId
  );
  const [lifecyclePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("lifecycle")],
    program.programId
  );
  const [globalMetadataPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("global_metadata")],
    program.programId
  );

  return program.methods
    // The struct name must match the IDL for the program.
    .initialize({
      admin: args.admin,
      feeBps: args.feeBps,
      maxModulesPerRepo: args.maxModulesPerRepo,
      policyRef: new Uint8Array(32),
      lifecycleNoteRef: new Uint8Array(32)
    })
    .accounts({
      config: configPda,
      metrics: metricsPda,
      lifecycle: lifecyclePda,
      globalMetadata: globalMetadataPda,
      admin: args.admin,
      payer,
      systemProgram: SystemProgram.programId
    });
}
