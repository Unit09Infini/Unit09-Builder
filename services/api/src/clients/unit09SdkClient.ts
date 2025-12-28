import { Connection, PublicKey } from "@solana/web3.js";
import { createUnit09Client, listRepos, listModulesByRepo, listForks, getGlobalStats } from "@unit09/sdk"; // hypothetical imports
import type { ApiConfig } from "../config";

/**
 * Thin wrapper around the Unit09 SDK.
 * You may need to adjust the imports to match the actual SDK surface.
 */

export interface Unit09SdkContext {
  connection: Connection;
  programId: PublicKey;
}

export function createUnit09SdkContext(config: ApiConfig): Unit09SdkContext {
  const connection = new Connection(config.solanaRpcUrl, "confirmed");
  const programId = new PublicKey(config.unit09ProgramId);
  return { connection, programId };
}

export function createUnit09SdkClient(ctx: Unit09SdkContext) {
  const client = createUnit09Client({
    connection: ctx.connection,
    programId: ctx.programId
  });

  return {
    sdk: client,
    async getRepos() {
      return listRepos(client);
    },
    async getModulesByRepo(repoKey: string) {
      return listModulesByRepo(client, { repoKey });
    },
    async getForks() {
      return listForks(client);
    },
    async getStats() {
      return getGlobalStats(client);
    }
  };
}
