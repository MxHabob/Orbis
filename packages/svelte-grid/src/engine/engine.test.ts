import { describe, expect, test } from "bun:test";
import { applyFilter } from "./filter";
import { applySort, toggleSort } from "./sort";
import {
  emptySelection,
  isRowSelected,
  selectedCount,
  selectScope,
  toggleId,
  rangeContains,
  normalizeRange,
} from "./selection";
import {
  applyPaste,
  parseClipboard,
  toTsv,
  fillDown,
  duplicateRows,
} from "./clipboard";
import {
  interpretEditorKey,
  interpretKey,
  clampIndex,
  resolveCellStep,
  detectEntryAxis,
} from "./keyboard";
import {
  countFrozenColumns,
  scrollForColumn,
  visibleVariableWindow,
  visibleWindow,
} from "./virtual";
import { groupRows, grandTotals } from "./grouping";
import { queryClient } from "./query";
import { autoscrollDelta, colFromOffset, rowFromOffset } from "./pointer";
import { coerceValue } from "./coerce";
import { headerLayout } from "./headers";
import type { GridRow, OrbisColumnDef } from "../types";

const cols: OrbisColumnDef[] = [
  { id: "name", header: "Name", type: "text" },
  { id: "gpa", header: "GPA", type: "decimal" },
  {
    id: "dept",
    header: "Dept",
    type: "enum",
    options: [{ value: "cs", label: "CS" }],
  },
  { id: "active", header: "Active", type: "boolean" },
];

function rows(): GridRow[] {
  return [
    { id: "1", name: "Ali", gpa: 3.9, dept: "cs", active: true },
    { id: "2", name: "Sara", gpa: 3.2, dept: "cs", active: true },
    { id: "3", name: "Omar", gpa: 2.1, dept: "ee", active: false },
  ];
}

describe("filter", () => {
  test("eq + gt AND nested group OR", () => {
    const result = applyFilter(rows(), {
      logic: "AND",
      conditions: [{ field: "active", operator: "bool_true" }],
      groups: [
        {
          logic: "OR",
          conditions: [
            { field: "gpa", operator: "gt", value: 3.5 },
            { field: "name", operator: "starts_with", value: "S" },
          ],
        },
      ],
    });
    expect(result.map((r) => r.id)).toEqual(["1", "2"]);
  });

  test("search is case-insensitive", () => {
    expect(
      applyFilter(rows(), { logic: "AND", conditions: [], search: "ali" }),
    ).toHaveLength(1);
  });
  test("search includes boolean values", () => {
    expect(
      applyFilter(rows(), { logic: "AND", conditions: [], search: "true" }),
    ).toHaveLength(2);
  });
});

describe("sort", () => {
  test("multi-column stable sort", () => {
    const data = [
      { id: "a", dept: "cs", gpa: 3, name: "B" },
      { id: "b", dept: "cs", gpa: 3, name: "A" },
      { id: "c", dept: "ee", gpa: 4, name: "C" },
    ];
    const sorted = applySort(
      data,
      [
        { id: "dept", dir: "asc" },
        { id: "gpa", dir: "desc" },
        { id: "name", dir: "asc" },
      ],
      cols,
    );
    expect(sorted.map((r) => r.id)).toEqual(["b", "a", "c"]);
  });

  test("toggle cycles asc/desc/off", () => {
    let s = toggleSort([], "gpa", false);
    expect(s).toEqual([{ id: "gpa", dir: "asc" }]);
    s = toggleSort(s, "gpa", false);
    expect(s).toEqual([{ id: "gpa", dir: "desc" }]);
    s = toggleSort(s, "gpa", false);
    expect(s).toEqual([]);
  });
});

describe("selection", () => {
  test("allMatching excludes toggled ids without storing every id", () => {
    const sel = selectScope(["1", "2", "3"], "filtered", ["1"], ["1"]);
    expect(sel.mode).toBe("allMatching");
    expect(selectedCount(sel, 10000)).toBe(10000);
    const next = toggleId(sel, "5");
    expect(isRowSelected(next, "5", 10000)).toBe(false);
    expect(isRowSelected(next, "1", 10000)).toBe(true);
    expect(selectedCount(next, 10000)).toBe(9999);
  });

  test("range contains", () => {
    const r = normalizeRange({
      startRow: 4,
      startCol: 2,
      endRow: 1,
      endCol: 0,
    });
    expect(r).toEqual({ startRow: 1, startCol: 0, endRow: 4, endCol: 2 });
    expect(rangeContains(r, 2, 1)).toBe(true);
    expect(rangeContains(r, 5, 1)).toBe(false);
  });
});

describe("clipboard", () => {
  test("parse TSV from Excel", () => {
    expect(parseClipboard("A\tB\nC\tD\n")).toEqual([
      ["A", "B"],
      ["C", "D"],
    ]);
  });

  test("paste validates and reports partial success", () => {
    const { result } = applyPaste({
      rows: [{ id: "1", name: "A", gpa: 1, dept: "cs", active: true }],
      columns: cols,
      startRow: 0,
      startCol: 0,
      matrix: [
        ["Zaid", "3.8", "cs", "true"],
        ["Bad", "nope", "cs", "true"],
        ["Lina", "2.2", "math", "true"],
      ],
      createEmptyRow: () => ({
        id: crypto.randomUUID(),
        name: "",
        gpa: null,
        dept: "cs",
        active: true,
      }),
    });
    expect(result.acceptedCount).toBe(1);
    expect(result.rejectedCount).toBe(2);
    expect(result.rejected[0]?.errors[0]?.columnId).toBe("gpa");
    expect(result.rejected[1]?.errors[0]?.columnId).toBe("dept");
  });

  test("fill down and tsv roundtrip", () => {
    const filled = fillDown(rows(), cols, {
      startRow: 0,
      startCol: 1,
      endRow: 2,
      endCol: 1,
    });
    expect(filled[2]?.gpa).toBe(3.9);
    expect(toTsv([["a", "b"]])).toBe("a\tb");
  });

  test("duplicate rows", () => {
    expect(duplicateRows(rows(), [0, 2])).toHaveLength(5);
  });
});

describe("keyboard + rtl", () => {
  test("arrow left moves +1 in RTL", () => {
    const cmd = interpretKey(
      {
        key: "ArrowLeft",
        shiftKey: false,
        ctrlKey: false,
        metaKey: false,
        altKey: false,
      },
      false,
      true,
    );
    expect(cmd.action).toBe("move");
    expect(cmd.colDelta).toBe(1);
  });
  test("clamp infinities", () => {
    expect(clampIndex(-Infinity, 9)).toBe(0);
    expect(clampIndex(Infinity, 9)).toBe(9);
  });

  test("Tab + Enter returns to the Tab start column on the next row", () => {
    const base = {
      rowCount: 5,
      colCount: 5,
      editableCols: [1, 2, 3, 4],
      range: null,
    };

    // Start at col 1, Tab to col 2 -> anchor stays 1
    const s1 = resolveCellStep(0, 1, 0, 1, { ...base, tabAnchorCol: null });
    expect(s1).toEqual({
      row: 0,
      col: 2,
      tabAnchorCol: 1,
      pastEnd: false,
      withinRange: false,
    });

    // Tab to col 3 -> anchor stays 1
    const s2 = resolveCellStep(s1.row, s1.col, 0, 1, {
      ...base,
      tabAnchorCol: s1.tabAnchorCol,
    });
    expect(s2).toEqual({
      row: 0,
      col: 3,
      tabAnchorCol: 1,
      pastEnd: false,
      withinRange: false,
    });

    // Enter -> drops to row 1, col 1 and clears anchor
    const s3 = resolveCellStep(s2.row, s2.col, 1, 0, {
      ...base,
      tabAnchorCol: s2.tabAnchorCol,
    });
    expect(s3).toEqual({
      row: 1,
      col: 1,
      tabAnchorCol: null,
      pastEnd: false,
      withinRange: false,
    });
  });

  test("Enter without prior Tab stays in the same column", () => {
    const step = resolveCellStep(1, 3, 1, 0, {
      rowCount: 5,
      colCount: 5,
      editableCols: [1, 2, 3, 4],
      range: null,
      tabAnchorCol: null,
    });
    expect(step).toEqual({
      row: 2,
      col: 3,
      tabAnchorCol: null,
      pastEnd: false,
      withinRange: false,
    });
  });

  test("adapts Enter to horizontal movement after horizontal row entry", () => {
    // User edits (0,1) then (0,2) -> detected as horizontal entry
    let axis = detectEntryAxis(null, 0, 1, null);
    expect(axis).toBeNull();
    axis = detectEntryAxis({ row: 0, col: 1 }, 0, 2, axis);
    expect(axis).toBe("horizontal");

    // User moves to row 1 (col 1), edits it, and presses Enter -> moves horizontally to (1,2)
    const next = resolveCellStep(1, 1, 1, 0, {
      rowCount: 5,
      colCount: 5,
      editableCols: [1, 2, 3],
      range: null,
      tabAnchorCol: null,
      entryAxis: axis,
    });
    expect(next).toEqual({
      row: 1,
      col: 2,
      tabAnchorCol: null,
      pastEnd: false,
      withinRange: false,
    });

    // At end of row 1 (col 3), Enter wraps to (2, 1)
    const wrap = resolveCellStep(1, 3, 1, 0, {
      rowCount: 5,
      colCount: 5,
      editableCols: [1, 2, 3],
      range: null,
      tabAnchorCol: null,
      entryAxis: axis,
    });
    expect(wrap).toEqual({
      row: 2,
      col: 1,
      tabAnchorCol: null,
      pastEnd: false,
      withinRange: false,
    });

    // If user edits vertically in the same column, axis switches back to vertical
    axis = detectEntryAxis({ row: 2, col: 1 }, 3, 1, axis);
    expect(axis).toBe("vertical");
    const down = resolveCellStep(3, 1, 1, 0, {
      rowCount: 5,
      colCount: 5,
      editableCols: [1, 2, 3],
      range: null,
      tabAnchorCol: null,
      entryAxis: axis,
    });
    expect(down).toEqual({
      row: 4,
      col: 1,
      tabAnchorCol: null,
      pastEnd: false,
      withinRange: false,
    });
  });

  test("editor arrow keys commit and move in entry mode, and respect RTL", () => {
    const ev = (key: string) => ({
      key,
      shiftKey: false,
      ctrlKey: false,
      metaKey: false,
      altKey: false,
    });

    // Entry mode: all 4 arrows commit and move
    expect(interpretEditorKey(ev("ArrowDown"), "entry", false)).toEqual({
      action: "commit",
      rowDelta: 1,
      colDelta: 0,
      stepMode: "arrow",
      preventDefault: true,
    });
    expect(interpretEditorKey(ev("ArrowUp"), "entry", false)).toEqual({
      action: "commit",
      rowDelta: -1,
      colDelta: 0,
      stepMode: "arrow",
      preventDefault: true,
    });
    // LTR: ArrowRight moves +1, ArrowLeft moves -1
    expect(interpretEditorKey(ev("ArrowRight"), "entry", false).colDelta).toBe(1);
    expect(interpretEditorKey(ev("ArrowLeft"), "entry", false).colDelta).toBe(-1);
    // RTL: ArrowLeft moves +1, ArrowRight moves -1
    expect(interpretEditorKey(ev("ArrowLeft"), "entry", true).colDelta).toBe(1);
    expect(interpretEditorKey(ev("ArrowRight"), "entry", true).colDelta).toBe(-1);

    // Edit mode (F2 / double-click): Up/Down commit & move, Left/Right stay inside text
    expect(interpretEditorKey(ev("ArrowDown"), "edit", true).action).toBe("commit");
    expect(interpretEditorKey(ev("ArrowLeft"), "edit", true).action).toBe("none");
    expect(interpretEditorKey(ev("ArrowRight"), "edit", true).action).toBe("none");
    expect(interpretEditorKey(ev("F2"), "edit", true).action).toBe("toggleMode");
  });

  test("Enter and Tab cycle inside a multi-cell selection range", () => {
    const opts = {
      rowCount: 10,
      colCount: 10,
      editableCols: [1, 2, 3],
      range: { startRow: 1, startCol: 1, endRow: 2, endCol: 2 },
      tabAnchorCol: null,
    };

    // Enter moves down inside range, wrapping to top of next column
    expect(resolveCellStep(1, 1, 1, 0, opts)).toMatchObject({
      row: 2,
      col: 1,
      withinRange: true,
    });
    expect(resolveCellStep(2, 1, 1, 0, opts)).toMatchObject({
      row: 1,
      col: 2,
      withinRange: true,
    });
    expect(resolveCellStep(2, 2, 1, 0, opts)).toMatchObject({
      row: 1,
      col: 1,
      withinRange: true,
    });

    // Shift+Enter moves backward inside range
    expect(resolveCellStep(1, 1, -1, 0, opts)).toMatchObject({
      row: 2,
      col: 2,
      withinRange: true,
    });

    // Tab moves horizontally inside range
    expect(resolveCellStep(1, 2, 0, 1, opts)).toMatchObject({
      row: 2,
      col: 1,
      withinRange: true,
    });
  });
});

describe("virtualization", () => {
  test("keeps a valid row window when the source shrinks at a deep scroll offset", () => {
    const window = visibleWindow(10000, 160, 40, 5, 0);
    expect([window.start, window.end, window.offset]).toEqual([1, 5, 40]);
    expect(visibleWindow(-80, 160, 40, 5, 0).start).toBe(0);
  });

  test("keeps the checkbox pinned when the viewport cannot fit every pinned column", () => {
    const widths = [40, 200, 120];
    expect(countFrozenColumns(widths, 260)).toBe(1);
    expect(countFrozenColumns(widths, 350)).toBe(2);
    const cells = headerLayout(
      widths.map((width, index) => ({
        id: String(index),
        width,
        group: "Students",
        pin: "colPinStart",
      })),
      1,
    ).cells.filter((cell) => cell.kind === "group");
    expect(cells.map(({ colStart, colSpan }) => [colStart, colSpan])).toEqual([
      [0, 1],
      [1, 2],
    ]);
  });
  test("reveals focused columns outside the pinned area in both directions", () => {
    const viewport = { viewportWidth: 400, frozenWidth: 150 };
    expect(
      scrollForColumn({
        ...viewport,
        columnStart: 300,
        columnWidth: 90,
        scrollLeft: -250,
        rtl: true,
      }),
    ).toBe(-150);
    expect(
      scrollForColumn({
        ...viewport,
        columnStart: 500,
        columnWidth: 80,
        scrollLeft: 50,
        rtl: false,
      }),
    ).toBe(180);
    expect(
      scrollForColumn({
        ...viewport,
        columnStart: 200,
        columnWidth: 300,
        scrollLeft: 0,
        rtl: false,
      }),
    ).toBe(50);
  });
  test("window for 1e6 rows only visits overscan band", () => {
    const w = visibleWindow(40_000 * 32, 640, 32, 1_000_000, 8);
    expect(w.count).toBeLessThan(50);
    expect(w.start).toBeGreaterThan(39_000);
  });
  test("variable-width columns keep the horizontal offset aligned", () => {
    const sizes = [80, 120, 160, 200, 240];
    const window = visibleVariableWindow(220, 180, sizes, 1);
    expect(window).toEqual({ start: 1, end: 5, offset: 80, count: 4 });
  });
});

describe("grouping", () => {
  test("group and sum", () => {
    const withAgg: OrbisColumnDef[] = [...cols];
    withAgg[1] = { ...cols[1]!, aggregate: "sum" };
    const groups = groupRows(rows(), "dept", withAgg);
    expect(groups.length).toBe(2);
    const cs = groups.find((g) => g.key === "cs")!;
    expect(cs.aggregates.gpa).toBeCloseTo(7.1);
    expect(grandTotals(rows(), withAgg).gpa).toBeCloseTo(9.2);
  });
});

describe("query + coerce", () => {
  test("client query combines filter and sort", () => {
    const out = queryClient(rows(), cols, {
      filter: {
        logic: "AND",
        conditions: [{ field: "active", operator: "bool_true" }],
      },
      sorts: [{ id: "gpa", dir: "desc" }],
    });
    expect(out.map((r) => r.id)).toEqual(["1", "2"]);
  });
  test("coerce integer/date/boolean", () => {
    expect(coerceValue("12", { type: "integer" }).value).toBe(12);
    expect(coerceValue("nope", { type: "integer" }).ok).toBe(false);
    expect(coerceValue("نعم", { type: "boolean" }).value).toBe(true);
    expect(coerceValue("not a date", { type: "datetime" }).ok).toBe(false);
    const datetime = coerceValue("2024-12-31T12:30", { type: "datetime" });
    expect(datetime.ok).toBe(true);
    expect(Number.isNaN(Date.parse(String(datetime.value)))).toBe(false);
    expect(coerceValue("31/12/2024", { type: "date" }).value).toBe(
      "2024-12-31",
    );
  });
  test("coerce validates against the current row", () => {
    const column: OrbisColumnDef = {
      id: "score",
      type: "decimal",
      validate: (value, row) =>
        Number(value) <= Number(row.maxScore) || "Score exceeds the limit",
    };
    expect(coerceValue("8.5", column, { maxScore: 10 })).toEqual({
      ok: true,
      value: 8.5,
    });
    expect(coerceValue("12", column, { maxScore: 10 })).toEqual({
      ok: false,
      value: "12",
      message: "Score exceeds the limit",
    });
  });
  test("date comparisons and accessors are supported in column filters", () => {
    const data = [
      { id: "a", start: "2024-01-01" },
      { id: "b", start: "2024-03-01" },
      { id: "c", start: "2024-06-01" },
    ];
    const dateColumn: OrbisColumnDef[] = [
      { id: "start", type: "date", accessor: (row) => row.start },
    ];
    expect(
      applyFilter(
        data,
        {
          logic: "AND",
          conditions: [{ field: "start", operator: "gt", value: "2024-02-01" }],
        },
        dateColumn,
      ).map((row) => row.id),
    ).toEqual(["b", "c"]);
  });
});

describe("pointer hit testing", () => {
  test("row and column from offsets", () => {
    expect(rowFromOffset(36 + 72, 36, 36, 10)).toBe(2);
    expect(colFromOffset(150, [100, 80, 120])).toBe(1);
    expect(colFromOffset(0, [100, 80])).toBe(0);
    expect(colFromOffset(999, [100, 80])).toBe(1);
  });
  test("autoscroll at edges", () => {
    expect(autoscrollDelta(10, 0, 400)).toBeLessThan(0);
    expect(autoscrollDelta(390, 0, 400)).toBeGreaterThan(0);
    expect(autoscrollDelta(200, 0, 400)).toBe(0);
  });
});

describe("performance", () => {
  test("filter/sort/select 100k rows under budget", () => {
    const n = 100_000;
    const data: GridRow[] = new Array(n);
    for (let i = 0; i < n; i++) {
      data[i] = {
        id: String(i),
        name: `S${i}`,
        gpa: (i % 400) / 100,
        dept: i % 2 ? "cs" : "ee",
        active: i % 3 !== 0,
      };
    }
    const t0 = performance.now();
    const filtered = applyFilter(data, {
      logic: "AND",
      conditions: [
        { field: "dept", operator: "eq", value: "cs" },
        { field: "gpa", operator: "gt", value: 3.5 },
        { field: "active", operator: "bool_true" },
      ],
    });
    const t1 = performance.now();
    applySort(
      filtered,
      [
        { id: "gpa", dir: "desc" },
        { id: "name", dir: "asc" },
      ],
      cols,
    );
    const t2 = performance.now();
    const sel = selectScope([], "filtered", [], []);
    expect(selectedCount(sel, filtered.length)).toBe(filtered.length);
    const filterMs = t1 - t0;
    const sortMs = t2 - t1;
    const searchStart = performance.now();
    const searchResults = applyFilter(
      data,
      { logic: "AND", conditions: [], search: "S99999" },
      cols,
    );
    const searchMs = performance.now() - searchStart;
    expect(filterMs).toBeLessThan(250);
    expect(sortMs).toBeLessThan(250);
    expect(searchMs).toBeLessThan(250);
    expect(searchResults).toHaveLength(1);
    expect(filtered.length).toBeGreaterThan(0);
  });
});

describe("worksheet layout", () => {
  test("Excel paste keeps empty student rows and decimal commas", () => {
    expect(parseClipboard("10,5\r\n\r\n12\r\n", "\t")).toEqual([
      ["10,5"],
      [""],
      ["12"],
    ]);
    expect(parseClipboard(toTsv([["two\nlines", "5"]]))).toEqual([
      ["two\nlines", "5"],
    ]);
  });

  test("merges subject and term headers while spanning pinned identity columns", () => {
    const { rowCount, cells } = headerLayout([
      { id: "student", pin: "colPinStart" },
      { id: "oral", groupPath: ["Math", "Term 1"], subHeader: "20" },
      { id: "written", groupPath: ["Math", "Term 1"], subHeader: "30" },
      { id: "term2", groupPath: ["Math", "Term 2"], subHeader: "50" },
      { id: "total", emphasis: true },
    ]);
    expect(rowCount).toBe(4);
    expect(cells.find((cell) => cell.colStart === 0)?.rowSpan).toBe(4);
    expect(
      cells
        .filter((cell) => cell.kind === "group")
        .map((cell) => [cell.label, cell.colSpan]),
    ).toEqual([
      ["Math", 3],
      ["Term 1", 2],
      ["Term 2", 1],
    ]);
    expect(
      cells.filter((cell) => cell.kind === "sub").map((cell) => cell.label),
    ).toEqual(["20", "30", "50"]);
  });

  test("grouped headers do not span the boundary between pinned and scrolling columns", () => {
    const { cells } = headerLayout([
      { id: "name", group: "Record", pin: "colPinStart" },
      { id: "status", group: "Record", pin: "colPinStart" },
      { id: "score", group: "Record" },
      { id: "notes", group: "Record" },
    ]);
    expect(
      cells
        .filter((cell) => cell.kind === "group")
        .map(({ colStart, colSpan }) => [colStart, colSpan]),
    ).toEqual([
      [0, 2],
      [2, 2],
    ]);
  });

  test("fixed rosters retain invalid target values and never append students", () => {
    const roster = [{ score: 7 }];
    const { rows: pasted, result } = applyPaste({
      rows: roster,
      columns: [{ id: "score", validate: (value) => Number(value) < 11 }],
      startRow: 0,
      startCol: 0,
      matrix: [["20"], ["8"]],
      fixedRows: true,
    });
    expect(pasted).toHaveLength(1);
    expect(pasted[0]?.score).toBe(7);
    expect(result.rejectedCount).toBe(1);
  });
});
