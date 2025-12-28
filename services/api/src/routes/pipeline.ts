import type { Router } from "express";
import type { PipelineControllerDeps } from "../controllers/pipelineController";
import { createPipelineController } from "../controllers/pipelineController";

export function registerPipelineRoutes(router: Router, deps: PipelineControllerDeps) {
  const controller = createPipelineController(deps);

  router.post("/pipeline/jobs", controller.enqueuePipeline);
  router.get("/pipeline/jobs", controller.listJobs);
  router.get("/pipeline/jobs/:jobId", controller.getJob);
}
