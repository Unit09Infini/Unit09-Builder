/**
 * ============================================================================
 * Unit09 – Explorer Verification Helper
 * Path: contracts/unit09-program/scripts/verify-on-explorer.ts
 *
 * This script helps you:
 *   1) Resolve the Unit09 program id (from CLI, Anchor.toml, or IDL)
 *   2) Optionally query an RPC endpoint to verify that:
 *        - The account exists
 *        - The account is executable (a deployed program)
 *   3) Print ready-to-click explorer URLs for:
 *        - Official Solana Explorer
 *        - Solscan
 *        - SolanaFM
 *   4) Summarize basic program account information (lamports, owner)
 *
 * Usage (from repo root):
 *   npx ts-node contracts/unit09-program/scripts/verify-on-explorer.ts
 *
 * Flags:
 *   --program-id=<ID>       Override program id (base58)
 *   --cluster=<name>        Cluster: localnet | devnet | mainnet | custom
 *   --rpc-url=<URL>         Custom RPC URL (implies cluster=custom)
 *   --skip-rpc              Do not query RPC, only print explorer URLs
 *   --anchor-profile=<P>    Anchor profile to read from Anchor.toml
 *   --idl-path=<PATH>       Custom IDL path (default: contracts/idl/unit09_program.json)
 *   --quiet                 Reduce log output
 *
 * Examples:
 *   npx ts-node contracts/unit09-program/scripts/verify-on-explorer.ts
 *   npx ts-node contracts/unit09-program/scripts/verify-on-explorer.ts --cluster=devnet
 *   npx ts-node contracts/unit09-program/scripts/verify-on-explorer.ts --program-id=YourProgramId
 *   npx ts-node contracts/unit09-program/scripts/verify-on-explorer.ts --rpc-url=https://api.mainnet-beta.solana.com
 *
 * Requirements:
 *   - Node.js >= 18 (for built-in fetch)
 *   - ts-node if running TypeScript directly
 *   - Anchor.toml and/or IDL present if you rely on automatic detection
 * ============================================================================
 */

import * as fs from "node:fs";
import * as path from "node:path";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const ROOT_DIR = path.resolve(__dirname, "..", ".."); // contracts/
const PROGRAM_DIR = path.resolve(ROOT_DIR, "unit09-program");
const ANCHOR_TOML_PATH = path.resolve(PROGRAM_DIR, "Anchor.toml");
const DEFAULT_IDL_PATH = path.resolve(ROOT_DIR, "idl", "unit09_program.json");

const DEFAULT_PROGRAM_NAME = "unit09_program";

type ClusterName = "localnet" | "devnet" | "mainnet" | "custom";

// ---------------------------------------------------------------------------
// CLI Options
// ---------------------------------------------------------------------------

interface VerifyCliOptions {
  programId: string | null;
  cluster: ClusterName;
  rpcUrl: string | null;
  skipRpc: boolean;
  anchorProfile: string | null;
  idlPath: string | null;
  quiet: boolean;
}

function parseCliArgs(argv: string[]): VerifyCliOptions {
  let programId: string | null = null;
  let cluster: ClusterName = "devnet";
  let rpcUrl: string | null = null;
  let skipRpc = false;
  let anchorProfile: string | null = null;
  let idlPath: string | null = null;
  let quiet = false;

  for (const arg of argv) {
    if (arg.startsWith("--program-id=")) {
      programId = arg.split("=")[1]?.trim() || null;
    } else if (arg.startsWith("--cluster=")) {
      const value = arg.split("=")[1]?.trim() || "";
      if (value === "localnet" || value === "devnet" || value === "mainnet" || value === "custom") {
        cluster = value;
      } else {
        throw new Error(`Invalid cluster value: ${value}. Expected localnet | devnet | mainnet | custom.`);
      }
    } else if (arg.startsWith("--rpc-url=")) {
      rpcUrl = arg.split("=")[1]?.trim() || null;
      cluster = "custom";
    } else if (arg === "--skip-rpc") {
      skipRpc = true;
    } else if (arg.startsWith("--anchor-profile=")) {
      anchorProfile = arg.split("=")[1]?.trim() || null;
    } else if (arg.startsWith("--idl-path=")) {
      idlPath = arg.split("=")[1]?.trim() || null;
    } else if (arg === "--quiet") {
      quiet = true;
    }
  }

  return {
    programId,
    cluster,
    rpcUrl,
    skipRpc,
    anchorProfile,
    idlPath,
    quiet,
  };
}

// ---------------------------------------------------------------------------
// Logging helpers
// ---------------------------------------------------------------------------

function log(message: string, quiet: boolean): void {
  if (!quiet) {
    // eslint-disable-next-line no-console
    console.log(`[verify-on-explorer] ${message}`);
  }
}

function logWarn(message: string, quiet: boolean): void {
  if (!quiet) {
    // eslint-disable-next-line no-console
    console.warn(`[verify-on-explorer] WARN: ${message}`);
  }
}

function logError(message: string): void {
  // eslint-disable-next-line no-console
  console.error(`[verify-on-explorer] ERROR: ${message}`);
}

// ---------------------------------------------------------------------------
// Program id resolution
// ---------------------------------------------------------------------------

/**
 * Try to parse program id for a given Anchor profile from Anchor.toml.
 * The logic is:
 *   - If profile is provided, read `[programs.<profile>]`
 *   - Else, try `[programs.devnet]`, then `[programs.localnet]`, then `[programs.mainnet]`
 */
function loadProgramIdFromAnchorToml(
  anchorTomlPath: string,
  profile: string | null,
  quiet: boolean
): string | null {
  if (!fs.existsSync(anchorTomlPath)) {
    logWarn(`Anchor.toml not found at ${anchorTomlPath}`, quiet);
    return null;
  }

  const content = fs.readFileSync(anchorTomlPath, "utf8");
  const lines = content.split(/\r?\n/);

  const candidateProfiles: string[] = [];
  if (profile) candidateProfiles.push(profile);
  candidateProfiles.push("devnet", "localnet", "mainnet");

  for (const candidate of candidateProfiles) {
    const header = `[programs.${candidate}]`;
    let inSection = false;

    for (const line of lines) {
      const trimmed = line.trim();

      if (trimmed.startsWith("[programs.")) {
        inSection = trimmed === header;
      }

      if (inSection && trimmed.startsWith(`${DEFAULT_PROGRAM_NAME}`)) {
        const parts = trimmed.split("=");
        if (parts.length >= 2) {
          const value = parts[1].trim();
          const match = value.match(/"([^"]+)"/);
          if (match && match[1]) {
            log(`Found program id in Anchor.toml [programs.${candidate}]`, quiet);
            return match[1];
          }
        }
      }
    }
  }

  logWarn("No matching program id found in Anchor.toml", quiet);
  return null;
}

/**
 * Try to load program id from IDL metadata.
 */
function loadProgramIdFromIdl(idlPath: string, quiet: boolean): string | null {
  if (!fs.existsSync(idlPath)) {
    logWarn(`IDL not found at ${idlPath}`, quiet);
    return null;
  }

  try {
    const raw = fs.readFileSync(idlPath, "utf8");
    const json = JSON.parse(raw) as { metadata?: { address?: string }; name?: string };

    if (json.metadata && json.metadata.address) {
      log(`Found program id in IDL metadata: ${json.metadata.address}`, quiet);
      return json.metadata.address;
    }

    logWarn("IDL metadata does not contain an address field", quiet);
    return null;
  } catch (err: any) {
    logWarn(`Failed to parse IDL JSON at ${idlPath}: ${err?.message || String(err)}`, quiet);
    return null;
  }
}

/**
 * Resolve the program id using:
 *   1) CLI override
 *   2) Anchor.toml
 *   3) IDL metadata
 */
function resolveProgramId(opts: VerifyCliOptions): string {
  if (opts.programId) {
    log(`Using program id provided via CLI: ${opts.programId}`, opts.quiet);
    return opts.programId;
  }

  const fromToml = loadProgramIdFromAnchorToml(ANCHOR_TOML_PATH, opts.anchorProfile, opts.quiet);
  if (fromToml) return fromToml;

  const idlPath = opts.idlPath || DEFAULT_IDL_PATH;
  const fromIdl = loadProgramIdFromIdl(idlPath, opts.quiet);
  if (fromIdl) return fromIdl;

  throw new Error(
    "Unable to resolve program id. Please provide --program-id or ensure Anchor.toml/IDL contains metadata.address."
  );
}

// ---------------------------------------------------------------------------
// RPC helpers
// ---------------------------------------------------------------------------

interface RpcRequestBody {
  jsonrpc: string;
  id: string;
  method: string;
  params?: unknown[];
}

interface RpcResponse<T> {
  jsonrpc: string;
  id: string;
  result?: T;
  error?: {
    code: number;
    message: string;
    data?: unknown;
  };
}

interface RpcAccountInfo {
  lamports: number;
  owner: string;
  executable: boolean;
  rentEpoch: number;
  data: [string, string]; // base64 encoded
}

/**
 * Choose an RPC URL based on cluster and optional override.
 */
function resolveRpcUrl(opts: VerifyCliOptions): string {
  if (opts.rpcUrl) return opts.rpcUrl;

  switch (opts.cluster) {
    case "localnet":
      return "http://127.0.0.1:8899";
    case "devnet":
      return "https://api.devnet.solana.com";
    case "mainnet":
      return "https://api.mainnet-beta.solana.com";
    case "custom":
      throw new Error(
        "Cluster is set to custom but no --rpc-url provided. Please pass --rpc-url=<URL> or choose devnet/mainnet/localnet."
      );
    default:
      return "https://api.devnet.solana.com";
  }
}

async function rpcGetAccountInfo(
  rpcUrl: string,
  account: string,
  quiet: boolean
): Promise<RpcAccountInfo | null> {
  const body: RpcRequestBody = {
    jsonrpc: "2.0",
    id: "unit09-verify",
    method: "getAccountInfo",
    params: [account, { encoding: "base64" }],
  };

  const response = await fetch(rpcUrl, {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(body),
  });

  if (!response.ok) {
    throw new Error(`RPC HTTP error: ${response.status} ${response.statusText}`);
  }

  const json = (await response.json()) as RpcResponse<{ value: RpcAccountInfo | null }>;
  if (json.error) {
    logWarn(
      `RPC responded with error: ${json.error.code} ${json.error.message} (${JSON.stringify(
        json.error.data ?? {}
      )})`,
      quiet
    );
    return null;
  }

  if (!json.result || !json.result.value) {
    logWarn("Account not found on RPC (result.value is null).", quiet);
    return null;
  }

  return json.result.value;
}

// ---------------------------------------------------------------------------
// Explorer URL helpers
// ---------------------------------------------------------------------------

interface ExplorerUrls {
  solanaExplorer: string;
  solscan: string;
  solanaFm: string;
}

/**
 * Map cluster name into the string used by Solana explorer query.
 */
function solanaExplorerClusterParam(cluster: ClusterName): string | null {
  switch (cluster) {
    case "localnet":
      return "custom"; // but explorer does not support local validators; treat as custom
    case "devnet":
      return "devnet";
    case "mainnet":
      return null; // mainnet is default
    case "custom":
      return "custom";
    default:
      return null;
  }
}

/**
 * Construct explorer URLs for a given program id and cluster.
 */
function buildExplorerUrls(programId: string, cluster: ClusterName): ExplorerUrls {
  const clusterParam = solanaExplorerClusterParam(cluster);

  const solanaExplorerBase = "https://explorer.solana.com/address";
  const solscanBase = "https://solscan.io/account";
  const solanaFmBase = "https://solana.fm/address";

  const solanaExplorerUrl =
    clusterParam && clusterParam !== "custom"
      ? `${solanaExplorerBase}/${programId}?cluster=${clusterParam}`
      : `${solanaExplorerBase}/${programId}`;

  const solscanCluster =
    cluster === "devnet" ? "?cluster=devnet" : cluster === "mainnet" ? "" : ""; // solscan does not support localnet/custom directly

  const solscanUrl = `${solscanBase}/${programId}${solscanCluster}`;

  const solanaFmCluster =
    cluster === "devnet"
      ? "?cluster=devnet-solana"
      : cluster === "mainnet"
      ? "?cluster=mainnet-solana"
      : ""; // localnet/custom not supported

  const solanaFmUrl = `${solanaFmBase}/${programId}${solanaFmCluster}`;

  return {
    solanaExplorer: solanaExplorerUrl,
    solscan: solscanUrl,
    solanaFm: solanaFmUrl,
  };
}

// ---------------------------------------------------------------------------
// Pretty printing
// ---------------------------------------------------------------------------

function printSummary(
  programId: string,
  cluster: ClusterName,
  rpcUrl: string | null,
  accountInfo: RpcAccountInfo | null,
  urls: ExplorerUrls,
  quiet: boolean
): void {
  const header = "================ Unit09 Program Verification ================";
  const footer = "=============================================================";

  if (!quiet) {
    // eslint-disable-next-line no-console
    console.log("");
    // eslint-disable-next-line no-console
    console.log(header);
  }

  // Basic info
  // eslint-disable-next-line no-console
  console.log(`Program Id : ${programId}`);
  // eslint-disable-next-line no-console
  console.log(`Cluster    : ${cluster}`);
  if (rpcUrl) {
    // eslint-disable-next-line no-console
    console.log(`RPC URL    : ${rpcUrl}`);
  }

  // Account info
  if (accountInfo) {
    // eslint-disable-next-line no-console
    console.log("");
    // eslint-disable-next-line no-console
    console.log("On-chain account info:");
    // eslint-disable-next-line no-console
    console.log(`  Lamports   : ${accountInfo.lamports}`);
    // eslint-disable-next-line no-console
    console.log(`  Owner      : ${accountInfo.owner}`);
    // eslint-disable-next-line no-console
    console.log(`  Executable : ${accountInfo.executable ? "yes" : "no"}`);
    // eslint-disable-next-line no-console
    console.log(`  Rent epoch : ${accountInfo.rentEpoch}`);
  } else {
    // eslint-disable-next-line no-console
    console.log("");
    // eslint-disable-next-line no-console
    console.log("On-chain account info: not available or account not found.");
  }

  // Explorer links
  // eslint-disable-next-line no-console
  console.log("");
  // eslint-disable-next-line no-console
  console.log("Explorer links:");
  // eslint-disable-next-line no-console
  console.log(`  Solana Explorer : ${urls.solanaExplorer}`);
  // eslint-disable-next-line no-console
  console.log(`  Solscan         : ${urls.solscan}`);
  // eslint-disable-next-line no-console
  console.log(`  SolanaFM        : ${urls.solanaFm}`);

  if (!quiet) {
    // eslint-disable-next-line no-console
    console.log(footer);
    // eslint-disable-next-line no-console
    console.log("");
  }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main(): Promise<void> {
  const opts = parseCliArgs(process.argv.slice(2));

  try {
    log("Resolving program id...", opts.quiet);
    const programId = resolveProgramId(opts);

    let rpcUrl: string | null = null;
    let accountInfo: RpcAccountInfo | null = null;

    if (!opts.skipRpc) {
      rpcUrl = resolveRpcUrl(opts);
      log(`Using RPC URL: ${rpcUrl}`, opts.quiet);

      try {
        log("Querying getAccountInfo via RPC...", opts.quiet);
        accountInfo = await rpcGetAccountInfo(rpcUrl, programId, opts.quiet);

        if (accountInfo && accountInfo.executable) {
          log("Account exists and is executable. It looks like a deployed program.", opts.quiet);
        } else if (accountInfo && !accountInfo.executable) {
          logWarn(
            "Account exists but is not marked as executable. It may not be a program account.",
            opts.quiet
          );
        } else {
          logWarn("Account not found or no account info returned by RPC.", opts.quiet);
        }
      } catch (err: any) {
        logWarn(`RPC query failed: ${err?.message || String(err)}`, opts.quiet);
      }
    } else {
      log("Skipping RPC queries due to --skip-rpc flag.", opts.quiet);
    }

    const urls = buildExplorerUrls(programId, opts.cluster);

    printSummary(programId, opts.cluster, rpcUrl, accountInfo, urls, opts.quiet);
  } catch (err: any) {
    logError(err?.message || String(err));
    process.exitCode = 1;
  }
}

// Execute when run directly
if (require.main === module) {
  void main();
}
