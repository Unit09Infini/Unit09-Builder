import type { Router } from "express";
import type { StatsControllerDeps } from "../controllers/statsController";
import { createStatsController } from "../controllers/statsController";

export function registerStatsRoutes(router: Router, deps: StatsControllerDeps) {
  const controller = createStatsController(deps);
  router.get("/stats", controller.getStats);
}
