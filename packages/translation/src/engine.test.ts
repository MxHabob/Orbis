import { describe, expect, test } from "bun:test";
import { academicDictionaryPack } from "./dictionaries/academic";
import { createTranslationEngine } from "./engine";
import type { DictionaryPack } from "./types";

describe("offline translation engine", () => {
  const engine = createTranslationEngine([academicDictionaryPack]);

  test("translates known Arabic academic terms", () => {
    expect(
      engine.suggest({
        text: "قاعة العلوم",
        sourceLanguage: "ar",
        kind: "label",
      }),
    ).toEqual({
      status: "suggestion",
      text: "Science Room",
      method: "dictionary",
      approximate: false,
    });
  });

  test("translates known English terms in reverse", () => {
    expect(
      engine.suggest({
        text: "Academic Year",
        sourceLanguage: "en",
        kind: "label",
      }),
    ).toMatchObject({ status: "suggestion", text: "السنة الدراسية" });
  });

  test("prefers the longest phrase and preserves punctuation and numbers", () => {
    expect(
      engine.suggest({
        text: "الفصل الدراسي 2!",
        sourceLanguage: "ar",
        kind: "label",
      }),
    ).toMatchObject({ status: "suggestion", text: "Class 2!" });
  });

  test("does not return a partial translation for an unknown term", () => {
    expect(
      engine.suggest({
        text: "قاعة الكواكب",
        sourceLanguage: "ar",
        kind: "label",
      }),
    ).toEqual({ status: "unavailable", reason: "unknown-term" });
  });

  test("normalizes Arabic diacritics and alef variants", () => {
    expect(
      engine.suggest({
        text: "قَاعَةُ العُلُوم",
        sourceLanguage: "ar",
        kind: "label",
      }),
    ).toMatchObject({ status: "suggestion", text: "Science Room" });
  });

  test("uses a known Arabic name spelling when available", () => {
    expect(
      engine.suggest({
        text: "محمد",
        sourceLanguage: "ar",
        kind: "person-name",
      }),
    ).toEqual({
      status: "suggestion",
      text: "Mohammed",
      method: "dictionary",
      approximate: true,
    });
  });

  test("transliterates unknown names in both directions approximately", () => {
    expect(
      engine.suggest({
        text: "زَيد",
        sourceLanguage: "ar",
        kind: "person-name",
      }),
    ).toMatchObject({ status: "suggestion", text: "zyd", approximate: true });

    expect(
      engine.suggest({
        text: "Thom",
        sourceLanguage: "en",
        kind: "person-name",
      }),
    ).toMatchObject({ status: "suggestion", text: "ثوم", approximate: true });
  });

  test("transliterates multi-part names and preserves their separators", () => {
    expect(
      engine.suggest({
        text: "محمد علي",
        sourceLanguage: "ar",
        kind: "person-name",
      }),
    ).toMatchObject({ status: "suggestion", text: "Mohammed Ali" });
  });

  test("supports adding local name spellings", () => {
    const customEngine = createTranslationEngine([
      {
        id: "local-names",
        version: 1,
        entries: [],
        nameEntries: [{ ar: "زَيد", en: "Zayd" }],
      },
    ]);

    expect(
      customEngine.suggest({
        text: "زيد",
        sourceLanguage: "ar",
        kind: "person-name",
      }),
    ).toMatchObject({
      status: "suggestion",
      text: "Zayd",
      method: "dictionary",
    });
  });

  test("supports local dictionary packs with later packs taking precedence", () => {
    const customPack: DictionaryPack = {
      id: "school-custom",
      version: 1,
      entries: [{ ar: "رياضيات", en: "Maths" }],
    };
    const customEngine = createTranslationEngine([
      academicDictionaryPack,
      customPack,
    ]);

    expect(
      customEngine.suggest({
        text: "رياضيات",
        sourceLanguage: "ar",
        kind: "label",
      }),
    ).toMatchObject({ status: "suggestion", text: "Maths" });
  });

  test("rejects duplicate entries within a dictionary pack", () => {
    expect(() =>
      createTranslationEngine([
        {
          id: "invalid",
          version: 1,
          entries: [
            { ar: "قاعة", en: "Room" },
            { ar: " قاعة ", en: "Hall" },
          ],
        },
      ]),
    ).toThrow("Duplicate dictionary entry");
  });

  test("reports empty input without creating a suggestion", () => {
    expect(
      engine.suggest({
        text: " ",
        sourceLanguage: "ar",
        kind: "label",
      }),
    ).toEqual({ status: "unavailable", reason: "empty" });
  });

  test("suggestion latency stays below the 10 ms p95 budget", () => {
    const text = Array.from({ length: 30 }, () => "قاعة").join(" ");
    const samples: number[] = [];

    for (let sample = 0; sample < 100; sample += 1) {
      const start = performance.now();
      engine.suggest({ text, sourceLanguage: "ar", kind: "label" });
      samples.push(performance.now() - start);
    }

    samples.sort((left, right) => left - right);
    expect(samples[94]!).toBeLessThan(10);
  });
});
