import type { Request, Response, NextFunction } from "express";
import { TooManyRequestsError } from "../utils/httpErrors";

/**
 * Very simple in-memory rate limiter using an IP + path key.
 * This is intentionally basic and suitable only for small deployments.
 */
interface HitInfo {
  count: number;
  resetAt: number;
}

const WINDOW_MS = 60_000;
const MAX_HITS = 300;

const hits: Record<string, HitInfo> = {};

export function rateLimiter(req: Request, _res: Response, next: NextFunction) {
  const ip = req.ip || req.connection.remoteAddress || "unknown";
  const key = `${ip}:${req.path}`;
  const now = Date.now();

  let info = hits[key];
  if (!info || info.resetAt < now) {
    info = {
      count: 0,
      resetAt: now + WINDOW_MS
    };
    hits[key] = info;
  }

  info.count += 1;

  if (info.count > MAX_HITS) {
    throw new TooManyRequestsError();
  }

  return next();
}
