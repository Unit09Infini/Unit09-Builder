export type Unit09ErrorCode =
  | "CONFIG_NOT_FOUND"
  | "REPO_NOT_FOUND"
  | "MODULE_NOT_FOUND"
  | "FORK_NOT_FOUND"
  | "METRICS_NOT_FOUND"
  | "PIPELINE_FAILED"
  | "VALIDATION_FAILED"
  | "UNSUPPORTED_OPERATION"
  | "NETWORK_ERROR"
  | "UNKNOWN";

export interface Unit09ErrorShape {
  code: Unit09ErrorCode;
  message: string;
  details?: unknown;
  retriable?: boolean;
  stack?: string;
}

export class Unit09Error extends Error implements Unit09ErrorShape {
  code: Unit09ErrorCode;
  details?: unknown;
  retriable?: boolean;

  constructor(input: Unit09ErrorShape) {
    super(input.message);
    this.name = "Unit09Error";
    this.code = input.code;
    this.details = input.details;
    this.retriable = input.retriable;
    if (input.stack) {
      this.stack = input.stack;
    }
  }

  toJSON(): Unit09ErrorShape {
    return {
      code: this.code,
      message: this.message,
      details: this.details,
      retriable: this.retriable,
      stack: this.stack,
    };
  }
}

export function createUnit09Error(
  code: Unit09ErrorCode,
  message: string,
  details?: unknown,
  retriable?: boolean
): Unit09Error {
  return new Unit09Error({ code, message, details, retriable });
}

export function isUnit09ErrorShape(value: any): value is Unit09ErrorShape {
  return (
    value != null &&
    typeof value === "object" &&
    typeof value.code === "string" &&
    typeof value.message === "string"
  );
}

export function isUnit09Error(value: unknown): value is Unit09Error {
  return value instanceof Unit09Error;
}
