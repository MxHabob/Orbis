import type { BulkProfileId } from "../types";

export interface BulkValidationContext {
  duplicateIndexes: Map<string, Map<string, [number, number?]>>;
}

type Row = Record<string, unknown>;

function getKeys(profileId: BulkProfileId, row: Row): Array<[string, string]> {
  const value = (field: string) => String(row[field] ?? "").trim();

  if (profileId === "students") {
    return [
      ["national_id", value("national_id")],
      ["student_number", value("student_number")],
    ];
  }
  if (profileId === "employees") {
    return [["national_id", value("national_id")]];
  }
  if (profileId === "grades") {
    const keyParts = ["student_number", "subject_name", "assessment_type"].map(value);
    return keyParts.every(Boolean)
      ? [["grade_assessment", JSON.stringify(keyParts)]]
      : [];
  }
  return [];
}

function normalize(value: string): string {
  return value.trim().toLocaleLowerCase();
}

export function createBulkValidationContext(
  profileId: BulkProfileId,
  rows: Row[],
): BulkValidationContext {
  const duplicateIndexes = new Map<string, Map<string, [number, number?]>>();

  rows.forEach((row, index) => {
    for (const [field, rawValue] of getKeys(profileId, row)) {
      if (!rawValue) continue;
      const value = normalize(rawValue);
      let values = duplicateIndexes.get(field);
      if (!values) {
        values = new Map();
        duplicateIndexes.set(field, values);
      }
      const indexes = values.get(value);
      if (!indexes) values.set(value, [index]);
      else if (indexes[1] === undefined) indexes[1] = index;
    }
  });

  return { duplicateIndexes };
}

export function findDuplicateIndex(
  context: BulkValidationContext,
  field: string,
  value: string,
  rowIndex: number,
): number | undefined {
  const indexes = context.duplicateIndexes.get(field)?.get(normalize(value));
  if (!indexes) return undefined;
  return indexes[0] === rowIndex ? indexes[1] : indexes[0];
}
