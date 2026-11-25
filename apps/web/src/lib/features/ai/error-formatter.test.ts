import { describe, expect, test } from "bun:test";
import * as m from "$lib/paraglide/messages";
import { formatAiErrorMessage } from "./error-formatter";

describe("AI error formatter", () => {
  test("treats OpenRouter credit exhaustion as a quota problem", () => {
    const message = formatAiErrorMessage(
      "This request requires more credits, or fewer max_tokens. You requested up to 2048 tokens, but can only afford 1386. To increase, visit https://openrouter.ai/workspaces/default/keys/41fca1...",
    );

    expect(message).toContain(m.ai_ui_error_quota_description());
    expect(message).toContain(m.ai_ui_error_quota_solution());
    expect(message).not.toContain("requires more credits");
  });

  test("keeps generic quota errors on the quota path", () => {
    const message = formatAiErrorMessage({
      message: "Quota exceeded for model openai/gpt-4o-mini",
    });

    expect(message).toContain(m.ai_ui_error_quota_description());
    expect(message).toContain(m.ai_ui_error_quota_solution());
  });
});
