/**
 * Simple in-memory queue client used to simulate asynchronous pipeline jobs.
 */

export type QueueJobStatus = "pending" | "running" | "completed" | "failed";

export interface QueueJob<TPayload = unknown, TResult = unknown> {
  id: string;
  createdAt: number;
  status: QueueJobStatus;
  payload: TPayload;
  result?: TResult;
  error?: string;
}

const jobs: Record<string, QueueJob<any, any>> = {};

export interface QueueClient<TPayload = unknown, TResult = unknown> {
  enqueue(payload: TPayload): QueueJob<TPayload, TResult>;
  getJob(id: string): QueueJob<TPayload, TResult> | undefined;
  listJobs(): QueueJob<TPayload, TResult>[];
  completeJob(id: string, result: TResult): void;
  failJob(id: string, error: string): void;
}

export function createInMemoryQueueClient<TPayload = unknown, TResult = unknown>(): QueueClient<TPayload, TResult> {
  return {
    enqueue(payload: TPayload): QueueJob<TPayload, TResult> {
      const id = `job-${Date.now()}-${Math.random().toString(16).slice(2)}`;
      const job: QueueJob<TPayload, TResult> = {
        id,
        createdAt: Date.now(),
        status: "pending",
        payload
      };
      jobs[id] = job;
      return job;
    },
    getJob(id: string) {
      return jobs[id] as QueueJob<TPayload, TResult> | undefined;
    },
    listJobs() {
      return Object.values(jobs) as QueueJob<TPayload, TResult>[];
    },
    completeJob(id: string, result: TResult) {
      const job = jobs[id];
      if (!job) return;
      job.status = "completed";
      job.result = result;
    },
    failJob(id: string, error: string) {
      const job = jobs[id];
      if (!job) return;
      job.status = "failed";
      job.error = error;
    }
  };
}
