import type { Unit09SdkContext } from "../clients/unit09SdkClient";
import { createUnit09SdkClient } from "../clients/unit09SdkClient";

export function createUnit09Service(sdkCtx: Unit09SdkContext) {
  const client = createUnit09SdkClient(sdkCtx);

  return {
    async listRepos() {
      return client.getRepos();
    },
    async listModulesByRepo(repoKey: string) {
      return client.getModulesByRepo(repoKey);
    },
    async listForks() {
      return client.getForks();
    },
    async getGlobalStats() {
      return client.getStats();
    }
  };
}
