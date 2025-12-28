import type { CoreEngineClient } from "../clients/coreEngineClient";
import type { QueueClient, QueueJob } from "../clients/queueClient";
import type { PipelineSource } from "@unit09/shared-types";

export function createPipelineService(
  engineClient: CoreEngineClient,
  queueClient: QueueClient<PipelineSource, unknown>
) {
  return {
    enqueuePipeline(source: PipelineSource): QueueJob<PipelineSource, unknown> {
      const job = queueClient.enqueue(source);
      // In a real deployment, this would be processed by a worker.
      // For now, run immediately in the background.
      engineClient
        .runPipeline(source)
        .then((result) => queueClient.completeJob(job.id, result))
        .catch((err) => queueClient.failJob(job.id, String(err?.message ?? err)));
      return job;
    },
    getJob(id: string) {
      return queueClient.getJob(id);
    },
    listJobs() {
      return queueClient.listJobs();
    }
  };
}
