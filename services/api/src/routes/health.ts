import type { Router } from "express";
import { getHealth } from "../controllers/healthController";

export function registerHealthRoutes(router: Router) {
  router.get("/health", getHealth);
}
