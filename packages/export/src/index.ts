import { saveAs } from "file-saver";
import html2canvas from "html2canvas";
import type { jsPDF } from "jspdf";
import Papa from "papaparse";

export * from "./excel-export";

export type ExportCsvOptions = {
  filename?: string;
};

export type ImportCsvResult<T> = {
  data: T[];
  meta: Papa.ParseMeta;
  errors: Papa.ParseError[];
};

export type ImportCsvOptions = Papa.ParseConfig;

export function downloadBase64(
  data: string,
  filename: string,
  mimeType: string,
) {
  const byteCharacters = atob(data);
  const byteNumbers = new Array(byteCharacters.length);
  for (let i = 0; i < byteCharacters.length; i++) {
    byteNumbers[i] = byteCharacters.charCodeAt(i);
  }
  const byteArray = new Uint8Array(byteNumbers);
  const blob = new Blob([byteArray], { type: mimeType });

  const url = window.URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  window.URL.revokeObjectURL(url);
}

export function exportToCsv<T>(data: T[], options?: ExportCsvOptions) {
  if (!data || data.length === 0) {
    throw new Error("No data to export");
  }

  const csv = Papa.unparse(data);
  const blob = new Blob(["\uFEFF" + csv], { type: "text/csv;charset=utf-8;" });

  const filename =
    options?.filename || `export_${new Date().toISOString()}.csv`;
  saveAs(blob, filename);
}

export function parseCsv<T>(
  file: File | string,
  options?: ImportCsvOptions,
): Promise<ImportCsvResult<T>> {
  return new Promise((resolve, reject) => {
    Papa.parse(file, {
      header: true,
      skipEmptyLines: true,
      ...options,
      complete: (results) => {
        resolve(results as ImportCsvResult<T>);
      },
      error: (error) => {
        reject(error);
      },
    });
  });
}

// PDF EXPORT LOGIC FROM DASHBOARD

// Paper sizes in mm for PDF generation
const PDF_PAPER_SIZES: Record<string, { width: number; height: number }> = {
  a4: { width: 210, height: 297 },
  a5: { width: 148, height: 210 },
  letter: { width: 216, height: 279 },
};

export interface CanvasPdfOptions {
  filename?: string;
  quality?: number;
  scale?: number;
  padding?: number;
  theme?: string;
  selector?: string;
  paperSize?: string;
  orientation?: "portrait" | "landscape";
}

function getCanvasContent(selector?: string): HTMLElement | null {
  const selectors = selector
    ? [selector]
    : ["[data-canvas-content]", "#main-content", "main"];

  for (const sel of selectors) {
    const element = document.querySelector(sel) as HTMLElement;
    if (element && (element.offsetHeight > 0 || element.scrollHeight > 0)) {
      return element;
    }
  }

  console.warn("Canvas content not found");
  return null;
}

export async function generateCanvasPdf(
  options: CanvasPdfOptions = {},
): Promise<void> {
  const {
    filename = "report.pdf",
    quality = 0.95,
    scale = 2,
    padding = 0,
    theme,
    selector,
    paperSize = "a4",
    orientation = "portrait",
  } = options;

  const resolvedTheme =
    theme === "system" || !theme
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light"
      : theme;

  const backgroundColor = resolvedTheme === "dark" ? "#0c0c0c" : "#ffffff";
  const backgroundRgb =
    resolvedTheme === "dark"
      ? { r: 12, g: 12, b: 12 }
      : { r: 255, g: 255, b: 255 };

  try {
    const canvasContent = getCanvasContent(selector);
    if (!canvasContent) {
      throw new Error("Content to export not found");
    }

    const canvas = await html2canvas(canvasContent, {
      scale,
      backgroundColor,
      useCORS: true,
      allowTaint: true,
      logging: false,
      removeContainer: true,
      imageTimeout: 0,
      scrollX: 0,
      scrollY: 0,
      width: canvasContent.offsetWidth,
      height: canvasContent.offsetHeight,
      windowWidth: canvasContent.offsetWidth,
      windowHeight: canvasContent.offsetHeight,
      onclone: (clonedDoc) => {
        // Critical for Arabic/RTL: Ensure the cloned document has the correct direction
        const dir = document.documentElement.dir || document.body.dir || "rtl";
        clonedDoc.documentElement.dir = dir;
        clonedDoc.body.dir = dir;

        const element = clonedDoc.querySelector(
          selector || "[data-canvas-content]",
        ) as HTMLElement;
        if (element) {
          // Isolate the element by hiding everything else
          Array.from(clonedDoc.body.children).forEach((child: any) => {
            if (child instanceof HTMLElement && !child.contains(element)) {
              child.style.display = "none";
            }
          });

          // Reset positioning to 0,0 of the canvas
          element.style.position = "absolute";
          element.style.top = "0";
          element.style.left = "0";
          element.style.margin = "0";
          element.style.padding = "0";
          element.style.transform = "none";
          element.style.visibility = "visible";
          element.style.opacity = "1";

          // Force body and document to match the element's width to fix percentage-based offsets
          clonedDoc.body.style.width = `${canvasContent.offsetWidth}px`;
          clonedDoc.body.style.height = `${canvasContent.offsetHeight}px`;
          clonedDoc.body.style.margin = "0";
          clonedDoc.body.style.padding = "0";
          clonedDoc.body.style.overflow = "hidden";
          clonedDoc.body.style.backgroundColor = backgroundColor;
        }
      },
    });

    const { jsPDF } = await import("jspdf");
    const imgData = canvas.toDataURL("image/jpeg", quality);

    const paper = PDF_PAPER_SIZES[paperSize] || PDF_PAPER_SIZES.a4!;
    const pageWidth = orientation === "portrait" ? paper!.width : paper!.height;
    const pageHeight =
      orientation === "portrait" ? paper!.height : paper!.width;

    const pdf = new jsPDF({
      orientation,
      unit: "mm",
      format: [pageWidth, pageHeight],
    });

    pdf.setFillColor(backgroundRgb.r, backgroundRgb.g, backgroundRgb.b);
    pdf.rect(0, 0, pageWidth, pageHeight, "F");

    pdf.addImage(
      imgData,
      "JPEG",
      padding,
      padding,
      pageWidth - padding * 2,
      pageHeight - padding * 2,
    );

    pdf.save(filename);
  } catch (error) {
    console.error("Error generating PDF:", error);
    throw error;
  }
}

export async function generateCanvasPdfBlob(
  options: CanvasPdfOptions = {},
): Promise<Blob> {
  const {
    quality = 0.95,
    scale = 2,
    padding = 0,
    theme,
    selector,
    paperSize = "a4",
    orientation = "portrait",
  } = options;

  const resolvedTheme =
    theme === "system" || !theme
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light"
      : theme;

  const backgroundColor = resolvedTheme === "dark" ? "#0c0c0c" : "#ffffff";
  const backgroundRgb =
    resolvedTheme === "dark"
      ? { r: 12, g: 12, b: 12 }
      : { r: 255, g: 255, b: 255 };

  try {
    const canvasContent = getCanvasContent(selector);
    if (!canvasContent) {
      throw new Error("Content to export not found");
    }

    const canvas = await html2canvas(canvasContent, {
      scale,
      backgroundColor,
      useCORS: true,
      allowTaint: true,
      logging: false,
      removeContainer: true,
      imageTimeout: 0,
      width: canvasContent.scrollWidth || canvasContent.offsetWidth,
      height: canvasContent.scrollHeight || canvasContent.offsetHeight,
      windowWidth: canvasContent.scrollWidth,
      foreignObjectRendering: false,
      onclone: (clonedDoc) => {
        const element = clonedDoc.body.querySelector(
          selector || "[data-canvas-content]",
        ) as HTMLElement;
        if (element) {
          element.style.overflow = "visible";
          element.style.maxHeight = "none";
          element.style.transform = "none";
          element.style.margin = "0";

          const parent = element.parentElement;
          if (parent) {
            parent.style.transform = "none";
            parent.style.scale = "1";
          }
        }

        const hideElements = clonedDoc.querySelectorAll(
          '[data-hide-in-pdf="true"]',
        );
        hideElements.forEach((el) => {
          (el as HTMLElement).style.display = "none";
        });
      },
    });

    const { jsPDF } = await import("jspdf");

    const imgData = canvas.toDataURL("image/jpeg", quality);

    const paper = PDF_PAPER_SIZES[paperSize] || PDF_PAPER_SIZES.a4!;
    const pageWidth = orientation === "portrait" ? paper!.width : paper!.height;
    const pageHeight =
      orientation === "portrait" ? paper!.height : paper!.width;

    const pdf = new jsPDF({
      orientation,
      unit: "mm",
      format: [pageWidth, pageHeight],
    });

    pdf.setFillColor(backgroundRgb.r, backgroundRgb.g, backgroundRgb.b);
    pdf.rect(0, 0, pageWidth, pageHeight, "F");

    pdf.addImage(
      imgData,
      "JPEG",
      padding,
      padding,
      pageWidth - padding * 2,
      pageHeight - padding * 2,
    );

    return pdf.output("blob");
  } catch (error) {
    console.error("Error generating PDF blob:", error);
    throw error;
  }
}

// ============================================================
// BULK PDF GENERATION — Multiple pages in a single PDF
// ============================================================

export type BulkPdfOptions = {
  filename?: string;
  quality?: number;
  scale?: number;
  padding?: number;
  orientation?: "portrait" | "landscape";
  paperSize?: string;
  /** Called with (completedCount, totalCount) for progress tracking */
  onProgress?: (completed: number, total: number) => void;
};

/**
 * Captures an array of HTMLElements (e.g. certificate containers),
 * renders each to a canvas, and combines them into a single multi-page PDF.
 * Returns a Blob for download.
 */
export async function generateBulkPdfFromElements(
  elements: HTMLElement[],
  options: BulkPdfOptions = {},
): Promise<Blob> {
  const {
    quality = 0.92,
    scale = 2,
    padding = 0,
    orientation = "portrait",
    paperSize = "a4",
    onProgress,
  } = options;

  if (elements.length === 0) {
    throw new Error("No elements to export");
  }

  const { jsPDF } = await import("jspdf");

  const paper = PDF_PAPER_SIZES[paperSize] || PDF_PAPER_SIZES.a4!;
  const pageWidth = orientation === "portrait" ? paper!.width : paper!.height;
  const pageHeight = orientation === "portrait" ? paper!.height : paper!.width;

  let pdf: InstanceType<typeof jsPDF> | null = null;

  for (let i = 0; i < elements.length; i++) {
    const el = elements[i];
    if (!el) continue;

    const canvas = await html2canvas(el, {
      scale,
      backgroundColor: "#ffffff",
      useCORS: true,
      allowTaint: true,
      logging: false,
      removeContainer: false,
      imageTimeout: 0,
      foreignObjectRendering: false,
    });

    const imgData = canvas.toDataURL("image/jpeg", quality);

    if (i === 0) {
      pdf = new jsPDF({
        orientation,
        unit: "mm",
        format: [pageWidth, pageHeight],
      });
    } else if (pdf) {
      pdf.addPage([pageWidth, pageHeight], orientation);
    }

    if (pdf) {
      pdf.setFillColor(255, 255, 255);
      pdf.rect(0, 0, pageWidth, pageHeight, "F");

      // Fill entire page edge-to-edge
      pdf.addImage(
        imgData,
        "JPEG",
        padding,
        padding,
        pageWidth - padding * 2,
        pageHeight - padding * 2,
      );
    }

    onProgress?.(i + 1, elements.length);
  }

  if (!pdf) {
    throw new Error("Failed to generate PDF");
  }

  return pdf.output("blob");
}

/**
 * Creates a hidden off-screen container for rendering certificate elements.
 * Returns the container element and a cleanup function.
 */
export function createOffscreenContainer(): {
  container: HTMLDivElement;
  cleanup: () => void;
} {
  const container = document.createElement("div");
  container.style.cssText = `
    position: fixed;
    left: -99999px;
    top: 0;
    width: auto;
    height: auto;
    overflow: visible;
    pointer-events: none;
    z-index: -1;
  `;
  document.body.appendChild(container);

  return {
    container,
    cleanup: () => {
      if (container.parentNode) {
        container.parentNode.removeChild(container);
      }
    },
  };
}
