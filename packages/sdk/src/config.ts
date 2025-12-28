import { PublicKey } from "@solana/web3.js";

/**
 * Basic configuration used to construct a Unit09 client.
 */
export interface Unit09SdkConfig {
  endpoint: string;
  programId: PublicKey;
  /**
   * Optional commitment to use for all RPC calls.
   */
  commitment?: "processed" | "confirmed" | "finalized";
}

export const DEFAULT_COMMITMENT: Unit09SdkConfig["commitment"] = "confirmed";

/**
 * Convenience helper to build a config from raw values.
 */
export function createSdkConfig(endpoint: string, programId: string): Unit09SdkConfig {
  return {
    endpoint,
    programId: new PublicKey(programId),
    commitment: DEFAULT_COMMITMENT
  };
}
