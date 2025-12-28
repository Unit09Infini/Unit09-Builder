import type { RepoMetadata } from "./repo";
import type { ModuleMetadata } from "./module";
import type { ForkMetadata } from "./fork";
import type { GlobalMetricsSnapshot } from "./metrics";

export interface BaseUnit09Event {
  id: string;
  type: string;
  timestamp: number;
  txSignature?: string;
}

export interface ConfigInitializedEvent extends BaseUnit09Event {
  type: "config-initialized";
  admin: string;
}

export interface RepoRegisteredEvent extends BaseUnit09Event {
  type: "repo-registered";
  repo: RepoMetadata;
}

export interface RepoUpdatedEvent extends BaseUnit09Event {
  type: "repo-updated";
  before: RepoMetadata;
  after: RepoMetadata;
}

export interface ModuleRegisteredEvent extends BaseUnit09Event {
  type: "module-registered";
  module: ModuleMetadata;
}

export interface ModuleUpdatedEvent extends BaseUnit09Event {
  type: "module-updated";
  before: ModuleMetadata;
  after: ModuleMetadata;
}

export interface ForkCreatedEvent extends BaseUnit09Event {
  type: "fork-created";
  fork: ForkMetadata;
}

export interface ForkUpdatedEvent extends BaseUnit09Event {
  type: "fork-updated";
  before: ForkMetadata;
  after: ForkMetadata;
}

export interface MetricsRecordedEvent extends BaseUnit09Event {
  type: "metrics-recorded";
  snapshot: GlobalMetricsSnapshot;
}

export type Unit09Event =
  | ConfigInitializedEvent
  | RepoRegisteredEvent
  | RepoUpdatedEvent
  | ModuleRegisteredEvent
  | ModuleUpdatedEvent
  | ForkCreatedEvent
  | ForkUpdatedEvent
  | MetricsRecordedEvent;

export function asUnit09Event(input: any): Unit09Event | null {
  if (!input || typeof input !== "object") return null;
  if (typeof input.type !== "string") return null;
  switch (input.type) {
    case "config-initialized":
    case "repo-registered":
    case "repo-updated":
    case "module-registered":
    case "module-updated":
    case "fork-created":
    case "fork-updated":
    case "metrics-recorded":
      return input as Unit09Event;
    default:
      return null;
  }
}
