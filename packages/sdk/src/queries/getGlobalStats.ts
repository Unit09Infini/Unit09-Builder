import type { Program } from "@coral-xyz/anchor";
import type { GlobalMetricsSnapshot } from "@unit09/shared-types";
import { PublicKey } from "@solana/web3.js";

/**
 * Fetch the global metrics account if present and project it into
 * the GlobalMetricsSnapshot shape.
 */
export async function getGlobalStats(
  program: Program
): Promise<GlobalMetricsSnapshot | null> {
  const [metricsPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("metrics")],
    program.programId
  );

  try {
    const acc: any = await program.account.metrics.fetch(metricsPda);
    return {
      totalRepos: BigInt(acc.totalRepos ?? 0),
      totalModules: BigInt(acc.totalModules ?? 0),
      totalForks: BigInt(acc.totalForks ?? 0),
      totalObservations: BigInt(acc.totalObservations ?? 0),
      totalLinesOfCode: BigInt(acc.totalLinesOfCode ?? 0),
      totalFilesProcessed: BigInt(acc.totalFilesProcessed ?? 0)
    };
  } catch {
    return null;
  }
}
