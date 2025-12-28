import type { Request, Response, NextFunction } from "express";
import type { Logger } from "pino";

export function createRequestLogger(logger: Logger) {
  return (req: Request, res: Response, next: NextFunction) => {
    const start = Date.now();
    const { method, url } = req;

    res.on("finish", () => {
      const duration = Date.now() - start;
      logger.info(
        {
          method,
          url,
          status: res.statusCode,
          durationMs: duration
        },
        "Request completed"
      );
    });

    next();
  };
}
