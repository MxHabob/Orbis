import { saveAs } from "file-saver";

type CsvRow = Record<string, string | number | null | undefined>;

function escapeCsvCell(value: string): string {
  if (value.includes(",") || value.includes('"') || value.includes("\n")) {
    return `"${value.replace(/"/g, '""')}"`;
  }
  return value;
}

export function exportToCsv(
  rows: CsvRow[],
  options: { filename?: string } = {},
) {
  if (rows.length === 0) {
    saveAs(new Blob([""], { type: "text/csv;charset=utf-8" }), options.filename ?? "export.csv");
    return;
  }

  const headers = Object.keys(rows[0] ?? {});
  const lines = [
    headers.map(escapeCsvCell).join(","),
    ...rows.map((row) =>
      headers.map((header) => escapeCsvCell(String(row[header] ?? ""))).join(","),
    ),
  ];

  const blob = new Blob([lines.join("\n")], { type: "text/csv;charset=utf-8" });
  saveAs(blob, options.filename ?? "export.csv");
}

export function downloadBase64(base64: string, filename: string) {
  const binary = atob(base64);
  const bytes = Uint8Array.from(binary, (char) => char.charCodeAt(0));
  saveAs(new Blob([bytes]), filename);
}

export async function generateCanvasPdf(options: {
  filename: string;
  selector: string;
  scale?: number;
}) {
  const [{ default: html2canvas }, { jsPDF }] = await Promise.all([
    import("html2canvas"),
    import("jspdf"),
  ]);

  const element = document.querySelector(options.selector);
  if (!element || !(element instanceof HTMLElement)) {
    throw new Error(`Element not found for selector: ${options.selector}`);
  }

  const canvas = await html2canvas(element, {
    scale: options.scale ?? 2,
    useCORS: true,
    backgroundColor: "#ffffff",
  });

  const imgData = canvas.toDataURL("image/png");
  const pdf = new jsPDF({
    orientation: canvas.width >= canvas.height ? "landscape" : "portrait",
    unit: "px",
    format: [canvas.width, canvas.height],
  });

  pdf.addImage(imgData, "PNG", 0, 0, canvas.width, canvas.height);
  pdf.save(options.filename);
}

export async function parseCsv<T extends Record<string, unknown>>(
  file: File,
): Promise<{ data: T[]; meta: { fields: string[] } }> {
  const text = await file.text();
  const lines = text
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);

  if (lines.length === 0) {
    return { data: [], meta: { fields: [] } };
  }

  const headers = parseCsvLine(lines[0]!);
  const data = lines.slice(1).map((line) => {
    const cells = parseCsvLine(line);
    return headers.reduce<Record<string, unknown>>((row, header, index) => {
      row[header] = cells[index] ?? "";
      return row;
    }, {}) as T;
  });

  return { data, meta: { fields: headers } };
}

function parseCsvLine(line: string): string[] {
  const result: string[] = [];
  let current = "";
  let inQuotes = false;

  for (const char of line) {
    if (char === '"') {
      inQuotes = !inQuotes;
      continue;
    }
    if (char === "," && !inQuotes) {
      result.push(current);
      current = "";
      continue;
    }
    current += char;
  }

  result.push(current);
  return result;
}
