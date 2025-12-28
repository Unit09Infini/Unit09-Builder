import type { Job, JobPayload, JobResult } from "./jobTypes";

export interface JobQueue {
  enqueue<TPayload extends JobPayload>(type: Job<TPayload>["type"], payload: TPayload): Job<TPayload>;
  next(): Job | undefined;
  markCompleted(job: Job, result: JobResult): void;
  list(): Job[];
}

const jobs: Job[] = [];
const results: Record<string, JobResult> = {};

export function createInMemoryJobQueue(): JobQueue {
  return {
    enqueue(type, payload) {
      const job: Job = {
        id: `job-${Date.now()}-${Math.random().toString(16).slice(2)}`,
        type,
        payload,
        createdAt: Date.now(),
        attempts: 0,
        maxAttempts: 3
      };
      jobs.push(job);
      return job;
    },
    next() {
      return jobs.find((job) => !job.startedAt);
    },
    markCompleted(job, result) {
      job.completedAt = Date.now();
      results[job.id] = result;
    },
    list() {
      return [...jobs];
    }
  };
}

export function getJobResult(jobId: string): JobResult | undefined {
  return results[jobId];
}
