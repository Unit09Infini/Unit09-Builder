import type { Router } from "express";
import type { ReposControllerDeps } from "../controllers/reposController";
import { createReposController } from "../controllers/reposController";

export function registerRepoRoutes(router: Router, deps: ReposControllerDeps) {
  const controller = createReposController(deps);

  router.get("/repos", controller.listRepos);
  router.get("/repos/:repoKey/modules", controller.listModulesByRepo);
}
