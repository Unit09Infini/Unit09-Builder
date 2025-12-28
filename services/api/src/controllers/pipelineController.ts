import type { Request, Response } from "express";
import type { PipelineService } from "./types";
import type { PipelineSource } from "@unit09/shared-types";
import { BadRequestError } from "../utils/httpErrors";

export interface PipelineControllerDeps {
  pipelineService: PipelineService;
}

export function createPipelineController(deps: PipelineControllerDeps) {
  const { pipelineService } = deps;

  return {
    enqueuePipeline: async (req: Request, res: Response) => {
      const body = req.body as Partial<PipelineSource>;
      if (!body || !body.repo) {
        throw new BadRequestError("Pipeline source must include a repo descriptor");
      }
      const job = pipelineService.enqueuePipeline(body as PipelineSource);
      res.status(202).json({ job });
    },
    getJob: async (req: Request, res: Response) => {
      const jobId = req.params.jobId;
      const job = pipelineService.getJob(jobId);
      if (!job) {
        res.status(404).json({ error: { message: "Job not found" } });
        return;
      }
      res.json({ job });
    },
    listJobs: async (_req: Request, res: Response) => {
      const jobs = pipelineService.listJobs();
      res.json({ items: jobs });
    }
  };
}
