/**
 * Small shared controller dependency types. The concrete shapes depend on
 * the SDK and engine implementations, so they are kept generic here.
 */

export interface Unit09Service {
  listRepos(): Promise<any[]>;
  listModulesByRepo(repoKey: string): Promise<any[]>;
  listForks(): Promise<any[]>;
  getGlobalStats(): Promise<any>;
}

export interface PipelineService {
  enqueuePipeline(source: any): any;
  getJob(id: string): any;
  listJobs(): any[];
}
