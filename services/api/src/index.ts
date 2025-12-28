import { createServer } from "./server";
import { loadApiConfig } from "./config";
import { createLogger } from "pino";

async function main() {
  const config = loadApiConfig();
  const logger = createLogger({ level: config.logLevel });

  const app = await createServer(config, logger);
  app.listen(config.port, () => {
    logger.info({ port: config.port }, "Unit09 API server started");
  });
}

main().catch((err) => {
  // eslint-disable-next-line no-console
  console.error("Fatal error starting Unit09 API:", err);
  process.exit(1);
});
