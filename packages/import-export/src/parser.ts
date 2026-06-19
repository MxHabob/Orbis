import { SCHEMAS, ImportSchema } from "./schemas";

export interface ParseResult<T> {
  data: T[];
  errors: string[];
}

export class OrbisCSVParser {
  /**
   * Parse a CSV string into strongly-typed objects based on a registered schema.
   */
  static parse<T>(
    csvText: string,
    schemaKey: "student" | "grades",
  ): ParseResult<T> {
    const schema = SCHEMAS[schemaKey];
    if (!schema) {
      return { data: [], errors: [`Schema "${schemaKey}" is not registered.`] };
    }

    const errors: string[] = [];
    const data: T[] = [];

    // Clean up carriage returns and split into lines
    const lines = csvText
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter((line) => line.length > 0);

    if (lines.length === 0) {
      return { data: [], errors: ["CSV file is empty."] };
    }

    // Parse header row
    const headers = this.parseCSVLine(lines[0]!);
    const headerIndexMap: Record<string, number> = {};

    // Map schema fields to header column indices
    for (const field of schema.fields) {
      let foundIndex = -1;

      for (let i = 0; i < headers.length; i++) {
        const header = headers[i]!.trim().toLowerCase();
        const label = field.label.toLowerCase();
        const labelAr = field.labelAr?.toLowerCase();
        const key = field.key.toLowerCase();

        if (header === label || header === labelAr || header === key) {
          foundIndex = i;
          break;
        }
      }

      if (foundIndex !== -1) {
        headerIndexMap[field.key] = foundIndex;
      } else if (field.required) {
        errors.push(
          `Missing required column header: "${field.label}" (or "${field.labelAr || ""}")`,
        );
      }
    }

    if (errors.length > 0) {
      return { data: [], errors };
    }

    // Parse data rows
    for (let rowIndex = 1; rowIndex < lines.length; rowIndex++) {
      const line = lines[rowIndex]!;
      const cells = this.parseCSVLine(line);

      // Skip empty lines
      if (cells.length === 0 || (cells.length === 1 && cells[0] === "")) {
        continue;
      }

      const rowObj: any = {};
      let rowValid = true;

      for (const field of schema.fields) {
        const colIndex = headerIndexMap[field.key];

        // If optional field index is missing
        if (colIndex === undefined || colIndex >= cells.length) {
          if (field.required) {
            errors.push(
              `Row ${rowIndex + 1}: Missing required field "${field.label}"`,
            );
            rowValid = false;
          } else {
            rowObj[field.key] = null;
          }
          continue;
        }

        const rawValue = cells[colIndex]!.trim();

        if (rawValue === "") {
          if (field.required) {
            errors.push(
              `Row ${rowIndex + 1}: Required field "${field.label}" is empty`,
            );
            rowValid = false;
          } else {
            rowObj[field.key] = null;
          }
          continue;
        }

        // Validate type
        let parsedValue: any = rawValue;
        if (field.type === "number") {
          const num = Number(rawValue);
          if (isNaN(num)) {
            errors.push(
              `Row ${rowIndex + 1}: Column "${field.label}" must be a valid number (got "${rawValue}")`,
            );
            rowValid = false;
          } else {
            parsedValue = num;
          }
        } else if (field.type === "email") {
          const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
          if (!emailRegex.test(rawValue)) {
            errors.push(
              `Row ${rowIndex + 1}: Column "${field.label}" must be a valid email (got "${rawValue}")`,
            );
            rowValid = false;
          }
        }

        if (rowValid) {
          rowObj[field.key] = parsedValue;
        }
      }

      if (rowValid) {
        data.push(rowObj as T);
      }
    }

    return { data, errors };
  }

  /**
   * Generate a CSV string from an array of objects and specific keys.
   */
  static generate<T>(
    data: T[],
    schemaKey: "student" | "grades",
    language: "en" | "ar" = "en",
  ): string {
    const schema = SCHEMAS[schemaKey];
    if (!schema) {
      throw new Error(`Schema "${schemaKey}" is not registered.`);
    }

    const headers = schema.fields.map((f) => {
      if (language === "ar" && f.labelAr) {
        return this.escapeCSVCell(f.labelAr);
      }
      return this.escapeCSVCell(f.label);
    });

    const csvLines = [headers.join(",")];

    for (const row of data) {
      const lineCells = schema.fields.map((field) => {
        const val = (row as any)[field.key];
        if (val === undefined || val === null) {
          return "";
        }
        return this.escapeCSVCell(String(val));
      });
      csvLines.push(lineCells.join(","));
    }

    return csvLines.join("\n");
  }

  /**
   * Helper to parse a single CSV line, respecting double quotes and escaped commas.
   */
  private static parseCSVLine(line: string): string[] {
    const result: string[] = [];
    let currentCell = "";
    let inQuotes = false;

    for (let i = 0; i < line.length; i++) {
      const char = line[i];

      if (char === '"') {
        // Toggle quotes flag
        inQuotes = !inQuotes;
      } else if (char === "," && !inQuotes) {
        // Cell boundary
        result.push(currentCell);
        currentCell = "";
      } else {
        currentCell += char;
      }
    }
    result.push(currentCell);
    return result;
  }

  /**
   * Helper to escape special characters in CSV cell output.
   */
  private static escapeCSVCell(value: string): string {
    if (value.includes(",") || value.includes('"') || value.includes("\n")) {
      const escaped = value.replace(/"/g, '""');
      return `"${escaped}"`;
    }
    return value;
  }
}
