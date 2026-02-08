
import { saveAs } from "file-saver";
import Papa from "papaparse";
import html2canvas from "html2canvas";
import { type jsPDF } from "jspdf";

export type ExportCsvOptions = {
  filename?: string;
};

export type ImportCsvResult<T> = {
  data: T[];
  meta: Papa.ParseMeta;
  errors: Papa.ParseError[];
};

export type ImportCsvOptions = Papa.ParseConfig;

export function downloadBase64(data: string, filename: string, mimeType: string) {
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
  
  const filename = options?.filename || `export_${new Date().toISOString()}.csv`;
  saveAs(blob, filename);
}

export function parseCsv<T>(
  file: File | string,
  options?: ImportCsvOptions
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

export interface CanvasPdfOptions {
  filename?: string;
  quality?: number;
  scale?: number;
  padding?: number;
  theme?: string;
  selector?: string; // Optional custom selector to override the default attribute search
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
    padding = 10,
    theme,
    selector,
  } = options;

  // Resolve theme
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

    const elementHeight = canvasContent.scrollHeight || canvasContent.offsetHeight;
    const extraHeight = 100;

    const canvas = await html2canvas(canvasContent, {
      scale,
      backgroundColor,
      useCORS: true,
      allowTaint: true,
      logging: false,
      removeContainer: true, // Clean up clone
      imageTimeout: 0,
      height: elementHeight + extraHeight,
      windowWidth: canvasContent.scrollWidth, // Ensure we capture horizontal scroll content if any
      foreignObjectRendering: false, // More stable across browsers
      onclone: (clonedDoc) => {
        const element = clonedDoc.body.querySelector(selector || "[data-canvas-content]") as HTMLElement;
        if (element) {
           element.style.height = "auto";
           element.style.overflow = "visible";
           element.style.maxHeight = "none";
        }
        
        // Hide elements marked to hide
        const hideElements = clonedDoc.querySelectorAll('[data-hide-in-pdf="true"]');
        hideElements.forEach((el) => {
          (el as HTMLElement).style.display = "none";
        });
      },
    });

    // Create PDF
    const { jsPDF } = await import("jspdf");
    
    const imgData = canvas.toDataURL("image/jpeg", quality);
    const imgWidth = canvas.width;
    const imgHeight = canvas.height;

    // A4
    const a4Width = 210;
    const a4Height = 297;

    const pdfScale = (a4Width - padding * 2) / imgWidth;
    const scaledWidth = imgWidth * pdfScale;
    const scaledHeight = imgHeight * pdfScale;
    
    // Determine PDF height based on content
    const finalPdfHeight = Math.max(scaledHeight + padding * 2, a4Height);

    const pdf = new jsPDF({
      orientation: "portrait",
      unit: "mm",
      format: [a4Width, finalPdfHeight],
    });

    // Set background
    pdf.setFillColor(backgroundRgb.r, backgroundRgb.g, backgroundRgb.b);
    pdf.rect(0, 0, a4Width, finalPdfHeight, "F");

    pdf.addImage(
      imgData,
      "JPEG",
      padding,
      padding,
      scaledWidth,
      scaledHeight
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
    padding = 10,
    theme,
    selector,
  } = options;

   // Resolve theme
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

    const elementHeight = canvasContent.scrollHeight || canvasContent.offsetHeight;
    const extraHeight = 100;

    const canvas = await html2canvas(canvasContent, {
      scale,
      backgroundColor,
      useCORS: true,
      allowTaint: true,
      logging: false,
      removeContainer: true,
      imageTimeout: 0,
      height: elementHeight + extraHeight,
      windowWidth: canvasContent.scrollWidth,
      foreignObjectRendering: false,
       onclone: (clonedDoc) => {
        const element = clonedDoc.body.querySelector(selector || "[data-canvas-content]") as HTMLElement;
        if (element) {
           element.style.height = "auto";
           element.style.overflow = "visible";
           element.style.maxHeight = "none";
        }
        
        const hideElements = clonedDoc.querySelectorAll('[data-hide-in-pdf="true"]');
        hideElements.forEach((el) => {
          (el as HTMLElement).style.display = "none";
        });
      },
    });

    const { jsPDF } = await import("jspdf");
    
    const imgData = canvas.toDataURL("image/jpeg", quality);
    const imgWidth = canvas.width;
    const imgHeight = canvas.height;

    const a4Width = 210;
    const a4Height = 297;

    const pdfScale = (a4Width - padding * 2) / imgWidth;
    const scaledWidth = imgWidth * pdfScale;
    const scaledHeight = imgHeight * pdfScale;
    
    const finalPdfHeight = Math.max(scaledHeight + padding * 2, a4Height);

    const pdf = new jsPDF({
      orientation: "portrait",
      unit: "mm",
      format: [a4Width, finalPdfHeight],
    });

    pdf.setFillColor(backgroundRgb.r, backgroundRgb.g, backgroundRgb.b);
    pdf.rect(0, 0, a4Width, finalPdfHeight, "F");

    pdf.addImage(imgData, "JPEG", padding, padding, scaledWidth, scaledHeight);

    return pdf.output("blob");

  } catch (error) {
    console.error("Error generating PDF blob:", error);
    throw error;
  }
}

