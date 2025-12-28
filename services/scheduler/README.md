# @unit09/scheduler

Scheduler service for the Unit09 project. This process is responsible for
triggering recurring jobs such as observing repositories, syncing metrics
on-chain, and cleaning up stale data.

## Responsibilities

- Periodically trigger observation of tracked repositories
- Schedule on-chain sync tasks
- Run cleanup routines for stale jobs or temporary data

## Configuration

Environment variables:

- `UNIT09_SCHED_LOG_LEVEL` — log level (`silent`, `error`, `warn`, `info`, `debug`)
- `UNIT09_SCHED_OBSERVE_CRON` — cron expression for observation (default: every 5 minutes)
- `UNIT09_SCHED_SYNC_CRON` — cron expression for sync (default: every 15 minutes)
- `UNIT09_SCHED_CLEANUP_CRON` — cron expression for cleanup (default: hourly)

## Development

```bash
npm install
npm run dev
```

## Production

```bash
npm run build
npm start
```
