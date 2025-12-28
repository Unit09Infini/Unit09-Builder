import type cron from "node-cron";
import type { Logger } from "pino";
import type { SchedulerConfig } from "../config";

/**
 * Schedule that periodically triggers on-chain sync operations.
 */
export function registerPeriodicSync(
  cronLib: typeof cron,
  logger: Logger,
  config: SchedulerConfig
): void {
  cronLib.schedule(config.syncCron, () => {
    logger.info("Running periodic sync schedule");
    // Here you would enqueue syncOnChain jobs or trigger stats sync.
  });
}
