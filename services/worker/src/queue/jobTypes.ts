import type { PipelineSource } from "@unit09/shared-types";

export type JobType =
  | "observeRepo"
  | "analyzeRepo"
  | "decompose"
  | "generateModules"
  | "validateModules"
  | "syncOnChain"
  | "forkEvolution";

export interface BaseJobPayload {
  repoKey: string;
}

export interface ObserveRepoPayload extends BaseJobPayload {
  source: PipelineSource;
}

export interface AnalyzeRepoPayload extends BaseJobPayload {
  source: PipelineSource;
}

export interface DecomposePayload extends BaseJobPayload {
  source: PipelineSource;
}

export interface GenerateModulesPayload extends BaseJobPayload {
  source: PipelineSource;
}

export interface ValidateModulesPayload extends BaseJobPayload {
  source: PipelineSource;
}

export interface SyncOnChainPayload extends BaseJobPayload {
  source: PipelineSource;
}

export interface ForkEvolutionPayload extends BaseJobPayload {
  forkId: string;
}

export type JobPayload =
  | ObserveRepoPayload
  | AnalyzeRepoPayload
  | DecomposePayload
  | GenerateModulesPayload
  | ValidateModulesPayload
  | SyncOnChainPayload
  | ForkEvolutionPayload;

export interface Job<TPayload extends JobPayload = JobPayload> {
  id: string;
  type: JobType;
  payload: TPayload;
  createdAt: number;
  startedAt?: number;
  completedAt?: number;
  attempts: number;
  maxAttempts: number;
}

export interface JobResult {
  success: boolean;
  error?: string;
  output?: unknown;
}
