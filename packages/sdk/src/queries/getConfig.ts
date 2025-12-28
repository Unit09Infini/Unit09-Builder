import type { Program } from "@coral-xyz/anchor";
import type { GlobalMetricsSnapshot } from "@unit09/shared-types";
import { fetchConfigAccount } from "../accounts/configAccount";

/**
 * Fetch and return the config account data shape directly,
 * or null if not found.
 */
export async function getConfig(program: Program): Promise<any | null> {
  const acc = await fetchConfigAccount(program);
  return acc ? acc.data : null;
}
