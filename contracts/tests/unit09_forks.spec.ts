/**
 * ============================================================================
 * Unit09 – Fork Integration Tests
 * Path: contracts/unit09-program/tests/unit09_forks.spec.ts
 *
 * This file focuses on fork-related behavior:
 *   - Creating root and child forks
 *   - Preventing duplicate fork creation for the same fork key
 *   - Updating fork state (label, metadata, tags, active flag)
 *   - Verifying lifecycle and metrics react to fork-level activity
 *
 * It relies on helpers from:
 *   - tests/helpers/provider.ts
 *   - tests/helpers/accounts.ts
 *   - tests/helpers/builders.ts
 *   - tests/helpers/assertions.ts
 *
 * All content is written in English only.
 * ============================================================================
 */

import { SystemProgram, PublicKey, Keypair } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

import { createUnit09TestContext } from "./helpers/provider";
import {
  deriveAllCorePdasFromProgram,
  getForkPda,
} from "./helpers/accounts";
import {
  BuildInitializeArgsOptions,
  buildCreateForkArgs,
  buildUpdateForkStateArgs,
  createForkOnChain,
  initializeUnit09OnChain,
} from "./helpers/builders";
import {
  assertFork,
  assertMetrics,
  assertLifecycle,
  expectDefined,
} from "./helpers/assertions";

// Increase timeout for CI or slow RPCs
jest.setTimeout(120_000);

// Shared test context
const ctx = createUnit09TestContext();

describe("unit09_program – forks", () => {
  const initOptions: BuildInitializeArgsOptions = {
    feeBps: 250,
    maxModulesPerRepo: 256,
  };

  // Canonical root fork used across tests
  let canonicalRootForkKey: PublicKey;
  let canonicalRootForkTx: string;

  beforeAll(async () => {
    // Ensure payer is funded
    await ctx.ensurePayerHasFunds(2 * 1_000_000_000); // 2 SOL

    const program = ctx.program;
    const { config } = deriveAllCorePdasFromProgram(program);

    // Initialize program if needed
    let needsInit = false;
    try {
      await program.account.config.fetch(config);
    } catch {
      needsInit = true;
    }

    if (needsInit) {
      await initializeUnit09OnChain(ctx, initOptions);
    }

    // Create a canonical root fork (representing the initial Unit09 raccoon)
    const rootResult = await createForkOnChain(ctx, {
      label: "unit09-root-fork",
      isRoot: true,
      depth: 0,
      tags: "unit09,fork,root",
    });

    canonicalRootForkKey = rootResult.forkKey;
    canonicalRootForkTx = rootResult.tx;
  });

  it("creates a root fork with expected metadata and depth", async () => {
    const program = ctx.program;
    const programId = program.programId;

    const forkKey = Keypair.generate().publicKey;

    const args = buildCreateForkArgs({
      forkKey,
      label: "unit09-fork-root-test",
      metadataUri: "https://unit09.org/meta/fork/root-test.json",
      tags: "unit09,fork,root-test",
      isRoot: true,
      depth: 0,
    });

    const pdas = deriveAllCorePdasFromProgram(program, { forkKey });

    const tx = await program.methods
      .createFork(args)
      .accounts({
        config: pdas.config,
        lifecycle: pdas.lifecycle,
        fork: pdas.fork,
        owner: ctx.wallet.publicKey,
        payer: ctx.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    expect(tx).toBeTruthy();

    const forkAcc = await program.account.fork.fetch(pdas.fork);
    const lifecycleAcc = await program.account.lifecycle.fetch(pdas.lifecycle);

    // Fork checks
    assertFork(
      { pubkey: pdas.fork, data: forkAcc },
      {
        label: args.label,
        isActive: true,
        depth: args.depth ?? undefined,
      }
    );

    expect(forkAcc.isRoot).toBe(true);
    expect(forkAcc.parent).toBeNull();
    expectDefined(forkAcc.metadataUri, "fork.metadataUri");
    expect(typeof forkAcc.createdAt).toBe("number");
    expect(forkAcc.createdAt).toBeGreaterThan(0);

    // Lifecycle sanity: lastActivityAt should be non-zero
    assertLifecycle(
      { pubkey: pdas.lifecycle, data: lifecycleAcc },
      {
        createdAt: lifecycleAcc.createdAt,
        lastActivityAt: lifecycleAcc.lastActivityAt,
      }
    );
  });

  it("creates a child fork that references a parent and increments depth", async () => {
    const program = ctx.program;

    const parentForkKey = canonicalRootForkKey;
    const childForkKey = Keypair.generate().publicKey;

    const args = buildCreateForkArgs({
      forkKey: childForkKey,
      parent: parentForkKey,
      label: "unit09-child-fork",
      metadataUri: "https://unit09.org/meta/fork/child.json",
      tags: "unit09,fork,child",
      isRoot: false,
      depth: 1,
    });

    const pdas = deriveAllCorePdasFromProgram(program, { forkKey: childForkKey });

    const tx = await program.methods
      .createFork(args)
      .accounts({
        config: pdas.config,
        lifecycle: pdas.lifecycle,
        fork: pdas.fork,
        owner: ctx.wallet.publicKey,
        payer: ctx.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    expect(tx).toBeTruthy();

    const forkAcc = await program.account.fork.fetch(pdas.fork);

    assertFork(
      { pubkey: pdas.fork, data: forkAcc },
      {
        label: args.label,
        depth: args.depth ?? undefined,
        isActive: true,
      }
    );

    expect(forkAcc.isRoot).toBe(false);
    expect(forkAcc.parent).toEqual(parentForkKey.toBase58());
  });

  it("stores and retrieves the canonical root fork created in setup", async () => {
    const program = ctx.program;
    const programId = program.programId;

    const forkPda = getForkPda(programId, canonicalRootForkKey);
    const forkAcc = await program.account.fork.fetch(forkPda);

    assertFork(
      { pubkey: forkPda, data: forkAcc },
      {
        label: "unit09-root-fork",
        isActive: true,
        depth: 0,
      }
    );

    expect(forkAcc.isRoot).toBe(true);
    expect(forkAcc.parent).toBeNull();
  });

  it("prevents creating the same forkKey twice", async () => {
    const program = ctx.program;

    const forkKey = Keypair.generate().publicKey;

    const firstArgs = buildCreateForkArgs({
      forkKey,
      label: "unit09-fork-dup-1",
      isRoot: true,
      depth: 0,
    });

    const pdas = deriveAllCorePdasFromProgram(program, { forkKey });

    // First creation should succeed
    const tx1 = await program.methods
      .createFork(firstArgs)
      .accounts({
        config: pdas.config,
        lifecycle: pdas.lifecycle,
        fork: pdas.fork,
        owner: ctx.wallet.publicKey,
        payer: ctx.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    expect(tx1).toBeTruthy();

    // Second creation with same forkKey should fail
    const secondArgs = buildCreateForkArgs({
      forkKey,
      label: "unit09-fork-dup-2",
      isRoot: false,
      depth: 1,
    });

    await expect(
      program.methods
        .createFork(secondArgs)
        .accounts({
          config: pdas.config,
          lifecycle: pdas.lifecycle,
          fork: pdas.fork,
          owner: ctx.wallet.publicKey,
          payer: ctx.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc()
    ).rejects.toThrow();
  });

  it("updates fork state metadata via updateForkState", async () => {
    const program = ctx.program;

    const forkKey = Keypair.generate().publicKey;
    const createArgs = buildCreateForkArgs({
      forkKey,
      label: "unit09-fork-updatable",
      metadataUri: "https://unit09.org/meta/fork/updatable.json",
      tags: "unit09,fork,updatable",
      isRoot: true,
      depth: 0,
    });

    const pdas = deriveAllCorePdasFromProgram(program, { forkKey });

    await program.methods
      .createFork(createArgs)
      .accounts({
        config: pdas.config,
        lifecycle: pdas.lifecycle,
        fork: pdas.fork,
        owner: ctx.wallet.publicKey,
        payer: ctx.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const updateArgs = buildUpdateForkStateArgs({
      label: "unit09-fork-updated",
      metadataUri: "https://unit09.org/meta/fork/updated.json",
      tags: "unit09,fork,updated",
      isActive: false,
    });

    const tx = await program.methods
      .updateForkState(updateArgs)
      .accounts({
        config: pdas.config,
        fork: pdas.fork,
        owner: ctx.wallet.publicKey,
      })
      .rpc();

    expect(tx).toBeTruthy();

    const forkAcc = await program.account.fork.fetch(pdas.fork);

    assertFork(
      { pubkey: pdas.fork, data: forkAcc },
      {
        label: updateArgs.label ?? undefined,
        isActive: updateArgs.isActive ?? undefined,
      }
    );

    expect(forkAcc.metadataUri).toEqual(updateArgs.metadataUri);
    expect(forkAcc.tags).toEqual(updateArgs.tags);
  });

  it("supports partial fork updates where null fields are ignored", async () => {
    const program = ctx.program;

    const forkKey = Keypair.generate().publicKey;
    const initialLabel = "unit09-fork-partial";
    const initialMeta = "https://unit09.org/meta/fork/partial.json";
    const initialTags = "unit09,fork,partial";

    const createArgs = buildCreateForkArgs({
      forkKey,
      label: initialLabel,
      metadataUri: initialMeta,
      tags: initialTags,
      isRoot: true,
      depth: 0,
    });

    const pdas = deriveAllCorePdasFromProgram(program, { forkKey });

    await program.methods
      .createFork(createArgs)
      .accounts({
        config: pdas.config,
        lifecycle: pdas.lifecycle,
        fork: pdas.fork,
        owner: ctx.wallet.publicKey,
        payer: ctx.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const updateArgs = buildUpdateForkStateArgs({
      label: null,
      metadataUri: null,
      // Only change tags and active flag
      tags: "unit09,fork,partial-updated",
      isActive: true,
    });

    await program.methods
      .updateForkState(updateArgs)
      .accounts({
        config: pdas.config,
        fork: pdas.fork,
        owner: ctx.wallet.publicKey,
      })
      .rpc();

    const forkAcc = await program.account.fork.fetch(pdas.fork);

    // Label and metadataUri should remain unchanged
    assertFork(
      { pubkey: pdas.fork, data: forkAcc },
      {
        label: initialLabel,
        isActive: true,
      }
    );

    expect(forkAcc.metadataUri).toEqual(initialMeta);
    expect(forkAcc.tags).toContain("partial-updated");
  });

  it("touches lifecycle timestamps when creating forks", async () => {
    const program = ctx.program;

    const pdasBefore = deriveAllCorePdasFromProgram(program);
    const lifecycleBefore = await program.account.lifecycle.fetch(pdasBefore.lifecycle);

    const forkKey = Keypair.generate().publicKey;
    const createArgs = buildCreateForkArgs({
      forkKey,
      label: "unit09-fork-lifecycle",
      isRoot: true,
      depth: 0,
      tags: "unit09,fork,lifecycle",
    });

    const pdasAfter = deriveAllCorePdasFromProgram(program, { forkKey });

    const tx = await program.methods
      .createFork(createArgs)
      .accounts({
        config: pdasAfter.config,
        lifecycle: pdasAfter.lifecycle,
        fork: pdasAfter.fork,
        owner: ctx.wallet.publicKey,
        payer: ctx.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    expect(tx).toBeTruthy();

    const lifecycleAfter = await program.account.lifecycle.fetch(pdasAfter.lifecycle);

    // lastActivityAt should be updated or at least not older than before
    expect(lifecycleAfter.lastActivityAt).toBeGreaterThanOrEqual(
      lifecycleBefore.lastActivityAt
    );

    assertLifecycle(
      { pubkey: pdasAfter.lifecycle, data: lifecycleAfter },
      {
        createdAt: lifecycleAfter.createdAt,
        lastActivityAt: lifecycleAfter.lastActivityAt,
      }
    );
  });

  it("keeps metrics consistent with fork creation", async () => {
    const program = ctx.program;

    const pdasMetricsBefore = deriveAllCorePdasFromProgram(program);
    const metricsBefore = await program.account.metrics.fetch(pdasMetricsBefore.metrics);

    // Create a few forks to bump metrics
    for (let i = 0; i < 3; i++) {
      const forkKey = Keypair.generate().publicKey;
      const createArgs = buildCreateForkArgs({
        forkKey,
        label: `unit09-metrics-fork-${i}`,
        isRoot: true,
        depth: 0,
      });

      const pdas = deriveAllCorePdasFromProgram(program, { forkKey });

      await program.methods
        .createFork(createArgs)
        .accounts({
          config: pdas.config,
          lifecycle: pdas.lifecycle,
          fork: pdas.fork,
          owner: ctx.wallet.publicKey,
          payer: ctx.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
    }

    const pdasMetricsAfter = deriveAllCorePdasFromProgram(program);
    const metricsAfter = await program.account.metrics.fetch(pdasMetricsAfter.metrics);

    expect(metricsAfter.totalForks.gt(metricsBefore.totalForks)).toBe(true);

    assertMetrics(
      { pubkey: pdasMetricsAfter.metrics, data: metricsAfter },
      {
        totalForks: metricsAfter.totalForks as unknown as bigint,
      }
    );
  });

  it("exposes the creation transaction for the canonical root fork", () => {
    expect(canonicalRootForkTx).toBeTruthy();
    if (canonicalRootForkTx) {
      // eslint-disable-next-line no-console
      console.log("Unit09 canonical root fork tx:", canonicalRootForkTx);
    }
  });
});
