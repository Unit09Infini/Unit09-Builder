export interface WorkerConfig {
  logLevel: string;
  pollIntervalMs: number;
  maxConcurrentJobs: number;
}

export function loadWorkerConfig(): WorkerConfig {
  return {
    logLevel: process.env.UNIT09_WORKER_LOG_LEVEL || "info",
    pollIntervalMs: parseInt(process.env.UNIT09_WORKER_POLL_INTERVAL_MS || "1000", 10),
    maxConcurrentJobs: parseInt(process.env.UNIT09_WORKER_MAX_CONCURRENCY || "4", 10)
  };
}
