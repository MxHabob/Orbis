import { expect, test } from "bun:test";
import * as XLSX from "xlsx";
import type { GradeDto } from "$lib/api";
import { buildGradeWorkbook, reviewGradeWorkbook, type GradeWorkbookSheet } from "./grade-workbook";
import { gradeStatus, parseSheetScore } from "./class-grade-sheet";

const context = { orgId: "org-a", periodId: "month-1", orgName: "School", periodLabel: "Month 1", arabic: false };
const field = "exam_math:written";
const component = { field, code: "written", name: "Written", max_score: 20, sort_order: 1 };

function scope(id: string, score?: number): GradeWorkbookSheet {
  const grade = score === undefined ? undefined : {
    score: String(score), component_code: "written", updated_at: "revision-1",
  } as GradeDto;
  return {
    classId: id,
    className: "Section " + id,
    rosterPeriodId: "term-1",
    canEnter: true,
    subjects: [{ id: "math", name: "Math", schemeId: "scheme", schemeRevision: "1", components: [component] }],
    rows: [{
      student_id: "student-" + id, student_name: "Student", student_name_ar: "Student",
      enrollment_id: "enrollment-" + id, savedGrades: { [field]: grade },
      [field]: score,
    }],
  };
}

test("Arabic attendance labels remain accepted regardless of the app language", () => {
  expect(parseSheetScore("غ", 20)).toBe(gradeStatus.absent());
  expect(parseSheetScore("معذور", 20)).toBe(gradeStatus.excused());
  expect(parseSheetScore("بعذر", 20)).toBe(gradeStatus.excused());
  expect(parseSheetScore("معفى", 20)).toBe(gradeStatus.exempt());
});

test("each section round-trips with its own enrollment, and blank cells leave grades untouched", async () => {
  const first = scope("A", 12);
  const second = scope("B");
  const workbook = buildGradeWorkbook(context, [first, second]);
  XLSX.utils.sheet_add_aoa(workbook.Sheets[workbook.SheetNames[0]!]!, [[""]], { origin: "E6" });
  XLSX.utils.sheet_add_aoa(workbook.Sheets[workbook.SheetNames[1]!]!, [[0]], { origin: "E6" });
  const decoded = XLSX.read(XLSX.write(workbook, { bookType: "xlsx", type: "array" }), { type: "array" });
  const result = await reviewGradeWorkbook(decoded, context, async (id) => id === "A" ? first : second, false);

  expect(decoded.Workbook?.Sheets?.at(-1)?.Hidden).toBe(2);
  expect(result.errors).toEqual([]);
  expect(result.sheets.map((sheet) => [sheet.classId, sheet.rows[0]?.student_name])).toEqual([["A", "Student"], ["B", "Student"]]);
  expect(result.changes.map(({ item }) => [item.student_id, item.enrollment_id, item.period_id, item.score]))
    .toEqual([["student-B", "enrollment-B", "month-1", "0"]]);
});

test("review keeps closed-period edits available for a request and rejects stale data", async () => {
  const original = scope("A", 12);
  const workbook = buildGradeWorkbook(context, [original]);
  XLSX.utils.sheet_add_aoa(workbook.Sheets[workbook.SheetNames[0]!]!, [[15]], { origin: "E6" });

  const closed = await reviewGradeWorkbook(workbook, context, async () => ({ ...original, canEnter: false }), false);
  expect(closed.sheets[0]?.canEnter).toBe(false);
  expect(closed.changes.map(({ item }) => item.score)).toEqual(["15"]);
  expect(closed.errors).toEqual([]);

  for (const conflict of [
    { ...original, subjects: [{ ...original.subjects[0]!, schemeRevision: "2" }] },
    scope("A", 13),
  ]) {
    const result = await reviewGradeWorkbook(workbook, context, async () => conflict, false);
    expect(result.changes).toEqual([]);
    expect(result.errors).toHaveLength(1);
  }

  await expect(reviewGradeWorkbook(workbook, { ...context, periodId: "month-2" }, async () => original, false)).rejects.toThrow();
});
