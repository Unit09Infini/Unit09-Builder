import { loadWorkerConfig } from "./config";
import { createLogger } from "./utils/logger";
import { createWorkerMetrics } from "./utils/metrics";
import { createInMemoryJobQueue } from "./queue/jobQueue";
import { handlers } from "./queue/jobHandlers";

async function main() {
  const config = loadWorkerConfig();
  const logger = createLogger(config);
  const metrics = createWorkerMetrics();
  const queue = createInMemoryJobQueue();

  logger.info("Unit09 worker starting");

  let activeJobs = 0;

  const tick = async () => {
    if (activeJobs >= config.maxConcurrentJobs) {
      return;
    }
    const job = queue.next();
    if (!job) {
      return;
    }

    const handler = handlers[job.type];
    if (!handler) {
      logger.error({ jobId: job.id, type: job.type }, "No handler for job type");
      return;
    }

    job.startedAt = Date.now();
    activeJobs += 1;
    metrics.jobsStarted.inc();
    metrics.activeJobs.set(activeJobs);

    try {
      const result = await handler(job, { logger, metrics });
      queue.markCompleted(job, result);
      metrics.jobsCompleted.inc();
      logger.info({ jobId: job.id, type: job.type }, "Job completed");
    } catch (err: any) {
      job.attempts += 1;
      metrics.jobsFailed.inc();
      logger.error({ jobId: job.id, type: job.type, err }, "Job failed");
    } finally {
      activeJobs -= 1;
      metrics.activeJobs.set(activeJobs);
    }
  };

  setInterval(tick, config.pollIntervalMs);

  logger.info(
    {
      pollIntervalMs: config.pollIntervalMs,
      maxConcurrentJobs: config.maxConcurrentJobs
    },
    "Worker loop started"
  );
}

main().catch((err) => {
  // eslint-disable-next-line no-console
  console.error("Fatal error in Unit09 worker:", err);
  process.exit(1);
});
