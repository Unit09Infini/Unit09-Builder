export type ForkType =
  | "unit09-instance"
  | "module-variant"
  | "experiment"
  | "archive";

export interface ForkIdentifier {
  forkKey: string;
}

export interface ForkMetadata extends ForkIdentifier {
  parent?: string | null;
  label: string;
  forkType: ForkType;
  metadataUri?: string;
  tags: string[];
  depth: number;
  isRoot: boolean;
  isActive: boolean;
  createdAt: number;
  updatedAt: number;
}

export interface ForkLineage {
  rootForkKey: string;
  path: string[];
}

export interface ForkFilter {
  forkType?: ForkType;
  parent?: string | null;
  activeOnly?: boolean;
  limit?: number;
  cursor?: string | null;
}

export interface ForkListPage {
  items: ForkMetadata[];
  nextCursor: string | null;
}

export function buildForkLineage(fork: ForkMetadata, ancestors: ForkMetadata[]): ForkLineage {
  const sorted = [...ancestors].sort((a, b) => a.depth - b.depth);
  const pathKeys = sorted.map((item) => item.forkKey);
  pathKeys.push(fork.forkKey);
  const root = sorted[0]?.forkKey ?? fork.forkKey;
  return {
    rootForkKey: root,
    path: pathKeys,
  };
}
