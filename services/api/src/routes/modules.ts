import type { Router } from "express";
import type { ModulesControllerDeps } from "../controllers/modulesController";
import { createModulesController } from "../controllers/modulesController";

export function registerModuleRoutes(router: Router, deps: ModulesControllerDeps) {
  const controller = createModulesController(deps);
  router.get("/modules/by-repo/:repoKey", controller.listModulesByRepo);
}
