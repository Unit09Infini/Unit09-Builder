# @unit09/sdk

TypeScript client SDK for the Unit09 on-chain AI raccoon program on Solana.

This package wraps an Anchor `Program` instance with a set of higher-level
helpers for:

- Creating a `ConnectionContext` and `Unit09Client`
- Fetching the global config and metrics accounts
- Listing repositories, modules, and forks as rich domain objects
- Building instruction calls for core Unit09 workflows

The SDK does not hide Anchor. Instead, it aims to make the most common
operations easier while keeping full flexibility for advanced use cases.

## Basic usage

```ts
import { readFileSync } from "fs";
import { createSdkConfig, createConnectionContext, createUnit09Client } from "@unit09/sdk";
import idl from "@unit09/idl/unit09_program.json";

const config = createSdkConfig(
  "https://api.mainnet-beta.solana.com",
  "UNIT09_PROGRAM_PUBKEY_HERE"
);

const ctx = createConnectionContext(config);
const client = createUnit09Client(ctx, { idl });

async function main() {
  const configAccount = await client.fetchConfig();
  console.log("Config:", configAccount);

  const repos = await client.fetchRepos({ activeOnly: true, limit: 10 });
  console.log("Repos:", repos.items);
}

main().catch(console.error);
```

Adjust the program id and RPC endpoint to match your deployment.
