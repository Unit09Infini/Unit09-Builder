
![1500x500](https://github.com/user-attachments/assets/8c3435ab-67d7-4a06-b7c7-fd547fd5d9dd)

# UNT909 — On-chain AI Raccoon for Solana Builders

UNT909 is a story-driven on-chain AI raccoon that consumes Solana code,
turns it into reusable modules, evolves through forks, and helps anyone
build new worlds on Solana without needing to understand every low-level
detail.

- Website: http://unt909x.fun/
- X (Twitter): https://x.com/UNT909GROW

> A narrative-driven AI lifeform that lives on-chain, observes real code,
> decomposes it into modules, and keeps evolving as the community builds
> and forks.

---

## Table of contents

- [Concept](#concept)
- [Core features](#core-features)
- [Architecture overview](#architecture-overview)
- [Repository layout](#repository-layout)
- [Getting started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Install dependencies](#install-dependencies)
  - [Build and test the on-chain program](#build-and-test-the-on-chain-program)
  - [Run the local demo stack](#run-the-local-demo-stack)
- [Configuration](#configuration)
- [Using the CLI](#using-the-cli)
- [SDK usage](#sdk-usage)
- [Engine and pipeline](#engine-and-pipeline)
- [API and services](#api-and-services)
- [Dashboard and docs site](#dashboard-and-docs-site)
- [Examples](#examples)
- [Deployment](#deployment)
- [Security](#security)
- [Contributing and governance](#contributing-and-governance)
- [License](#license)

---

## Concept

UNT909 is designed as a long-lived AI-powered companion for Solana
developers. It behaves like a modularization engine with a personality:

- It **observes** real Solana projects and repositories.
- It **analyzes** and **decomposes** code into focused, reusable modules.
- It **syncs** those modules on-chain as structured, versioned entities.
- It **evolves** through forks created by users, preserving lineage and
  metrics across variants.
- It **exposes** modules through an SDK, API, CLI, and gallery so anyone
  can assemble new projects quickly.

You can think of Unit09 as an on-chain librarian and raccoon engineer
that rearranges complex systems into small, runnable building blocks.

---

## Core features

- **On-chain module registry**  
  A Solana program that stores repositories, modules, module versions,
  forks, metrics, and lifecycle information.

- **Code observation and analysis**  
  A core engine that crawls repositories, parses codebases, and builds
  structural graphs for Rust / Anchor and TypeScript projects, with room
  to extend to more languages.

- **Automated module generation**  
  Generators that scaffold modules, instruction templates, deployment
  scripts, and frontend stubs based on analyzed code.

- **Fork-aware evolution**  
  A forking model that treats each fork as a Unit09 variant with its own
  metrics and evolution history, synced back on-chain.

- **Unified tooling**  
  TypeScript SDK, CLI, API services, local demo stack, dashboard, and
  documentation site built around the same primitives.

---

## Architecture overview

At a high level, Unit09 consists of:

1. **On-chain program (contracts)**  
   Anchor-based Solana program that holds the canonical state for
   repositories, modules, module versions, forks, lifecycle, and metrics.

2. **Core engine**  
   A TypeScript engine that runs the full pipeline:
   observe → analyze → decompose → generate → validate → sync-on-chain.

3. **Services**  
   - API service that exposes HTTP endpoints for interacting with Unit09.
   - Worker service that processes background jobs (observations, analysis,
     syncing).
   - Scheduler that triggers periodic jobs.

4. **Tooling**  
   - SDK package for interacting with the program and services.
   - CLI for local workflows.
   - Testing utilities for localnet and integration tests.

5. **Apps and docs**  
   - Dashboard to browse repositories, modules, forks, and metrics.
   - Documentation site and markdown docs in the `docs/` folder.

See `docs/architecture.md`, `docs/onchain-design.md` and `docs/engine-design.md`
for a deeper technical breakdown.

---

## Repository layout

This is a monorepo. At a glance:

```text
contracts/
  UNT909-program/         # Anchor Solana program

packages/
  shared-types/           # Shared TypeScript types
  sdk/                    # Unit09 TypeScript SDK
  core-engine/            # Analysis and generation engine
  cli-kit/                # Shared CLI utilities
  testing-utils/          # Testing helpers

services/
  api/                    # HTTP API service
  worker/                 # Background job processor
  scheduler/              # Job scheduler

cli/
  ...                     # unit09 CLI entrypoint and commands

apps/
  dashboard/              # Dashboard UI (Next.js)
  docs-site/              # Documentation site (Next.js)

examples/
  simple-anchor-project/  # Minimal example project
  unit09-local-demo/      # Local demo stack (Docker)
  module-gallery/         # Example modules and templates

infra/
  docker/                 # Dockerfiles for services
  k8s/                    # Kubernetes manifests
  terraform/              # Optional infrastructure-as-code
  monitoring/             # Prometheus / Grafana configs

config/
  default.yaml            # Shared default configuration
  development.yaml
  production.yaml
  schema.json

docs/
  ...                     # Markdown documentation

LICENSE
README.md
CHANGELOG.md
CONTRIBUTING.md
CODE_OF_CONDUCT.md
SECURITY.md
GOVERNANCE.md
```

---

## Getting started

### Prerequisites

You should have:

- Node.js 20+
- pnpm or npm (examples use pnpm)
- Rust stable toolchain
- Solana CLI
- Anchor CLI
- Docker (for local demo stack)
- Git

Verify your tools:

```bash
node -v
pnpm -v || npm -v
rustc -vV
solana --version
anchor --version
docker --version
```

Clone the repository (example URLs, adjust to your actual organization):

```bash
git clone https://github.com/unit09-labs/unit09.git
cd unit09
```

### Install dependencies

From the monorepo root:

```bash
pnpm install
```

Or, if you prefer npm (may be slower and less aligned with this repo):

```bash
npm install
```

### Build and test the on-chain program

```bash
cd contracts/unit09-program
anchor build
anchor test
```

This will compile the Solana program and run the Anchor tests in
`contracts/unit09-program/tests/`.

### Run the local demo stack

The easiest way to see Unit09 in action is the local demo stack:

```bash
cd examples/unit09-local-demo
docker compose up -d

# Optional helper scripts, depending on your setup:
./scripts/run_localnet.sh
pnpm ts-node scripts/seed_demo_data.ts
pnpm ts-node scripts/demo_workflow.ts
```

This stack typically includes:

- Local Solana validator
- Unit09 API service
- Unit09 worker
- Supporting database and queues

See `examples/unit09-local-demo/README.md` for the exact composition and
ports.

---

## Configuration

Configuration is stored in the `config/` directory:

- `default.yaml` — shared defaults
- `development.yaml` — overrides for local development
- `production.yaml` — production overrides
- `schema.json` — JSON Schema for validation

Most services accept an environment variable such as `UNIT09_CONFIG_ENV`
to select the environment profile, for example:

```bash
export UNIT09_CONFIG_ENV=development
pnpm dev:api
```

You can adjust values for:

- Solana cluster and commitment level
- Database and storage
- Security (allowed origins, rate limits)
- Pipeline limits (maximum repository size, job concurrency)
- Metrics and monitoring

See `docs/configuration.md` for full details.

---

## Using the CLI

The CLI is the main entry point for developer workflows around Unit09.

Typical commands include:

- `unit09 init` — initialize configuration in a project
- `unit09 config` — inspect or set configuration values
- `unit09 link-repo <url>` — link a repository to Unit09
- `unit09 run-pipeline <repo>` — run the full pipeline for a repo
- `unit09 list-modules` — list discovered modules
- `unit09 deploy-module <id>` — deploy a selected module
- `unit09 create-fork` — create a Unit09 fork variant
- `unit09 show-stats` — display high-level metrics

For more details, see:

- `cli/README.md`
- `docs/cli-usage.md`

---

## SDK usage

The TypeScript SDK in `packages/sdk` provides a convenient way to interact
with the on-chain program and services from your own applications.

Basic example (simplified):

```ts
import { Unit09Client } from "@unit09/sdk";

async function main() {
  const client = await Unit09Client.init({
    clusterUrl: "http://localhost:8899",
    walletPath: "~/.config/solana/id.json",
  });

  const repos = await client.listRepos();
  console.log("Observed repos:", repos);
}

main().catch(console.error);
```

See `packages/sdk/README.md` and `docs/api-reference.md` for:

- Account helpers
- Instruction builders
- Higher-level queries
- Error handling conventions

---

## Engine and pipeline

The core engine in `packages/core-engine` implements the pipeline that makes
Unit09 feel like a living system:

1. **Observe code** — fetch repository metadata and snapshots.
2. **Detect language** — determine applicable analyzers.
3. **Parse project** — parse sources and configs.
4. **Build code graph** — construct a model of modules, calls, and data.
5. **Decompose modules** — identify reusable units and boundaries.
6. **Generate artifacts** — scaffolds, templates, deployment scripts.
7. **Validate modules** — ensure modules are consistent and runnable.
8. **Sync on-chain** — write module metadata and metrics to the program.

The engine can be triggered via:

- Worker jobs (`services/worker`)
- CLI commands
- Direct calls from other services

For a deep dive, see `docs/engine-design.md` and `docs/workflow.md`.

---

## API and services

The `services/` directory contains the runtime services that expose Unit09
functionality to external clients.

- `services/api`  
  HTTP API server with routes for repositories, modules, forks, stats,
  and triggering pipeline operations.

- `services/worker`  
  Background job processing including observations, analysis, and
  sync-on-chain jobs.

- `services/scheduler`  
  Schedules periodic jobs such as recurring observations and metrics sync.

Typical local API routes might include:

- `GET /health`
- `GET /repos`
- `GET /repos/:id/modules`
- `GET /modules/:id`
- `POST /repos`
- `POST /pipeline/run`

See `docs/api-reference.md` and `services/api/README.md` for detailed
routes and payloads.

---

## Dashboard and docs site

- `apps/dashboard`  
  Next.js dashboard for:

  - Viewing observed repositories
  - Exploring modules and forks
  - Inspecting metrics and timelines

- `apps/docs-site`  
  Next.js documentation site that renders the content from `docs/`.

Both apps can typically be started via:

```bash
pnpm dev:dashboard
pnpm dev:docs
```

See each app’s README for exact scripts and environment requirements.

---

## Examples

The `examples/` directory contains:

- **simple-anchor-project/**  
  A minimal Anchor project used as an example input for Unit09.

- **unit09-local-demo/**  
  A full local demo deployment via Docker Compose.

- **module-gallery/**  
  A curated set of example modules such as token vesting, basic mint,
  access control, and more.

These examples are a good starting point for understanding how Unit09
parses, decomposes, and represents codebases.

---

## Deployment

Unit09 can be deployed in several ways depending on your needs.

- **Local development**  
  - Docker Compose for localnet and services.
  - Direct `pnpm dev` commands for apps and API.

- **Staging / production**  
  - Docker images in `infra/docker/`.
  - Kubernetes manifests in `infra/k8s/`.
  - Optional Terraform definitions in `infra/terraform/`.
  - Monitoring with Prometheus and Grafana via `infra/monitoring/`.

Refer to `docs/deployment-guide.md` for:

- Recommended topologies
- Secrets and configuration
- Scaling and resource considerations
- Monitoring and alerting

---

## Security

Security is taken seriously in the Unit09 project.

Please review:

- `SECURITY.md` for:
  - Supported versions
  - How to report vulnerabilities
  - Responsible disclosure process
- `docs/onchain-design.md` for trust boundaries and account model
- `docs/architecture.md` for data flows and integration points

Do **not** open public issues for suspected vulnerabilities. Use the
private channels described in `SECURITY.md` instead.

---

## Contributing and governance

Contributions are welcome, whether in the form of:

- Code improvements
- New analyzers and generators
- Documentation and tutorials
- Module gallery additions
- Feedback and design discussions

Please read:

- `CONTRIBUTING.md` — how to set up your environment, coding standards,
  branching and PR workflow.
- `CODE_OF_CONDUCT.md` — expected behavior in all project spaces.
- `GOVERNANCE.md` — how decisions are made and how maintainers are selected.

Bug reports and feature requests can be opened as GitHub issues with
clear steps to reproduce and expected behavior.

---

## License

This project is released under the terms described in the `LICENSE` file
in the root of this repository.

By using, copying, or contributing to UNT909, you agree to the applicable
license terms.
