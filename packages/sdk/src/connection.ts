import { Connection, PublicKey, Commitment } from "@solana/web3.js";
import type { Unit09SdkConfig } from "./config";
import { DEFAULT_COMMITMENT } from "./config";

/**
 * Shared connection context used by the client and helpers.
 */
export interface ConnectionContext {
  connection: Connection;
  programId: PublicKey;
  commitment: Commitment;
}

/**
 * Create a ConnectionContext from an SDK config.
 */
export function createConnectionContext(config: Unit09SdkConfig): ConnectionContext {
  const commitment = (config.commitment ?? DEFAULT_COMMITMENT) as Commitment;
  return {
    connection: new Connection(config.endpoint, commitment),
    programId: config.programId,
    commitment
  };
}
