# @unit09/worker

Background worker service for Unit09. This process pulls jobs from a queue and
executes core-engine pipeline stages such as observation, analysis, module
generation, validation, and on-chain synchronization.

## Responsibilities

- Poll a job queue for pending jobs
- Execute the appropriate core-engine operation for each job
- Track basic metrics for started, completed, and failed jobs
- Log job lifecycle events

## Configuration

Environment variables:

- `UNIT09_WORKER_LOG_LEVEL` — log level (`silent`, `error`, `warn`, `info`, `debug`)
- `UNIT09_WORKER_POLL_INTERVAL_MS` — polling interval in milliseconds (default: 1000)
- `UNIT09_WORKER_MAX_CONCURRENCY` — maximum number of concurrent jobs (default: 4)

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
