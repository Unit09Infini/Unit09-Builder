import type cron from "node-cron";
import type { Logger } from "pino";
import type { SchedulerConfig } from "../config";

/**
 * Schedule that periodically triggers repository observation jobs.
 * In a full deployment, this would enqueue jobs into the worker queue.
 */
export function registerPeriodicObserve(
  cronLib: typeof cron,
  logger: Logger,
  config: SchedulerConfig
): void {
  cronLib.schedule(config.observeCron, () => {
    logger.info("Running periodic observe schedule");
    // Here you would lookup known repositories and enqueue observe jobs.
  });
}
