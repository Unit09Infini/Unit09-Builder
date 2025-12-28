import cron from "node-cron";
import { createLogger } from "pino";
import { loadSchedulerConfig } from "./config";
import { registerPeriodicObserve } from "./schedules/periodicObserve";
import { registerPeriodicSync } from "./schedules/periodicSync";
import { registerCleanup } from "./schedules/cleanup";

async function main() {
  const config = loadSchedulerConfig();
  const logger = createLogger({ level: config.logLevel });

  logger.info("Unit09 scheduler starting");

  registerPeriodicObserve(cron, logger, config);
  registerPeriodicSync(cron, logger, config);
  registerCleanup(cron, logger, config);

  logger.info("All schedules registered");
}

main().catch((err) => {
  // eslint-disable-next-line no-console
  console.error("Fatal error in Unit09 scheduler:", err);
  process.exit(1);
});
