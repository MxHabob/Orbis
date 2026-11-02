import { describe, expect, test } from "bun:test";
import { createBulkValidationContext } from "../engine/duplicate-index";
import { revalidateCellChange } from "../engine/revalidator";
import type { BulkReferenceData } from "../types";
import {
  buildGradeBatchItems,
  type GradeRowData,
  gradeImportProfile,
} from "./grade-profile";

const emptyReferenceData: BulkReferenceData = {
  classes: [],
  teachers: [],
  sections: [],
  levels: [],
  subjects: [],
  feeTemplates: [],
  discountTemplates: [],
  userGroups: [],
  assessmentSchemes: [],
  academicPeriods: [],
  enums: {
    genders: [],
    studyModes: [],
    employeeTypes: [],
    employmentTypes: [],
    bloodTypes: [],
    relationships: [],
  },
  existingSectionSubjectAssignments: [],
};

const validRow: GradeRowData = {
  student_id: "student-1",
  period_id: "period-1",
  section_subject_id: "offering-1",
  component_id: "component-1",
  score: "18.5",
  max_score: "20",
};

describe("grade import", () => {
  test("maps explicit academic IDs and score bounds to the batch API contract", () => {
    expect(buildGradeBatchItems([validRow])).toEqual([
      {
        student_id: "student-1",
        period_id: "period-1",
        section_subject_id: "offering-1",
        component_id: "component-1",
        score: "18.5",
        max_score: "20",
        notes: undefined,
      },
    ]);
  });

  test("rejects absent IDs and scores outside the explicitly supplied maximum", () => {
    expect(() =>
      buildGradeBatchItems([{ ...validRow, period_id: "" }]),
    ).toThrow(TypeError);
    expect(() =>
      buildGradeBatchItems([{ ...validRow, score: "20.01" }]),
    ).toThrow(RangeError);
    expect(() =>
      buildGradeBatchItems([{ ...validRow, max_score: "" }]),
    ).toThrow(RangeError);
  });

  test("validates duplicate student-period-offering-component combinations", () => {
    const rows = [
      validRow,
      {
        ...validRow,
        student_id: "STUDENT-1",
        score: "12",
      },
    ];
    const context = createBulkValidationContext("grades", rows);

    expect(
      gradeImportProfile.validateRow(
        rows[1]!,
        rows,
        1,
        emptyReferenceData,
        context,
      ),
    ).toHaveLength(1);
  });

  test("does not default a missing maximum score", () => {
    const issue = gradeImportProfile.validateCell(
      "max_score",
      "",
      validRow,
      emptyReferenceData,
    );

    expect(issue?.severity).toBe("BLOCKING_ERROR");
  });

  test("revalidates the existing score when the maximum is lowered", () => {
    const issue = gradeImportProfile.validateCell(
      "max_score",
      "10",
      { ...validRow, score: "15" },
      emptyReferenceData,
    );

    expect(issue?.columnId).toBe("score");
  });

  test("clears the score error when the maximum is raised again", () => {
    const row = {
      rowNumber: 2,
      data: { ...validRow, id: "row-1", score: "15", max_score: "10" },
      errors: new Map([["score", "Score exceeds maximum"]]),
      warnings: new Map<string, string>(),
      info: new Map<string, string>(),
      status: "error" as const,
    };

    const result = revalidateCellChange(
      [row],
      { rowIndex: 0, columnId: "max_score", newValue: "20" },
      gradeImportProfile,
      emptyReferenceData,
    );

    expect(result.updatedRows[0]?.errors.has("score")).toBe(false);
    expect(result.updatedRows[0]?.status).toBe("valid");
  });
});
