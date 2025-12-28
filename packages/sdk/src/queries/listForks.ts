import type { Program } from "@coral-xyz/anchor";
import type { ForkFilter, ForkListPage, ForkMetadata } from "@unit09/shared-types";

/**
 * List forks, optionally filtering by forkType, parent, or activeOnly flag.
 */
export async function listForks(
  program: Program,
  filter: ForkFilter = {}
): Promise<ForkListPage> {
  const accounts = await program.account.fork.all();

  const items: ForkMetadata[] = accounts.map((acc: any) => ({
    forkKey: acc.account.forkKey?.toString?.() ?? acc.publicKey.toString(),
    parent: acc.account.parent ?? null,
    label: acc.account.label,
    forkType: acc.account.forkType ?? "unit09-instance",
    metadataUri: acc.account.metadataUri ?? undefined,
    tags:
      typeof acc.account.tags === "string"
        ? acc.account.tags.split(",").map((s: string) => s.trim()).filter(Boolean)
        : acc.account.tags ?? [],
    depth: acc.account.depth ?? 0,
    isRoot: acc.account.isRoot ?? false,
    isActive: acc.account.isActive ?? true,
    createdAt: Number(acc.account.createdAt ?? 0),
    updatedAt: Number(acc.account.updatedAt ?? acc.account.createdAt ?? 0)
  }));

  const filtered = items.filter((item) => {
    if (filter.forkType && item.forkType !== filter.forkType) return false;
    if (filter.parent !== undefined && item.parent !== filter.parent) return false;
    if (filter.activeOnly && !item.isActive) return false;
    return true;
  });

  const limit = filter.limit ?? 100;
  return {
    items: filtered.slice(0, limit),
    nextCursor: null
  };
}
