import type { Request, Response, NextFunction } from "express";
import { HttpError, InternalServerError } from "../utils/httpErrors";
import type { Logger } from "pino";

/**
 * Express error handling middleware.
 */
export function createErrorHandler(logger: Logger) {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  return (err: unknown, req: Request, res: Response, _next: NextFunction) => {
    let httpError: HttpError;

    if (err instanceof HttpError) {
      httpError = err;
    } else {
      httpError = new InternalServerError("Unexpected error", err);
    }

    logger.error(
      {
        err: httpError,
        path: req.path,
        method: req.method
      },
      "Request failed"
    );

    res.status(httpError.status).json({
      error: {
        message: httpError.message,
        status: httpError.status
      }
    });
  };
}
