import type { RepoMetadata } from "./repo";
import type { ModuleMetadata, ModuleVersionSnapshot } from "./module";

export type PipelineStage =
  | "observe-code"
  | "detect-language"
  | "parse-project"
  | "build-code-graph"
  | "decompose-modules"
  | "generate-artifacts"
  | "assemble-build-plan"
  | "validate-modules"
  | "sync-on-chain";

export interface PipelineSource {
  repo: RepoMetadata;
  revision: string;
  localPath?: string;
}

export interface PipelineContext {
  source: PipelineSource;
  stage: PipelineStage;
  graphId?: string;
  startedAt: number;
  metadata: Record<string, unknown>;
}

export interface PipelineCheckpoint {
  stage: PipelineStage;
  startedAt: number;
  completedAt?: number;
  diagnostics: string[];
}

export interface PipelineOutput {
  modules: ModuleMetadata[];
  versions: ModuleVersionSnapshot[];
  diagnostics: string[];
  checkpoints: PipelineCheckpoint[];
  completedAt: number;
}

export interface PipelineError {
  code: string;
  message: string;
  stage: PipelineStage;
  cause?: unknown;
}

export function createInitialPipelineContext(source: PipelineSource): PipelineContext {
  return {
    source,
    stage: "observe-code",
    startedAt: Date.now(),
    metadata: {},
  };
}
