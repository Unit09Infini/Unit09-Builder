import type { Program } from "@coral-xyz/anchor";
import type { ModuleFilter, ModuleListPage, ModuleMetadata } from "@unit09/shared-types";
import { PublicKey } from "@solana/web3.js";
import { parseSemanticVersion } from "@unit09/shared-types";

/**
 * List modules belonging to a given repo.
 *
 * This uses a memcmp filter on the repo key field, assuming it is the
 * first field after the discriminator in the `module` account.
 */
export async function listModulesByRepo(
  program: Program,
  filter: ModuleFilter
): Promise<ModuleListPage> {
  if (!filter.repoKey) {
    throw new Error("repoKey is required for listModulesByRepo");
  }

  const repoKeyBytes = new PublicKey(filter.repoKey).toBytes();
  const accounts = await program.account.module.all([
    {
      memcmp: {
        offset: 8, // discriminator
        bytes: Buffer.from(repoKeyBytes).toString("base64")
      }
    }
  ]);

  const items: ModuleMetadata[] = accounts.map((acc: any) => {
    const rawVersion: [number, number, number] = acc.account.currentVersion ?? [0, 1, 0];
    const versionTuple = parseSemanticVersion(
      Array.isArray(rawVersion) ? rawVersion.join(".") : String(rawVersion)
    ) ?? [0, 1, 0];

    return {
      moduleKey: acc.account.moduleKey?.toString?.() ?? acc.publicKey.toString(),
      repoKey: acc.account.repoKey?.toString?.() ?? filter.repoKey!,
      name: acc.account.name,
      kind: acc.account.kind ?? "anchor-program",
      description: acc.account.description ?? undefined,
      metadataUri: acc.account.metadataUri ?? undefined,
      tags:
        typeof acc.account.tags === "string"
          ? acc.account.tags.split(",").map((s: string) => s.trim()).filter(Boolean)
          : acc.account.tags ?? [],
      isActive: acc.account.isActive,
      currentVersion: versionTuple,
      recommendedVersion: acc.account.recommendedVersion ?? undefined,
      createdAt: Number(acc.account.createdAt ?? 0),
      updatedAt: Number(acc.account.updatedAt ?? acc.account.createdAt ?? 0)
    };
  });

  const limit = filter.limit ?? 100;
  return {
    items: items.slice(0, limit),
    nextCursor: null
  };
}
