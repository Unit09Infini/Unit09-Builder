/**
 * Small collection of shared utility types for the SDK.
 */

export type Nullable<T> = T | null | undefined;

export interface PageResult<T> {
  items: T[];
  nextCursor: string | null;
}
