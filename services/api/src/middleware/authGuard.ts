import type { Request, Response, NextFunction } from "express";
import type { ApiConfig } from "../config";
import { UnauthorizedError } from "../utils/httpErrors";

/**
 * Very small API key based guard. If no apiKey is configured,
 * the guard becomes a no-op.
 */
export function createAuthGuard(config: ApiConfig) {
  return (req: Request, _res: Response, next: NextFunction) => {
    if (!config.apiKey) {
      return next();
    }

    const headerKey = req.header("x-api-key");
    if (!headerKey || headerKey !== config.apiKey) {
      throw new UnauthorizedError();
    }
    return next();
  };
}
