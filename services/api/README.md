# @unit09/api

HTTP API service for the Unit09 on-chain AI raccoon project.

This service exposes a small REST API that allows external tools to:

- Query observed repositories and generated modules
- Inspect forks of the Unit09 lifeform
- Enqueue and track core-engine pipeline runs
- Retrieve aggregate statistics about Unit09 activity

## Endpoints

Base path: `/api`

- `GET /health` — simple liveness probe
- `GET /repos` — list repositories tracked by Unit09
- `GET /repos/:repoKey/modules` — list modules by repository
- `GET /modules/by-repo/:repoKey` — alias for modules by repository
- `GET /forks` — list Unit09 forks
- `POST /pipeline/jobs` — enqueue a new pipeline run
- `GET /pipeline/jobs` — list known jobs
- `GET /pipeline/jobs/:jobId` — get job status
- `GET /stats` — global stats

## Configuration

The service can be configured using environment variables:

- `UNIT09_API_PORT` — HTTP port (default: 8080)
- `UNIT09_API_LOG_LEVEL` — log level (`silent`, `error`, `warn`, `info`, `debug`)
- `UNIT09_API_CORS` — set to `false` to disable CORS
- `UNIT09_API_KEY` — optional API key; if set, clients must send `x-api-key` header
- `UNIT09_PROGRAM_ID` — Unit09 program public key
- `SOLANA_RPC_URL` — Solana RPC endpoint
- `UNIT09_ENGINE_ROOT_DIR` — root directory for the core-engine

## Development

```bash
npm install
npm run dev
```

The dev server will start on `http://localhost:8080` by default.

## Production build

```bash
npm run build
npm start
```

You can also build and run the Docker image:

```bash
docker build -t unit09-api .
docker run --rm -p 8080:8080 unit09-api
```
