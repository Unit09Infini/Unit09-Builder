import { BadRequestError } from "./httpErrors";

export function requireString(value: unknown, fieldName: string): string {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new BadRequestError(`Field '${fieldName}' is required and must be a non-empty string`);
  }
  return value.trim();
}

export function optionalString(value: unknown): string | undefined {
  if (typeof value !== "string") return undefined;
  const trimmed = value.trim();
  return trimmed.length === 0 ? undefined : trimmed;
}

export function requireNumber(value: unknown, fieldName: string): number {
  const num = Number(value);
  if (!Number.isFinite(num)) {
    throw new BadRequestError(`Field '${fieldName}' must be a valid number`);
  }
  return num;
}

export function parsePagination(query: any): { limit: number; offset: number } {
  const limit = query.limit ? requireNumber(query.limit, "limit") : 50;
  const offset = query.offset ? requireNumber(query.offset, "offset") : 0;
  return { limit, offset };
}
