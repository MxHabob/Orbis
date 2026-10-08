import { describe, expect, test } from "bun:test";
import { canApplyTranslationSuggestion } from "./suggestion-policy";

describe("canApplyTranslationSuggestion", () => {
  test("allows a suggestion when the target field is empty", () => {
    expect(
      canApplyTranslationSuggestion({
        targetValue: " ",
        targetContainsCurrentSuggestion: false,
        linkedSourceStillMatches: false,
        mayReplaceUneditedInitialValue: false,
      }),
    ).toBe(true);
  });

  test("preserves a target value edited by the user", () => {
    expect(
      canApplyTranslationSuggestion({
        targetValue: "My preferred spelling",
        targetContainsCurrentSuggestion: false,
        linkedSourceStillMatches: false,
        mayReplaceUneditedInitialValue: false,
      }),
    ).toBe(false);
  });

  test("does not replace a non-empty initial value unless it is known to be untouched", () => {
    expect(
      canApplyTranslationSuggestion({
        targetValue: "Existing name",
        targetContainsCurrentSuggestion: false,
        linkedSourceStillMatches: false,
        mayReplaceUneditedInitialValue: false,
      }),
    ).toBe(false);
  });

  test("allows replacing an untouched initial value when explicitly enabled", () => {
    expect(
      canApplyTranslationSuggestion({
        targetValue: "Existing name",
        targetContainsCurrentSuggestion: false,
        linkedSourceStillMatches: false,
        mayReplaceUneditedInitialValue: true,
      }),
    ).toBe(true);
  });

  test("allows refreshing a target generated from the current source", () => {
    expect(
      canApplyTranslationSuggestion({
        targetValue: "Old suggestion",
        targetContainsCurrentSuggestion: true,
        linkedSourceStillMatches: false,
        mayReplaceUneditedInitialValue: false,
      }),
    ).toBe(true);
  });
});
