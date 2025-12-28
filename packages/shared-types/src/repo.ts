export type RepoVisibility = "public" | "private";

export type RepoSourceType = "github" | "git-local" | "archive" | "custom";

export interface RepoIdentifier {
  repoKey: string;
}

export interface RepoStats {
  files: number;
  linesOfCode: number;
  modulesDetected: number;
  lastObservationAt?: number;
}

export interface RepoMetadata extends RepoIdentifier {
  name: string;
  url: string;
  description?: string;
  tags: string[];
  visibility: RepoVisibility;
  sourceType: RepoSourceType;
  defaultBranch?: string;
  allowObservation: boolean;
  isActive: boolean;
  stats?: RepoStats;
  createdAt: number;
  updatedAt: number;
}

export interface CreateRepoInput {
  name: string;
  url: string;
  description?: string;
  tags?: string[];
  visibility?: RepoVisibility;
  sourceType?: RepoSourceType;
  defaultBranch?: string;
  allowObservation?: boolean;
}

export interface UpdateRepoInput {
  name?: string;
  url?: string;
  description?: string;
  tags?: string[];
  visibility?: RepoVisibility;
  defaultBranch?: string;
  allowObservation?: boolean;
  isActive?: boolean;
}

export interface RepoFilter {
  search?: string;
  tags?: string[];
  visibility?: RepoVisibility;
  sourceType?: RepoSourceType;
  activeOnly?: boolean;
  limit?: number;
  cursor?: string | null;
}

export interface RepoListPage {
  items: RepoMetadata[];
  nextCursor: string | null;
}

export function normalizeRepoTags(tags: string[] | undefined): string[] {
  if (!tags || tags.length === 0) return [];
  return tags
    .map((tag) => tag.trim())
    .filter((tag) => tag.length > 0)
    .map((tag) => tag.toLowerCase());
}
