import { describe, expect, test } from "bun:test";
import { OrbisError } from "./errors.js";

describe("OrbisError.fromUnknown", () => {
  test("preserves structured Tauri validation errors", () => {
    const error = OrbisError.fromUnknown({
      message: "Identifier is required",
      code: "VALIDATION_ERROR",
      field: "identifier",
    });

    expect(error.code).toBe("VALIDATION_ERROR");
    expect(error.statusCode).toBe(422);
    expect(error.details).toEqual({ field: "identifier", details: undefined });
  });
});
