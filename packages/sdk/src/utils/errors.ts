/**
 * Error type used by the SDK for wrapping lower-level failures.
 */
export class SdkError extends Error {
  readonly cause?: unknown;

  constructor(message: string, cause?: unknown) {
    super(message);
    this.name = "SdkError";
    this.cause = cause;
  }
}

/**
 * Helper to wrap an arbitrary error into an SdkError.
 */
export function wrapSdkError(message: string, cause: unknown): SdkError {
  if (cause instanceof SdkError) {
    return cause;
  }
  const error = new SdkError(message, cause);
  if (cause instanceof Error && cause.stack) {
    error.stack = `${error.stack ?? ""}
Caused by: ${cause.stack}`;
  }
  return error;
}
