import { AnchorProvider, Program, Idl } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import type { ConnectionContext } from "./connection";
import type { GlobalMetricsSnapshot } from "@unit09/shared-types";
import {
  fetchConfigAccount
} from "./accounts/configAccount";
import {
  listRepos
} from "./queries/listRepos";
import {
  listModulesByRepo
} from "./queries/listModulesByRepo";
import {
  listForks
} from "./queries/listForks";
import {
  getGlobalStats
} from "./queries/getGlobalStats";

/**
 * Options used when creating a Unit09Client.
 */
export interface Unit09ClientOptions {
  wallet?: AnchorProvider["wallet"];
  idl: Idl;
}

/**
 * Thin wrapper around an Anchor Program instance with a few
 * convenience methods that are commonly used by applications.
 */
export class Unit09Client {
  readonly program: Program;
  readonly provider: AnchorProvider;

  constructor(ctx: ConnectionContext, opts: Unit09ClientOptions) {
    const wallet = opts.wallet ?? new AnchorProvider.Wallet(Keypair.generate());

    const provider = new AnchorProvider(ctx.connection, wallet, {
      commitment: ctx.commitment
    });

    this.provider = provider;
    this.program = new Program(opts.idl, ctx.programId, provider);
  }

  /**
   * Shortcut to retrieve the program id.
   */
  getProgramId() {
    return this.program.programId;
  }

  /**
   * Fetch the single global config account, if it exists.
   */
  async fetchConfig() {
    return fetchConfigAccount(this.program);
  }

  /**
   * List repository accounts with optional filtering.
   * This returns a simplified off-chain projection.
   */
  async fetchRepos(filter?: Parameters<typeof listRepos>[1]) {
    return listRepos(this.program, filter);
  }

  /**
   * List modules belonging to a specific repo.
   */
  async fetchModulesByRepo(filter: Parameters<typeof listModulesByRepo>[1]) {
    return listModulesByRepo(this.program, filter);
  }

  /**
   * List forks with optional filtering.
   */
  async fetchForks(filter?: Parameters<typeof listForks>[1]) {
    return listForks(this.program, filter);
  }

  /**
   * Fetch global metrics snapshot, if the metrics account is present.
   */
  async fetchGlobalStats(): Promise<GlobalMetricsSnapshot | null> {
    return getGlobalStats(this.program);
  }
}

/**
 * Helper to construct a Unit09Client in one call.
 */
export function createUnit09Client(ctx: ConnectionContext, opts: Unit09ClientOptions): Unit09Client {
  return new Unit09Client(ctx, opts);
}
