import type { CellRange } from "../types";
import { normalizeRange } from "./selection";

export type KeyAction =
  | "move"
  | "extend"
  | "edit"
  | "commit"
  | "cancel"
  | "tab"
  | "delete"
  | "copy"
  | "cut"
  | "paste"
  | "selectAll"
  | "home"
  | "end"
  | "page"
  | "fillDown"
  | "fillRight"
  | "none";

export interface KeyCommand {
  action: KeyAction;
  rowDelta: number;
  colDelta: number;
  preventDefault: boolean;
}

const MOD = (e: Pick<KeyboardEvent, "ctrlKey" | "metaKey">) =>
  e.ctrlKey || e.metaKey;

export function interpretKey(
  e: Pick<KeyboardEvent, "key" | "shiftKey" | "ctrlKey" | "metaKey" | "altKey">,
  editing: boolean,
  rtl: boolean,
): KeyCommand {
  const none: KeyCommand = {
    action: "none",
    rowDelta: 0,
    colDelta: 0,
    preventDefault: false,
  };
  const dir = rtl ? -1 : 1;

  if (editing) {
    if (e.key === "Escape")
      return {
        action: "cancel",
        rowDelta: 0,
        colDelta: 0,
        preventDefault: true,
      };
    if (e.key === "Enter")
      return {
        action: "commit",
        rowDelta: e.shiftKey ? -1 : 1,
        colDelta: 0,
        preventDefault: true,
      };
    if (e.key === "Tab")
      return {
        action: "tab",
        rowDelta: 0,
        colDelta: e.shiftKey ? -1 : 1,
        preventDefault: true,
      };
    return none;
  }

  if (MOD(e) && e.key.toLowerCase() === "c")
    return { action: "copy", rowDelta: 0, colDelta: 0, preventDefault: true };
  if (MOD(e) && e.key.toLowerCase() === "x")
    return { action: "cut", rowDelta: 0, colDelta: 0, preventDefault: true };
  if (MOD(e) && e.key.toLowerCase() === "v")
    return { action: "paste", rowDelta: 0, colDelta: 0, preventDefault: true };
  if (MOD(e) && e.key.toLowerCase() === "a")
    return {
      action: "selectAll",
      rowDelta: 0,
      colDelta: 0,
      preventDefault: true,
    };
  if (MOD(e) && e.key.toLowerCase() === "d")
    return {
      action: "fillDown",
      rowDelta: 0,
      colDelta: 0,
      preventDefault: true,
    };
  if (MOD(e) && e.key.toLowerCase() === "r")
    return {
      action: "fillRight",
      rowDelta: 0,
      colDelta: 0,
      preventDefault: true,
    };

  if (e.key === "F2")
    return { action: "edit", rowDelta: 0, colDelta: 0, preventDefault: true };
  if (e.key === "Enter")
    return { action: "edit", rowDelta: 0, colDelta: 0, preventDefault: true };
  if (e.key === "Tab")
    return {
      action: "tab",
      rowDelta: 0,
      colDelta: e.shiftKey ? -1 : 1,
      preventDefault: true,
    };
  if (e.key === "Delete" || e.key === "Backspace") {
    return { action: "delete", rowDelta: 0, colDelta: 0, preventDefault: true };
  }
  if (e.key === "Escape")
    return { action: "cancel", rowDelta: 0, colDelta: 0, preventDefault: true };

  const extend = e.shiftKey;
  const jump = MOD(e);

  switch (e.key) {
    case "ArrowUp":
      return {
        action: extend ? "extend" : "move",
        rowDelta: jump ? -Infinity : -1,
        colDelta: 0,
        preventDefault: true,
      };
    case "ArrowDown":
      return {
        action: extend ? "extend" : "move",
        rowDelta: jump ? Infinity : 1,
        colDelta: 0,
        preventDefault: true,
      };
    case "ArrowLeft":
      return {
        action: extend ? "extend" : "move",
        rowDelta: 0,
        colDelta: jump ? -Infinity * dir : -dir,
        preventDefault: true,
      };
    case "ArrowRight":
      return {
        action: extend ? "extend" : "move",
        rowDelta: 0,
        colDelta: jump ? Infinity * dir : dir,
        preventDefault: true,
      };
    case "Home":
      return {
        action: "home",
        rowDelta: jump ? -Infinity : 0,
        colDelta: jump ? 0 : -Infinity,
        preventDefault: true,
      };
    case "End":
      return {
        action: "end",
        rowDelta: jump ? Infinity : 0,
        colDelta: jump ? 0 : Infinity,
        preventDefault: true,
      };
    case "PageUp":
      return {
        action: "page",
        rowDelta: -1,
        colDelta: 0,
        preventDefault: true,
      };
    case "PageDown":
      return { action: "page", rowDelta: 1, colDelta: 0, preventDefault: true };
    default:
      if (e.key.length === 1 && !MOD(e) && !e.altKey) {
        return {
          action: "edit",
          rowDelta: 0,
          colDelta: 0,
          preventDefault: false,
        };
      }
      return none;
  }
}

export function clampIndex(value: number, max: number): number {
  if (!Number.isFinite(value)) return value < 0 ? 0 : max;
  return Math.min(max, Math.max(0, value));
}

export type EntryAxis = "horizontal" | "vertical";

export interface CellStepOptions {
  rowCount: number;
  colCount: number;
  editableCols: number[];
  range: CellRange | null;
  tabAnchorCol: number | null;
  entryAxis?: EntryAxis | null;
}

/**
 * Infers whether the user is entering data horizontally (across a row) or vertically (down a column)
 * based on consecutive committed edits.
 */
export function detectEntryAxis(
  lastEdit: { row: number; col: number } | null,
  row: number,
  col: number,
  current: EntryAxis | null,
): EntryAxis | null {
  if (!lastEdit) return current;
  if (row === lastEdit.row && col !== lastEdit.col) return "horizontal";
  if (col === lastEdit.col && row !== lastEdit.row) return "vertical";
  return current;
}

export interface CellStepResult {
  row: number;
  col: number;
  tabAnchorCol: number | null;
  pastEnd: boolean;
  withinRange: boolean;
}

/**
 * Computes the next active cell after an edit commit or Tab navigation.
 *
 * Mirrors Excel ergonomics:
 * - Multi-cell selection: Enter steps vertically and Tab steps horizontally within the selected range.
 * - Tab + Enter workflow: horizontal Tab navigation records `tabAnchorCol` so a subsequent Enter
 *   drops to the next row and returns to the column where Tabbing started.
 */
export function resolveCellStep(
  row: number,
  col: number,
  rowDelta: number,
  colDelta: number,
  options: CellStepOptions,
): CellStepResult {
  const { rowCount, colCount, editableCols, range, tabAnchorCol, entryAxis } =
    options;
  const maxRow = Math.max(0, rowCount - 1);
  const maxCol = Math.max(0, colCount - 1);

  // When no active Tab-return anchor is set, adapt Enter/Shift+Enter to horizontal mode
  // if the user's recent edits are horizontal across a row.
  let effectiveRowDelta = rowDelta;
  let effectiveColDelta = colDelta;
  if (
    colDelta === 0 &&
    rowDelta !== 0 &&
    tabAnchorCol === null &&
    entryAxis === "horizontal"
  ) {
    effectiveColDelta = rowDelta > 0 ? 1 : -1;
    effectiveRowDelta = 0;
  }

  if (range && (rowDelta !== 0 || colDelta !== 0)) {
    const n = normalizeRange(range);
    const isMultiCell = n.endRow > n.startRow || n.endCol > n.startCol;
    const inRange =
      row >= n.startRow &&
      row <= n.endRow &&
      col >= n.startCol &&
      col <= n.endCol;

    if (isMultiCell && inRange) {
      let nextRow = row;
      let nextCol = col;
      if (colDelta !== 0) {
        if (colDelta > 0) {
          if (col + 1 <= n.endCol) nextCol = col + 1;
          else if (row + 1 <= n.endRow) {
            nextRow = row + 1;
            nextCol = n.startCol;
          } else {
            nextRow = n.startRow;
            nextCol = n.startCol;
          }
        } else {
          if (col - 1 >= n.startCol) nextCol = col - 1;
          else if (row - 1 >= n.startRow) {
            nextRow = row - 1;
            nextCol = n.endCol;
          } else {
            nextRow = n.endRow;
            nextCol = n.endCol;
          }
        }
      } else {
        if (rowDelta > 0) {
          if (row + 1 <= n.endRow) nextRow = row + 1;
          else if (col + 1 <= n.endCol) {
            nextRow = n.startRow;
            nextCol = col + 1;
          } else {
            nextRow = n.startRow;
            nextCol = n.startCol;
          }
        } else {
          if (row - 1 >= n.startRow) nextRow = row - 1;
          else if (col - 1 >= n.startCol) {
            nextRow = n.endRow;
            nextCol = col - 1;
          } else {
            nextRow = n.endRow;
            nextCol = n.endCol;
          }
        }
      }
      return {
        row: clampIndex(nextRow, maxRow),
        col: clampIndex(nextCol, maxCol),
        tabAnchorCol: null,
        pastEnd: false,
        withinRange: true,
      };
    }
  }

  let nextRow = row + effectiveRowDelta;
  let nextCol = col;
  let nextAnchor: number | null = null;

  if (effectiveColDelta !== 0) {
    const anchor = colDelta !== 0 ? (tabAnchorCol ?? col) : null;
    if (editableCols.length > 0) {
      if (effectiveColDelta > 0) {
        const nextEditable = editableCols.find((index) => index > col);
        if (nextEditable !== undefined) {
          nextCol = nextEditable;
          nextAnchor = anchor;
        } else {
          nextRow++;
          nextCol = editableCols[0]!;
          nextAnchor = null;
        }
      } else {
        let previousEditable: number | undefined;
        for (let index = editableCols.length - 1; index >= 0; index--) {
          if (editableCols[index]! < col) {
            previousEditable = editableCols[index]!;
            break;
          }
        }
        if (previousEditable !== undefined) {
          nextCol = previousEditable;
          nextAnchor = anchor;
        } else {
          nextRow--;
          nextCol = editableCols[editableCols.length - 1]!;
          nextAnchor = null;
        }
      }
    } else if (colCount > 0) {
      if (effectiveColDelta > 0) {
        if (col + 1 < colCount) {
          nextCol = col + 1;
          nextAnchor = anchor;
        } else {
          nextRow++;
          nextCol = 0;
          nextAnchor = null;
        }
      } else {
        if (col - 1 >= 0) {
          nextCol = col - 1;
          nextAnchor = anchor;
        } else {
          nextRow--;
          nextCol = maxCol;
          nextAnchor = null;
        }
      }
    }
  } else if (effectiveRowDelta > 0 && tabAnchorCol !== null) {
    nextCol = tabAnchorCol;
    nextAnchor = null;
  }

  const pastEnd =
    nextRow >= rowCount && (effectiveRowDelta > 0 || effectiveColDelta > 0);
  return {
    row: clampIndex(nextRow, maxRow),
    col: clampIndex(nextCol, maxCol),
    tabAnchorCol: nextAnchor,
    pastEnd,
    withinRange: false,
  };
}
