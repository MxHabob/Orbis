import { describe, expect, test } from "bun:test";
import type {
  BulkImportProfile,
  BulkReferenceData,
  BulkRowState,
} from "../types";
import { executeCommitPipeline } from "./commit-pipeline";

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
  existingClassSubjectAssignments: [],
};

function validRow(rowNumber: number): BulkRowState {
  return {
    rowNumber,
    data: {},
    errors: new Map(),
    warnings: new Map(),
    info: new Map(),
    status: "valid",
  };
}

describe("bulk commit pipeline", () => {
  test("stops later chunks after a row-level organization license rejection", async () => {
    let commitCalls = 0;
    const profile: BulkImportProfile = {
      id: "students",
      displayName: { ar: "طلاب", en: "Students" },
      description: { ar: "", en: "" },
      templateVersion: "1",
      columnMetadata: [],
      gridColumns: [],
      buildReferenceData: async () => emptyReferenceData,
      validateCell: () => null,
      validateRow: () => [],
      generatePreview: async () => ({
        totalRows: 0,
        validRowsCount: 0,
        errorRowsCount: 0,
        warningRowsCount: 0,
        details: {},
      }),
      commitChunk: async (chunk) => {
        commitCalls++;
        expect(chunk).toHaveLength(2);
        return {
          success: true,
          rowErrors: [
            {
              rowNumber: 1,
              reason:
                "Row 1: Forbidden: The license is not valid for the active organization",
            },
          ],
        };
      },
    };

    const rows = [validRow(1), validRow(2), validRow(3)];
    const result = await executeCommitPipeline({
      rows,
      profile,
      refData: emptyReferenceData,
      orgId: "org-id",
      chunkSize: 2,
    });

    expect(commitCalls).toBe(1);
    expect(result.committedCount).toBe(0);
    expect(result.failedCount).toBe(3);
    expect(result.errors).toHaveLength(3);
    expect(result.licenseRejected).toBe(true);
    expect(rows.every((row) => row.status === "failed")).toBe(true);
  });

  test("stops later chunks when a transaction throws a license rejection", async () => {
    let commitCalls = 0;
    const profile: BulkImportProfile = {
      id: "students",
      displayName: { ar: "طلاب", en: "Students" },
      description: { ar: "", en: "" },
      templateVersion: "1",
      columnMetadata: [],
      gridColumns: [],
      buildReferenceData: async () => emptyReferenceData,
      validateCell: () => null,
      validateRow: () => [],
      generatePreview: async () => ({
        totalRows: 0,
        validRowsCount: 0,
        errorRowsCount: 0,
        warningRowsCount: 0,
        details: {},
      }),
      commitChunk: async () => {
        commitCalls++;
        throw new Error(
          "License required: The license is not valid for the active organization",
        );
      },
    };

    const rows = [validRow(1), validRow(2), validRow(3)];
    const result = await executeCommitPipeline({
      rows,
      profile,
      refData: emptyReferenceData,
      orgId: "org-id",
      chunkSize: 2,
    });

    expect(commitCalls).toBe(1);
    expect(result.committedCount).toBe(0);
    expect(result.failedCount).toBe(3);
    expect(result.licenseRejected).toBe(true);
  });
});
