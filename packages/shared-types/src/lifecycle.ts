export type LifecycleStatus =
  | "created"
  | "observing"
  | "stable"
  | "degraded"
  | "error"
  | "archived";

export interface LifecycleSummary {
  createdAt: number;
  lastActivityAt: number;
  lastObservationAt?: number;
  lastErrorAt?: number;
  status?: LifecycleStatus;
}

export type LifecycleEventKind =
  | "repo-created"
  | "repo-updated"
  | "module-registered"
  | "module-updated"
  | "fork-created"
  | "fork-updated"
  | "observation-started"
  | "observation-completed"
  | "metrics-updated"
  | "error";

export interface LifecycleEvent {
  id: string;
  kind: LifecycleEventKind;
  subjectType: "repo" | "module" | "fork" | "global";
  subjectKey: string;
  message: string;
  timestamp: number;
  errorCode?: string;
}

export function applyLifecycleEvent(
  summary: LifecycleSummary,
  event: LifecycleEvent
): LifecycleSummary {
  const next: LifecycleSummary = { ...summary };
  next.lastActivityAt = Math.max(next.lastActivityAt, event.timestamp);

  if (event.kind === "observation-started" || event.kind === "observation-completed") {
    next.lastObservationAt = Math.max(next.lastObservationAt ?? 0, event.timestamp);
  }

  if (event.kind === "error") {
    next.lastErrorAt = Math.max(next.lastErrorAt ?? 0, event.timestamp);
    next.status = "error";
  }

  if (!next.status) {
    next.status = "created";
  }

  return next;
}
