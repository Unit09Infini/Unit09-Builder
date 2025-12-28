import type cron from "node-cron";
import type { Logger } from "pino";
import type { SchedulerConfig } from "../config";

/**
 * Schedule that cleans up stale jobs or old temporary data.
 */
export function registerCleanup(
  cronLib: typeof cron,
  logger: Logger,
  config: SchedulerConfig
): void {
  cronLib.schedule(config.cleanupCron, () => {
    logger.info("Running cleanup schedule");
    // Here you would remove stale jobs or rotate logs.
  });
}
