import type {
  ColumnValidateResult,
  GridRow,
  OrbisColumnDef,
} from "../types";

const TRUE_SET = new Set(["true", "1", "yes", "y", "نعم", "صح"]);
const FALSE_SET = new Set(["false", "0", "no", "n", "لا"]);

export function coerceValue(
  raw: unknown,
  col: OrbisColumnDef,
  row: GridRow = {},
): ColumnValidateResult {
  let value = raw;
  if (col.validate) {
    const result = col.validate(raw, row);
    if (typeof result === "boolean" && !result)
      return { ok: false, value: raw, message: "Invalid value" };
    if (typeof result === "string")
      return { ok: false, value: raw, message: result };
    if (typeof result === "object") {
      if (!result.ok) return result;
      if (result.value !== undefined) value = result.value;
    }
  }

  const type = col.type ?? inferType(col);
  if (value === null || value === undefined || value === "") {
    return type === "boolean"
      ? { ok: true, value: false }
      : { ok: true, value: null };
  }

  const text = String(value).trim();

  switch (type) {
    case "integer": {
      const n = Number(text.replace(/,/g, ""));
      if (!Number.isInteger(n))
        return { ok: false, value, message: "Must be an integer" };
      return { ok: true, value: n };
    }
    case "number":
    case "decimal":
    case "currency":
    case "percentage": {
      const n = Number(text.replace(/[%$,\s]/g, ""));
      if (!Number.isFinite(n))
        return { ok: false, value, message: "Must be a number" };
      return { ok: true, value: n };
    }
    case "boolean": {
      const low = text.toLowerCase();
      if (TRUE_SET.has(low)) return { ok: true, value: true };
      if (FALSE_SET.has(low)) return { ok: true, value: false };
      return { ok: false, value, message: "Must be true or false" };
    }
    case "date": {
      const iso = normalizeDate(text);
      if (!iso) return { ok: false, value, message: "Invalid date" };
      return { ok: true, value: iso };
    }
    case "datetime": {
      const timestamp = Date.parse(text);
      if (!Number.isFinite(timestamp))
        return { ok: false, value, message: "Invalid date and time" };
      return { ok: true, value: new Date(timestamp).toISOString() };
    }
    case "enum":
    case "status":
    case "badge": {
      if (!col.options?.length) return { ok: true, value: text };
      const hit = col.options.find(
        (o) =>
          String(o.value).toLowerCase() === text.toLowerCase() ||
          o.label.toLowerCase() === text.toLowerCase(),
      );
      if (!hit)
        return { ok: false, value, message: "Value is not allowed" };
      return { ok: true, value: hit.value };
    }
    default:
      return { ok: true, value: text };
  }
}

function inferType(col: OrbisColumnDef): NonNullable<OrbisColumnDef["type"]> {
  const editor = typeof col.editor === "string" ? col.editor : "";
  if (editor === "checkbox") return "boolean";
  if (editor === "number") return "number";
  if (editor === "date") return "date";
  return "text";
}

export function normalizeDate(val: string): string | null {
  const clean = val.trim().replace(/\s+/g, "");
  let match = clean.match(/^(\d{1,2})[/\-](\d{1,2})[/\-](\d{4})$/);
  if (match) {
    return `${match[3]}-${match[2]!.padStart(2, "0")}-${match[1]!.padStart(2, "0")}`;
  }
  match = clean.match(/^(\d{4})[/\-](\d{1,2})[/\-](\d{1,2})$/);
  if (match) {
    return `${match[1]}-${match[2]!.padStart(2, "0")}-${match[3]!.padStart(2, "0")}`;
  }
  if (!Number.isNaN(Date.parse(clean))) {
    return new Date(clean).toISOString().slice(0, 10);
  }
  return null;
}
