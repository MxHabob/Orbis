import type { GridRow, OrbisColumnDef } from "../types";
import { getCellValue } from "../types";

export function formatCell(
  row: GridRow,
  col: OrbisColumnDef,
  locale = "en",
): string {
  const value = getCellValue(row, col);
  if (value == null || value === "") return "";
  const type = col.type;

  if (type === "boolean") return value === true || value === 1 ? "✓" : "";
  if (type === "percentage" && typeof value === "number") {
    return `${value.toLocaleString(locale, { maximumFractionDigits: 1 })}%`;
  }
  if (type === "currency" && typeof value === "number") {
    try {
      return new Intl.NumberFormat(locale, {
        style: "currency",
        currency: col.currency || "USD",
        maximumFractionDigits: 2,
      }).format(value);
    } catch {
      return value.toFixed(2);
    }
  }
  if (
    (type === "number" || type === "decimal" || type === "integer") &&
    typeof value === "number"
  ) {
    return value.toLocaleString(locale);
  }
  if (
    (type === "date" || type === "datetime") &&
    (typeof value === "string" || value instanceof Date)
  ) {
    const d = value instanceof Date ? value : new Date(value);
    if (Number.isNaN(d.getTime())) return String(value);
    return type === "datetime"
      ? d.toLocaleString(locale)
      : d.toLocaleDateString(locale);
  }
  if (
    (type === "enum" || type === "status" || type === "badge") &&
    col.options?.length
  ) {
    const hit = col.options.find((o) => String(o.value) === String(value));
    return hit?.label ?? String(value);
  }
  if (Array.isArray(value)) {
    return value
      .map((v) =>
        typeof v === "object" && v
          ? v.name || v.label || v.title || JSON.stringify(v)
          : String(v),
      )
      .join(", ");
  }
  if (typeof value === "object" && value !== null) {
    const obj = value as Record<string, unknown>;
    if (obj.name != null) return String(obj.name);
    if (obj.title != null) return String(obj.title);
    if (obj.label != null) return String(obj.label);
    if (locale === "ar" && obj.name_ar != null) return String(obj.name_ar);
    if (obj.name_en != null) return String(obj.name_en);
    if (locale === "ar" && obj.ar != null) return String(obj.ar);
    if (obj.en != null) return String(obj.en);
  }
  return String(value);
}

export function defaultAlign(col: OrbisColumnDef): "start" | "center" | "end" {
  if (col.align) return col.align;
  switch (col.type) {
    case "number":
    case "integer":
    case "decimal":
    case "currency":
    case "percentage":
      return "end";
    case "boolean":
    case "status":
    case "badge":
      return "center";
    default:
      return "start";
  }
}
