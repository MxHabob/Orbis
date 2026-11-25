import { describe, expect, test } from "bun:test";
import {
  createBulkValidationContext,
  findDuplicateIndex,
} from "./duplicate-index";

describe("bulk duplicate index", () => {
  test("finds the other matching row without scanning the batch", () => {
    const context = createBulkValidationContext("students", [
      { national_id: "A-1" },
      { national_id: " a-1 " },
      { national_id: "B-2" },
    ]);

    expect(findDuplicateIndex(context, "national_id", "A-1", 0)).toBe(1);
    expect(findDuplicateIndex(context, "national_id", "A-1", 1)).toBe(0);
    expect(
      findDuplicateIndex(context, "national_id", "B-2", 2),
    ).toBeUndefined();
  });
});
