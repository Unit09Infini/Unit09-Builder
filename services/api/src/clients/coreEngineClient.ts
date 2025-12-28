import { runFullPipeline } from "@unit09/core-engine";
import type { PipelineSource } from "@unit09/shared-types"; // hypothetical shared type

export interface CoreEngineClient {
  runPipeline(source: PipelineSource): ReturnType<typeof runFullPipeline>;
}

export function createCoreEngineClient(): CoreEngineClient {
  return {
    runPipeline(source) {
      return runFullPipeline(source, {});
    }
  };
}
