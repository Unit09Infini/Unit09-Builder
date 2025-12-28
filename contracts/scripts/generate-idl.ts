/**
 * ============================================================================
 * Unit09 – IDL Generation Script
 * Path: contracts/unit09-program/scripts/generate-idl.ts
 *
 * This script does the following:
 *  1) Optionally runs `anchor build` for the Unit09 program
 *  2) Copies the generated IDL JSON into `contracts/idl/unit09_program.json`
 *  3) Patches metadata fields (program address, origin, version, name)
 *  4) Optionally validates that the IDL shape roughly matches expectations
 *
 * Usage (from repo root):
 *   npx ts-node contracts/unit09-program/scripts/generate-idl.ts
 *
 * Options:
 *   --skip-build       Skip running `anchor build`
 *   --program-id=...   Override program id (otherwise inferred from Anchor.toml)
 *   --quiet            Reduce console output
 *
 * Requirements:
 *   - Node.js >= 18
 *   - Anchor CLI installed and on PATH (`anchor --version`)
 *   - `ts-node` installed if you run this file directly
 * ============================================================================
 */

import { execFile } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

// ----------------------------------------------------------------------------
// Constants
// ----------------------------------------------------------------------------

const ROOT_DIR = path.resolve(__dirname, "..", ".."); // contracts/unit09-program/..
const PROGRAM_DIR = path.resolve(ROOT_DIR, "unit09-program");
const ANCHOR_TOML_PATH = path.resolve(PROGRAM_DIR, "Anchor.toml");

// Anchor build output
const ANCHOR_TARGET_DIR = path.resolve(PROGRAM_DIR, "target");
const ANCHOR_IDL_DIR = path.resolve(ANCHOR_TARGET_DIR, "idl");

// Project-level IDL directory
const PROJECT_IDL_DIR = path.resolve(ROOT_DIR, "idl");
const PROJECT_IDL_JSON_PATH = path.resolve(PROJECT_IDL_DIR, "unit09_program.json");
const PROJECT_IDL_TYPES_PATH = path.resolve(PROJECT_IDL_DIR, "types.d.ts");

// Program metadata
const DEFAULT_PROGRAM_NAME = "unit09_program";
const DEFAULT_PROGRAM_ADDRESS = "Unit09Program11111111111111111111111111111111";
const DEFAULT_ORIGIN = "unit09.org";

// ----------------------------------------------------------------------------
// CLI Args
// ----------------------------------------------------------------------------

interface CliOptions {
  skipBuild: boolean;
  programId: string | null;
  quiet: boolean;
}

function parseCliArgs(argv: string[]): CliOptions {
  let skipBuild = false;
  let programId: string | null = null;
  let quiet = false;

  for (const arg of argv) {
    if (arg === "--skip-build") {
      skipBuild = true;
    } else if (arg === "--quiet") {
      quiet = true;
    } else if (arg.startsWith("--program-id=")) {
      programId = arg.split("=")[1]?.trim() || null;
    }
  }

  return { skipBuild, programId, quiet };
}

// ----------------------------------------------------------------------------
// Logging helpers
// ----------------------------------------------------------------------------

function log(message: string, quiet: boolean): void {
  if (!quiet) {
    // eslint-disable-next-line no-console
    console.log(`[generate-idl] ${message}`);
  }
}

function logError(message: string): void {
  // eslint-disable-next-line no-console
  console.error(`[generate-idl] ERROR: ${message}`);
}

// ----------------------------------------------------------------------------
// File helpers
// ----------------------------------------------------------------------------

function ensureDirExists(dir: string): void {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

function loadTomlProgramId(anchorTomlPath: string): string | null {
  if (!fs.existsSync(anchorTomlPath)) return null;
  const content = fs.readFileSync(anchorTomlPath, "utf8");

  // Very small regex-based parser to find program id for DEFAULT_PROGRAM_NAME
  // Matches lines like:
  //   [programs.localnet]
  //   unit09_program = "..."
  const lines = content.split(/\r?\n/);
  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.startsWith(`${DEFAULT_PROGRAM_NAME}`)) {
      const parts = trimmed.split("=");
      if (parts.length >= 2) {
        const value = parts[1].trim();
        const match = value.match(/"([^"]+)"/);
        if (match && match[1]) {
          return match[1];
        }
      }
    }
  }
  return null;
}

// ----------------------------------------------------------------------------
// Anchor build
// ----------------------------------------------------------------------------

async function runAnchorBuild(quiet: boolean): Promise<void> {
  log("Running `anchor build` for unit09_program...", quiet);

  try {
    const { stdout, stderr } = await execFileAsync("anchor", ["build"], {
      cwd: PROGRAM_DIR,
      env: process.env,
    });

    if (!quiet && stdout) {
      // eslint-disable-next-line no-console
      console.log(stdout.trim());
    }
    if (stderr) {
      // Anchor often prints to stderr even on success, so do not treat as fatal
      if (!quiet) {
        // eslint-disable-next-line no-console
        console.warn(stderr.trim());
      }
    }
  } catch (err: any) {
    logError(`anchor build failed: ${err?.message || String(err)}`);
    throw err;
  }
}

// ----------------------------------------------------------------------------
// IDL copy + patch
// ----------------------------------------------------------------------------

function findSourceIdlPath(): string {
  // Standard Anchor output:
  //   target/idl/unit09_program.json
  const candidate = path.resolve(ANCHOR_IDL_DIR, `${DEFAULT_PROGRAM_NAME}.json`);
  if (!fs.existsSync(candidate)) {
    throw new Error(`Expected IDL not found at: ${candidate}`);
  }
  return candidate;
}

interface IdlMetadata {
  address?: string;
  origin?: string;
  [key: string]: unknown;
}

interface IdlRoot {
  version?: string;
  name?: string;
  instructions?: unknown[];
  accounts?: unknown[];
  types?: unknown[];
  events?: unknown[];
  errors?: unknown[];
  metadata?: IdlMetadata;
  [key: string]: unknown;
}

function patchIdlMetadata(idl: IdlRoot, programId: string | null): IdlRoot {
  const existingMetadata = idl.metadata ?? {};
  const address = programId || existingMetadata.address || DEFAULT_PROGRAM_ADDRESS;

  return {
    ...idl,
    name: idl.name || DEFAULT_PROGRAM_NAME,
    version: idl.version || "0.1.0",
    metadata: {
      ...existingMetadata,
      address,
      origin: existingMetadata.origin || DEFAULT_ORIGIN,
    },
  };
}

function validateIdlShape(idl: IdlRoot): void {
  if (!idl.name || idl.name !== DEFAULT_PROGRAM_NAME) {
    throw new Error(`IDL "name" must be "${DEFAULT_PROGRAM_NAME}"`);
  }
  if (!Array.isArray(idl.instructions)) {
    throw new Error("IDL is missing instructions array");
  }
  if (!Array.isArray(idl.accounts)) {
    throw new Error("IDL is missing accounts array");
  }
  if (!Array.isArray(idl.errors)) {
    throw new Error("IDL is missing errors array");
  }
}

// ----------------------------------------------------------------------------
// Main
// ----------------------------------------------------------------------------

async function main(): Promise<void> {
  const args = parseCliArgs(process.argv.slice(2));

  try {
    log("Starting IDL generation for Unit09...", args.quiet);

    if (!args.skipBuild) {
      await runAnchorBuild(args.quiet);
    } else {
      log("Skipping `anchor build` because --skip-build was provided.", args.quiet);
    }

    // Locate and load source IDL
    const sourceIdlPath = findSourceIdlPath();
    log(`Found source IDL at: ${sourceIdlPath}`, args.quiet);

    const rawIdl = fs.readFileSync(sourceIdlPath, "utf8");
    let idlJson: IdlRoot;
    try {
      idlJson = JSON.parse(rawIdl) as IdlRoot;
    } catch (err: any) {
      throw new Error(`Failed to parse JSON IDL at ${sourceIdlPath}: ${err?.message || String(err)}`);
    }

    // Infer program id if not provided
    let programId = args.programId;
    if (!programId) {
      programId = loadTomlProgramId(ANCHOR_TOML_PATH) || null;
      if (!programId) {
        log(
          `Program id not found in Anchor.toml; falling back to default: ${DEFAULT_PROGRAM_ADDRESS}`,
          args.quiet
        );
        programId = DEFAULT_PROGRAM_ADDRESS;
      } else {
        log(`Program id inferred from Anchor.toml: ${programId}`, args.quiet);
      }
    } else {
      log(`Program id overridden via CLI: ${programId}`, args.quiet);
    }

    // Patch metadata
    const patchedIdl = patchIdlMetadata(idlJson, programId);

    // Optional validation
    validateIdlShape(patchedIdl);

    // Ensure output directory exists
    ensureDirExists(PROJECT_IDL_DIR);

    // Write patched JSON
    const pretty = JSON.stringify(patchedIdl, null, 2);
    fs.writeFileSync(PROJECT_IDL_JSON_PATH, pretty, "utf8");
    log(`Patched IDL written to: ${PROJECT_IDL_JSON_PATH}`, args.quiet);

    // The `types.d.ts` in this project is maintained by hand.
    // As a light safety check, we verify that the file exists.
    if (!fs.existsSync(PROJECT_IDL_TYPES_PATH)) {
      log(
        `Warning: types.d.ts not found at ${PROJECT_IDL_TYPES_PATH}. ` +
          `You may want to add or regenerate TypeScript bindings.`,
        args.quiet
      );
    } else {
      log(`Type declarations found at: ${PROJECT_IDL_TYPES_PATH}`, args.quiet);
    }

    log("IDL generation for Unit09 completed successfully.", args.quiet);
  } catch (err: any) {
    logError(err?.message || String(err));
    process.exitCode = 1;
  }
}

// Execute when run directly
if (require.main === module) {
  void main();
}
