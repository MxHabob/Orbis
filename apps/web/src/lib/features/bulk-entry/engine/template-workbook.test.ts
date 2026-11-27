import { expect, test } from "bun:test";
import * as XLSX from "xlsx";
import type { BulkImportProfile, BulkReferenceData } from "../types";
import { parseSpreadsheetBytes } from "./file-parser";
import { buildTemplateWorkbook } from "./template-generator";

const profile = {
  id: "students",
  templateVersion: "1",
  columnMetadata: [
    {
      id: "full_name_ar",
      headerAr: "Name",
      headerEn: "Name",
      required: true,
      type: "text",
      example: "Example student",
    },
    {
      id: "national_id",
      headerAr: "National ID",
      headerEn: "National ID",
      required: false,
      type: "text",
    },
  ],
} as BulkImportProfile<any, any>;

const referenceData: BulkReferenceData = {
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
  existingSectionSubjectAssignments: [],
  enums: {
    genders: [],
    studyModes: [],
    employeeTypes: [],
    employmentTypes: [],
    bloodTypes: [],
    relationships: [],
  },
};

function bytes(workbook: XLSX.WorkBook) {
  return new Uint8Array(
    XLSX.write(workbook, { bookType: "xlsx", type: "array" }),
  );
}

test("an empty template never stages its examples, and a filled template preserves identifier zeros", async () => {
  const template = await buildTemplateWorkbook({
    profile,
    refData: referenceData,
  });
  const firstSheet = template.Sheets[template.SheetNames[0]!]!;
  await expect(
    parseSpreadsheetBytes(bytes(template), "template.xlsx", profile),
  ).rejects.toThrow();
  XLSX.utils.sheet_add_aoa(firstSheet, [["Student", "001234"]], {
    origin: "A4",
  });
  const result = await parseSpreadsheetBytes(
    bytes(template),
    "template.xlsx",
    profile,
  );
  expect(
    result.rows.map((row) => [
      row.rowNumber,
      row.data.full_name_ar,
      row.data.national_id,
    ]),
  ).toEqual([[4, "Student", "001234"]]);
});

test("wrong-profile and duplicate mapping headers are rejected", async () => {
  const parse = (header: string[]) => {
    const workbook = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(
      workbook,
      XLSX.utils.aoa_to_sheet([header, ["a", "b"]]),
      "Data",
    );
    return parseSpreadsheetBytes(bytes(workbook), "input.xlsx", profile);
  };
  await expect(parse(["price", "product"])).rejects.toThrow();
  await expect(parse(["national_id", "National ID"])).rejects.toThrow();
});

test("CSV identifiers stay as text", async () => {
  const csv = new TextEncoder().encode(
    "full_name_ar,national_id\nStudent,001234\n",
  );
  const result = await parseSpreadsheetBytes(csv, "students.csv", profile);
  expect(result.rows[0]?.data.national_id).toBe("001234");
});
