import express, { type Express } from "express";
import cors from "cors";
import bodyParser from "body-parser";
import type { Logger } from "pino";
import type { ApiConfig } from "./config";
import { createErrorHandler } from "./middleware/errorHandler";
import { createRequestLogger } from "./middleware/requestLogger";
import { createAuthGuard } from "./middleware/authGuard";
import { rateLimiter } from "./middleware/rateLimiter";
import { registerHealthRoutes } from "./routes/health";
import { registerRepoRoutes } from "./routes/repos";
import { registerModuleRoutes } from "./routes/modules";
import { registerForkRoutes } from "./routes/forks";
import { registerPipelineRoutes } from "./routes/pipeline";
import { registerStatsRoutes } from "./routes/stats";
import { createUnit09SdkContext } from "./clients/unit09SdkClient";
import { createUnit09Service } from "./services/unit09Service";
import { createCoreEngineClient } from "./clients/coreEngineClient";
import { createInMemoryQueueClient } from "./clients/queueClient";
import { createPipelineService } from "./services/pipelineService";

export async function createServer(config: ApiConfig, logger: Logger): Promise<Express> {
  const app = express();

  if (config.corsEnabled) {
    app.use(cors());
  }

  app.use(bodyParser.json({ limit: "2mb" }));
  app.use(createRequestLogger(logger));
  app.use(rateLimiter);
  app.use(createAuthGuard(config));

  const router = express.Router();

  const sdkCtx = createUnit09SdkContext(config);
  const unit09Service = createUnit09Service(sdkCtx);
  const engineClient = createCoreEngineClient();
  const queueClient = createInMemoryQueueClient();
  const pipelineService = createPipelineService(engineClient, queueClient);

  registerHealthRoutes(router);
  registerRepoRoutes(router, { unit09Service });
  registerModuleRoutes(router, { unit09Service });
  registerForkRoutes(router, { unit09Service });
  registerPipelineRoutes(router, { pipelineService });
  registerStatsRoutes(router, { unit09Service });

  app.use("/api", router);

  app.use(createErrorHandler(logger));

  return app;
}
