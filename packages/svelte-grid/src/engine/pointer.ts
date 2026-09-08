import { clampIndex } from "./keyboard";

export function autoscrollDelta(pos: number, start: number, end: number, edge = 32, speed = 22): number {
  if (end <= start) return 0;
  if (pos < start + edge) return -speed * Math.min(2, (start + edge - pos) / edge);
  if (pos > end - edge) return speed * Math.min(2, (pos - (end - edge)) / edge);
  return 0;
}

export function rowFromOffset(y: number, headerH: number, rowSize: number, total: number): number {
  if (rowSize <= 0 || total <= 0) return 0;
  return clampIndex(Math.floor((y - headerH) / rowSize), total - 1);
}

export function colFromOffset(x: number, widths: number[]): number {
  if (!widths.length) return 0;
  if (x < 0) return 0;
  let acc = 0;
  for (let i = 0; i < widths.length; i++) {
    acc += widths[i] ?? 0;
    if (x < acc) return i;
  }
  return widths.length - 1;
}
