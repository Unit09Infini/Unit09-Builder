/**
 * Split a comma-separated tag string into an array of trimmed values.
 */
export function parseCsvTags(input: string | null | undefined): string[] {
  if (!input) return [];
  return input
    .split(",")
    .map((v) => v.trim())
    .filter((v) => v.length > 0);
}
