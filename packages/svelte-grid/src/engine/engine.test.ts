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
import { applyPaste, parseClipboard, toTsv, fillDown, duplicateRows } from "./clipboard";
import { interpretKey, clampIndex } from "./keyboard";
import { visibleWindow } from "./virtual";
import { groupRows, grandTotals } from "./grouping";
import { queryClient } from "./query";
import { autoscrollDelta, colFromOffset, rowFromOffset } from "./pointer";
import { coerceValue } from "./coerce";
import type { GridRow, OrbisColumnDef } from "../types";

const cols: OrbisColumnDef[] = [
  { id: "name", header: "Name", type: "text" },
  { id: "gpa", header: "GPA", type: "decimal" },
  { id: "dept", header: "Dept", type: "enum", options: [{ value: "cs", label: "CS" }] },
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
    expect(applyFilter(rows(), { logic: "AND", conditions: [], search: "ali" })).toHaveLength(1);
  });
});

describe("sort", () => {
  test("multi-column stable sort", () => {
    const data = [
      { id: "a", dept: "cs", gpa: 3, name: "B" },
      { id: "b", dept: "cs", gpa: 3, name: "A" },
      { id: "c", dept: "ee", gpa: 4, name: "C" },
    ];
    const sorted = applySort(data, [
      { id: "dept", dir: "asc" },
      { id: "gpa", dir: "desc" },
      { id: "name", dir: "asc" },
    ], cols);
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
    const r = normalizeRange({ startRow: 4, startCol: 2, endRow: 1, endCol: 0 });
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
      createEmptyRow: () => ({ id: crypto.randomUUID(), name: "", gpa: null, dept: "cs", active: true }),
    });
    expect(result.acceptedCount).toBe(1);
    expect(result.rejectedCount).toBe(2);
    expect(result.rejected[0]?.errors[0]?.columnId).toBe("gpa");
    expect(result.rejected[1]?.errors[0]?.columnId).toBe("dept");
  });

  test("fill down and tsv roundtrip", () => {
    const filled = fillDown(rows(), cols, { startRow: 0, startCol: 1, endRow: 2, endCol: 1 });
    expect(filled[2]?.gpa).toBe(3.9);
    expect(toTsv([["a", "b"]])).toBe("a\tb");
  });

  test("duplicate rows", () => {
    expect(duplicateRows(rows(), [0, 2])).toHaveLength(5);
  });
});

describe("keyboard + rtl", () => {
  test("arrow left moves +1 in RTL", () => {
    const cmd = interpretKey({ key: "ArrowLeft", shiftKey: false, ctrlKey: false, metaKey: false, altKey: false }, false, true);
    expect(cmd.action).toBe("move");
    expect(cmd.colDelta).toBe(1);
  });
  test("clamp infinities", () => {
    expect(clampIndex(-Infinity, 9)).toBe(0);
    expect(clampIndex(Infinity, 9)).toBe(9);
  });
});

describe("virtualization", () => {
  test("window for 1e6 rows only visits overscan band", () => {
    const w = visibleWindow(40_000 * 32, 640, 32, 1_000_000, 8);
    expect(w.count).toBeLessThan(50);
    expect(w.start).toBeGreaterThan(39_000);
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
      filter: { logic: "AND", conditions: [{ field: "active", operator: "bool_true" }] },
      sorts: [{ id: "gpa", dir: "desc" }],
    });
    expect(out.map((r) => r.id)).toEqual(["1", "2"]);
  });
  test("coerce integer/date/boolean", () => {
    expect(coerceValue("12", { type: "integer" }).value).toBe(12);
    expect(coerceValue("nope", { type: "integer" }).ok).toBe(false);
    expect(coerceValue("نعم", { type: "boolean" }).value).toBe(true);
    expect(coerceValue("31/12/2024", { type: "date" }).value).toBe("2024-12-31");
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
      data[i] = { id: String(i), name: `S${i}`, gpa: (i % 400) / 100, dept: i % 2 ? "cs" : "ee", active: i % 3 !== 0 };
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
    applySort(filtered, [{ id: "gpa", dir: "desc" }, { id: "name", dir: "asc" }], cols);
    const t2 = performance.now();
    const sel = selectScope([], "filtered", [], []);
    expect(selectedCount(sel, filtered.length)).toBe(filtered.length);
    const filterMs = t1 - t0;
    const sortMs = t2 - t1;
    expect(filterMs).toBeLessThan(250);
    expect(sortMs).toBeLessThan(250);
    expect(filtered.length).toBeGreaterThan(0);
  });
});
