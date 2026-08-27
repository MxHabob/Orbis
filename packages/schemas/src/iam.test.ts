import { describe, expect, test } from "bun:test";
import { loginInputSchema } from "./iam.js";

describe("loginInputSchema", () => {
  test("accepts a username or email with a password", () => {
    const result = loginInputSchema.safeParse({
      identifier: "  admin@example.com ",
      password: "secret",
    });

    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.identifier).toBe("admin@example.com");
    }
  });

  test("rejects missing credentials", () => {
    const result = loginInputSchema.safeParse({
      identifier: "",
      password: "",
    });

    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error.issues).toHaveLength(2);
    }
  });
});
