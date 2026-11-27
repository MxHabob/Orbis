import {
  columnHeader,
  columnId,
  isColumnPinnedLeft,
  type OrbisColumnDef,
} from "../types";

export interface HeaderCellModel {
  colStart: number;
  colSpan: number;
  rowStart: number;
  rowSpan: number;
  col: OrbisColumnDef;
  label: string;
  kind: "group" | "leaf" | "sub" | "span";
  key: string;
}

function groupsOf(col: OrbisColumnDef): readonly string[] {
  return col.groupPath ?? (col.group ? [col.group] : []);
}

export function headerLayout(columns: OrbisColumnDef[], frozenCount?: number) {
  const pinned = (index: number) =>
    frozenCount === undefined
      ? isColumnPinnedLeft(columns[index]!)
      : index < frozenCount;
  const paths = columns.map(groupsOf);
  const depth = paths.reduce((max, path) => Math.max(max, path.length), 0);
  const hasLimits = columns.some((col) => col.subHeader != null);
  const rowCount = depth + 1 + Number(hasLimits);
  const cells: HeaderCellModel[] = [];

  for (let index = 0; index < columns.length; index++) {
    const col = columns[index]!;
    const path = paths[index]!;
    const key = columnId(col) || String(index);
    const span = Math.max(1, Math.min(rowCount, col.headerRowSpan ?? 1));
    for (let level = 0; span === 1 && level < path.length; level++) {
      const sharesGroup = (other: number) => {
        const next = columns[other];
        return (
          next &&
          (next.headerRowSpan ?? 1) <= 1 &&
          pinned(other) === pinned(index) &&
          path
            .slice(0, level + 1)
            .every((label, i) => paths[other]?.[i] === label)
        );
      };
      if (sharesGroup(index - 1)) continue;
      let end = index + 1;
      while (sharesGroup(end)) end++;
      cells.push({
        colStart: index,
        colSpan: end - index,
        rowStart: level,
        rowSpan: 1,
        col,
        label: path[level]!,
        kind: "group",
        key: `group-${level}-${key}`,
      });
    }

    const leafStart = span > 1 ? 0 : path.length;
    const hasOwnLimit = hasLimits && col.subHeader != null && span < rowCount;
    const leafEnd = rowCount - Number(hasOwnLimit);
    cells.push({
      colStart: index,
      colSpan: 1,
      rowStart: leafStart,
      rowSpan: leafEnd - leafStart,
      col,
      label: columnHeader(col),
      kind: leafEnd - leafStart > 1 ? "span" : "leaf",
      key: `leaf-${key}`,
    });
    if (hasOwnLimit) {
      cells.push({
        colStart: index,
        colSpan: 1,
        rowStart: rowCount - 1,
        rowSpan: 1,
        col,
        label: col.subHeader ?? "",
        kind: "sub",
        key: `sub-${key}`,
      });
    }
  }

  return { rowCount, cells };
}
