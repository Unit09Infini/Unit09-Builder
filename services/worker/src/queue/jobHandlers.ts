import type { Job, JobResult } from "./jobTypes";
import type { WorkerLogger } from "../utils/logger";
import type { WorkerMetrics } from "../utils/metrics";
import {
  runFullPipeline,
  observeCode,
  parseProject,
  buildCodeGraph,
  decomposeModules,
  generateModuleArtifacts,
  validateModules,
  syncOnChain
} from "@unit09/core-engine"; // you may adapt to the real core-engine API

export interface JobHandlerContext {
  logger: WorkerLogger;
  metrics: WorkerMetrics;
}

export type JobHandler = (job: Job, ctx: JobHandlerContext) => Promise<JobResult>;

export const handlers: Record<Job["type"], JobHandler> = {
  async observeRepo(job, ctx) {
    ctx.logger.info({ jobId: job.id, repoKey: job.payload.repoKey }, "Running observeRepo job");
    const payload: any = job.payload;
    const observed = await observeCode(payload.source);
    return { success: true, output: observed };
  },
  async analyzeRepo(job, ctx) {
    ctx.logger.info({ jobId: job.id, repoKey: job.payload.repoKey }, "Running analyzeRepo job");
    const payload: any = job.payload;
    const project = await parseProject(payload.source);
    const graph = await buildCodeGraph(project);
    return { success: true, output: graph };
  },
  async decompose(job, ctx) {
    ctx.logger.info({ jobId: job.id, repoKey: job.payload.repoKey }, "Running decompose job");
    const payload: any = job.payload;
    const project = await parseProject(payload.source);
    const graph = await buildCodeGraph(project);
    const modules = await decomposeModules(graph);
    return { success: true, output: modules };
  },
  async generateModules(job, ctx) {
    ctx.logger.info({ jobId: job.id, repoKey: job.payload.repoKey }, "Running generateModules job");
    const payload: any = job.payload;
    const project = await parseProject(payload.source);
    const graph = await buildCodeGraph(project);
    const modules = await decomposeModules(graph);
    const artifacts = await generateModuleArtifacts(modules);
    return { success: true, output: artifacts };
  },
  async validateModules(job, ctx) {
    ctx.logger.info({ jobId: job.id, repoKey: job.payload.repoKey }, "Running validateModules job");
    const payload: any = job.payload;
    const project = await parseProject(payload.source);
    const graph = await buildCodeGraph(project);
    const modules = await decomposeModules(graph);
    const validation = await validateModules(modules, graph);
    return { success: true, output: validation };
  },
  async syncOnChain(job, ctx) {
    ctx.logger.info({ jobId: job.id, repoKey: job.payload.repoKey }, "Running syncOnChain job");
    const payload: any = job.payload;
    const result = await syncOnChain(payload.source);
    return { success: true, output: result };
  },
  async forkEvolution(job, ctx) {
    ctx.logger.info({ jobId: job.id, forkId: (job.payload as any).forkId }, "Running forkEvolution job");
    const payload: any = job.payload;
    const result = await runFullPipeline(payload.source, { forkId: payload.forkId });
    return { success: true, output: result };
  }
};
