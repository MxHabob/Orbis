import { describe, expect, test } from "bun:test";
import {
  createRoomInputSchema,
  createSubjectInputSchema,
} from "./academic.js";

describe("academic input schemas", () => {
  test("accepts a subject with one localized name", () => {
    const result = createSubjectInputSchema.safeParse({
      name: "Mathematics",
      name_ar: "",
      code: "MATH-01",
      color: null,
    });

    expect(result.success).toBe(true);
  });

  test("rejects a room without a name or invalid capacity", () => {
    const result = createRoomInputSchema.safeParse({
      name: "",
      name_ar: "",
      capacity: 0,
      room_type: "classroom",
      building: null,
      floor: null,
    });

    expect(result.success).toBe(false);
  });
});