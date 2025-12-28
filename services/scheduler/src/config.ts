export interface SchedulerConfig {
  logLevel: string;
  observeCron: string;
  syncCron: string;
  cleanupCron: string;
}

export function loadSchedulerConfig(): SchedulerConfig {
  return {
    logLevel: process.env.UNIT09_SCHED_LOG_LEVEL || "info",
    observeCron: process.env.UNIT09_SCHED_OBSERVE_CRON || "*/5 * * * *",
    syncCron: process.env.UNIT09_SCHED_SYNC_CRON || "*/15 * * * *",
    cleanupCron: process.env.UNIT09_SCHED_CLEANUP_CRON || "0 * * * *"
  };
}
