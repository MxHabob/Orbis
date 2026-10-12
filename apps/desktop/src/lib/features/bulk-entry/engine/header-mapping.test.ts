import { describe, expect, test } from "bun:test";
import { findSpreadsheetColumn, normalizeSpreadsheetHeader } from "./header-mapping";

describe("spreadsheet header mapping", () => {
  test("matches the localized class header used by generated templates", () => {
    const columns = [
      {
        id: "class_name",
        headerAr: "الصف / الشعبة*",
        headerEn: "Class / Section*",
        headerAliases: ["المستوى الدراسي / الشعبة"],
      },
    ];

    const match = findSpreadsheetColumn(
      "المستوى الدراسي / الشعبة*",
      columns,
      () => "Class / Section*",
    );

    expect(match?.id).toBe("class_name");
  });

  test("normalizes required markers, punctuation, and Arabic diacritics", () => {
    expect(normalizeSpreadsheetHeader(" الصَّف / الشعبة* ")).toBe(
      normalizeSpreadsheetHeader("الصف-الشعبة"),
    );
  });
});
