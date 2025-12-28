export type ModuleKind =
  | "anchor-program"
  | "solana-client"
  | "typescript-sdk"
  | "cli-tool"
  | "frontend-stub"
  | "config-bundle"
  | "other";

export type SemanticVersionTuple = [number, number, number];

export interface ModuleIdentifier {
  moduleKey: string;
}

export interface ModuleVersionSnapshot {
  moduleKey: string;
  version: SemanticVersionTuple;
  versionLabel: string;
  changelogUri?: string;
  isStable: boolean;
  createdAt: number;
  notes?: string;
}

export interface ModuleMetadata extends ModuleIdentifier {
  repoKey: string;
  name: string;
  kind: ModuleKind;
  description?: string;
  metadataUri?: string;
  tags: string[];
  isActive: boolean;
  currentVersion: SemanticVersionTuple;
  recommendedVersion?: SemanticVersionTuple;
  createdAt: number;
  updatedAt: number;
}

export interface ModuleDependencyEdge {
  fromModuleKey: string;
  toModuleKey: string;
  kind: "build-time" | "runtime" | "dev";
}

export interface ModuleWithVersions {
  module: ModuleMetadata;
  versions: ModuleVersionSnapshot[];
}

export interface ModuleFilter {
  repoKey?: string;
  search?: string;
  tags?: string[];
  kind?: ModuleKind;
  activeOnly?: boolean;
  limit?: number;
  cursor?: string | null;
}

export interface ModuleListPage {
  items: ModuleMetadata[];
  nextCursor: string | null;
}

export function parseSemanticVersion(input: string): SemanticVersionTuple | null {
  const trimmed = input.trim();
  const withoutPrefix = trimmed.startsWith("v") ? trimmed.slice(1) : trimmed;
  const parts = withoutPrefix.split(".");
  if (parts.length !== 3) return null;
  const [majorStr, minorStr, patchStr] = parts;
  const major = Number(majorStr);
  const minor = Number(minorStr);
  const patch = Number(patchStr);
  if (!Number.isInteger(major) || !Number.isInteger(minor) || !Number.isInteger(patch)) {
    return null;
  }
  return [major, minor, patch];
}

export function formatSemanticVersion(version: SemanticVersionTuple): string {
  const [major, minor, patch] = version;
  return `v${major}.${minor}.${patch}`;
}

export function compareSemanticVersion(
  a: SemanticVersionTuple,
  b: SemanticVersionTuple
): number {
  for (let i = 0; i < 3; i += 1) {
    if (a[i] < b[i]) return -1;
    if (a[i] > b[i]) return 1;
  }
  return 0;
}

export function bumpSemanticVersion(
  version: SemanticVersionTuple,
  part: "major" | "minor" | "patch"
): SemanticVersionTuple {
  const [major, minor, patch] = version;
  if (part == "major") return [major + 1, 0, 0];
  if (part == "minor") return [major, minor + 1, 0];
  return [major, minor, patch + 1];
}
