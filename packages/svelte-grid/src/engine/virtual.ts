import type { VirtualWindow } from "../types";

export function visibleWindow(
  scroll: number,
  viewport: number,
  itemSize: number,
  total: number,
  overscan = 8,
): VirtualWindow {
  if (itemSize <= 0 || total <= 0 || viewport < 0) {
    return { start: 0, end: 0, offset: 0, count: 0 };
  }
  const rawStart = Math.floor(scroll / itemSize);
  const visible = Math.ceil(viewport / itemSize) + 1;
  const start = Math.max(0, rawStart - overscan);
  const end = Math.min(total, rawStart + visible + overscan);
  return {
    start,
    end,
    offset: start * itemSize,
    count: Math.max(0, end - start),
  };
}

export function visibleVariableWindow(
  scroll: number,
  viewport: number,
  itemSizes: readonly number[],
  overscan = 4,
): VirtualWindow {
  const total = itemSizes.length;
  if (total === 0 || viewport < 0) {
    return { start: 0, end: 0, offset: 0, count: 0 };
  }

  const scrollStart = Math.max(0, scroll);
  const scrollEnd = scrollStart + viewport;
  let firstVisible = 0;
  let visibleOffset = 0;
  while (
    firstVisible < total - 1 &&
    visibleOffset + Math.max(0, itemSizes[firstVisible]!) <= scrollStart
  ) {
    visibleOffset += Math.max(0, itemSizes[firstVisible]!);
    firstVisible++;
  }

  const buffer = Math.max(0, Math.floor(overscan));
  const start = Math.max(0, firstVisible - buffer);
  let offset = 0;
  for (let index = 0; index < start; index++) {
    offset += Math.max(0, itemSizes[index]!);
  }

  let end = firstVisible;
  let endOffset = visibleOffset;
  while (end < total && (endOffset < scrollEnd || end === firstVisible)) {
    endOffset += Math.max(0, itemSizes[end]!);
    end++;
  }
  end = Math.min(total, end + buffer);
  return { start, end, offset, count: end - start };
}

export function totalSize(count: number, itemSize: number): number {
  return Math.max(0, count) * Math.max(0, itemSize);
}

export function indexFromOffset(
  scroll: number,
  itemSize: number,
  total: number,
): number {
  if (itemSize <= 0 || total <= 0) return 0;
  return Math.min(total - 1, Math.max(0, Math.floor(scroll / itemSize)));
}
