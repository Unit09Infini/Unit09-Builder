import type { Router } from "express";
import type { ForksControllerDeps } from "../controllers/forksController";
import { createForksController } from "../controllers/forksController";

export function registerForkRoutes(router: Router, deps: ForksControllerDeps) {
  const controller = createForksController(deps);
  router.get("/forks", controller.listForks);
}
