import type { Program } from "@coral-xyz/anchor";
import type { RepoFilter, RepoListPage, RepoMetadata } from "@unit09/shared-types";
import { normalizeRepoTags } from "@unit09/shared-types";

/**
 * List repos by scanning all repo accounts and projecting them to RepoMetadata.
 * This helper assumes an Anchor account named `repo` with a layout compatible
 * with the RepoMetadata fields. Adjust mapping as needed.
 */
export async function listRepos(program: Program, filter: RepoFilter = {}): Promise<RepoListPage> {
  const accounts = await program.account.repo.all();

  const items: RepoMetadata[] = accounts.map((acc: any) => ({
    repoKey: acc.account.repoKey?.toString?.() ?? acc.publicKey.toString(),
    name: acc.account.name,
    url: acc.account.url,
    description: acc.account.description ?? undefined,
    tags: normalizeRepoTags(
      typeof acc.account.tags === "string"
        ? acc.account.tags.split(",")
        : acc.account.tags ?? []
    ),
    visibility: acc.account.visibility ?? "public",
    sourceType: acc.account.sourceType ?? "github",
    defaultBranch: acc.account.defaultBranch ?? undefined,
    allowObservation: acc.account.allowObservation,
    isActive: acc.account.isActive,
    stats: acc.account.stats ?? undefined,
    createdAt: Number(acc.account.createdAt ?? 0),
    updatedAt: Number(acc.account.updatedAt ?? acc.account.createdAt ?? 0)
  }));

  const filtered = items.filter((item) => {
    if (filter.activeOnly && !item.isActive) return false;
    if (filter.visibility && item.visibility !== filter.visibility) return false;
    if (filter.sourceType && item.sourceType !== filter.sourceType) return false;
    if (filter.search) {
      const s = filter.search.toLowerCase();
      if (!item.name.toLowerCase().includes(s) && !item.url.toLowerCase().includes(s)) {
        return false;
      }
    }
    if (filter.tags && filter.tags.length > 0) {
      const tagSet = new Set(item.tags);
      const hasAll = filter.tags.every((t) => tagSet.has(t.toLowerCase()));
      if (!hasAll) return false;
    }
    return true;
  });

  const limit = filter.limit ?? 100;
  return {
    items: filtered.slice(0, limit),
    nextCursor: null
  };
}
