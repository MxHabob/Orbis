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
        colDelta: e.shiftKey ? -dir : dir,
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
      colDelta: e.shiftKey ? -dir : dir,
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
