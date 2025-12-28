import { createLogger as createPinoLogger, Logger as PinoLogger } from "pino";
import type { WorkerConfig } from "../config";

export type WorkerLogger = PinoLogger;

export function createLogger(config: WorkerConfig): WorkerLogger {
  return createPinoLogger({
    level: config.logLevel,
    transport: process.env.NODE_ENV === "production" ? undefined : {
      target: "pino-pretty",
      options: {
        colorize: true,
        translateTime: "SYS:standard"
      }
    }
  });
}
