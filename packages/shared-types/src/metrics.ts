export interface GlobalMetricsSnapshot {
  totalRepos: bigint;
  totalModules: bigint;
  totalForks: bigint;
  totalObservations: bigint;
  totalLinesOfCode: bigint;
  totalFilesProcessed: bigint;
}

export interface GlobalMetricsSnapshotJson {
  totalRepos: string;
  totalModules: string;
  totalForks: string;
  totalObservations: string;
  totalLinesOfCode: string;
  totalFilesProcessed: string;
}

export interface RepoMetricsSnapshot {
  repoKey: string;
  modules: number;
  forks: number;
  observations: number;
  linesOfCode: bigint;
  filesProcessed: bigint;
}

export interface RepoMetricsSnapshotJson {
  repoKey: string;
  modules: number;
  forks: number;
  observations: number;
  linesOfCode: string;
  filesProcessed: string;
}

export interface ModuleMetricsSnapshot {
  moduleKey: string;
  forks: number;
  runs: number;
  lastRunAt?: number;
}

export function globalMetricsToJson(input: GlobalMetricsSnapshot): GlobalMetricsSnapshotJson {
  return {
    totalRepos: input.totalRepos.toString(),
    totalModules: input.totalModules.toString(),
    totalForks: input.totalForks.toString(),
    totalObservations: input.totalObservations.toString(),
    totalLinesOfCode: input.totalLinesOfCode.toString(),
    totalFilesProcessed: input.totalFilesProcessed.toString(),
  };
}

export function globalMetricsFromJson(input: GlobalMetricsSnapshotJson): GlobalMetricsSnapshot {
  return {
    totalRepos: BigInt(input.totalRepos),
    totalModules: BigInt(input.totalModules),
    totalForks: BigInt(input.totalForks),
    totalObservations: BigInt(input.totalObservations),
    totalLinesOfCode: BigInt(input.totalLinesOfCode),
    totalFilesProcessed: BigInt(input.totalFilesProcessed),
  };
}

export function repoMetricsToJson(input: RepoMetricsSnapshot): RepoMetricsSnapshotJson {
  return {
    repoKey: input.repoKey,
    modules: input.modules,
    forks: input.forks,
    observations: input.observations,
    linesOfCode: input.linesOfCode.toString(),
    filesProcessed: input.filesProcessed.toString(),
  };
}

export function repoMetricsFromJson(input: RepoMetricsSnapshotJson): RepoMetricsSnapshot {
  return {
    repoKey: input.repoKey,
    modules: input.modules,
    forks: input.forks,
    observations: input.observations,
    linesOfCode: BigInt(input.linesOfCode),
    filesProcessed: BigInt(input.filesProcessed),
  };
}
