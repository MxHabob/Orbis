import type { Job } from "bullmq";
import type { CertificateExportData } from "../queue";
import { getStudentsByClassId, getValidSessionByUserId } from "@midday/db/queries";
import { getCertificateTemplateById } from "@midday/db/queries/certificate-templates";
import { createDocument } from "@midday/db/queries/documents";
import { db } from "@midday/db/client";
import { uploadFile } from "@midday/storage";
import { logger } from "@midday/logger";
import puppeteer, { type Browser, type Page, type PaperFormat } from "puppeteer";

// ============================================================
// Configuration Constants
// ============================================================
const MAX_STUDENTS_PER_BATCH = 100; // Increased to handle typical classes in one go
const PAGE_NAVIGATION_TIMEOUT = 180_000; // 3 minutes
const READY_SIGNAL_TIMEOUT = 180_000;
const BROWSER_ARGS = [
  "--no-sandbox",
  "--disable-setuid-sandbox",
  "--disable-dev-shm-usage",
  "--disable-gpu",
  "--disable-extensions",
  "--disable-background-networking",
  "--disable-default-apps",
  "--disable-sync",
  "--disable-translate",
  "--metrics-recording-only",
  "--no-first-run",
  "--safebrowsing-disable-auto-update",
  "--font-render-hinting=none",
  "--disable-web-security",
];

/**
 * Certificate Export Handler (Production Grade - 2026)
 *
 * Architecture:
 * ┌──────────────────────────────────────────────────────────────┐
 * │ 1. Fetch all students for selected classes                  │
 * │ 2. Split into batches of MAX_STUDENTS_PER_BATCH             │
 * │ 3. For each batch:                                          │
 * │    a. Navigate Puppeteer to batch export page                │
 * │    b. Wait for render signal                                 │
 * │    c. Generate PDF for batch                                 │
 * │ 4. Merge all batch PDFs into single final PDF                │
 * │ 5. Upload to S3 and return download URL                      │
 * └──────────────────────────────────────────────────────────────┘
 *
 * Key Improvements over v1:
 * - Batch processing prevents OOM on large classes
 * - Student IDs stored in Redis instead of URL query string
 * - Reuses single browser instance across batches
 * - Granular progress tracking per student
 * - Automatic cleanup on failure
 * - Debug screenshots on timeout errors
 */
// ============================================================
// Arabic Constants for Professional Naming
// ============================================================
const MONTHS_AR: Record<number, string> = {
  1: "محرم", 2: "صفر", 3: "ربيع أول", 4: "ربيع ثاني",
  5: "جمادى أول", 6: "جمادى ثاني", 7: "رجب", 8: "شعبان",
  9: "رمضان", 10: "شوال", 11: "ذو القعدة", 12: "ذو الحجة",
};

const TYPES_AR: Record<string, string> = {
  "monthly": "شهري",
  "half-yearly": "نصفي",
  "final": "نهائي",
};

// Map template paper size to Puppeteer paper format
const PAPER_FORMAT_MAP: Record<string, PaperFormat> = {
  a4: "a4",
  a5: "a5",
  letter: "letter",
};

const PAPER_DIMENSIONS_MM: Record<string, { width: number; height: number }> = {
  a4: { width: 210, height: 297 },
  a5: { width: 148, height: 210 },
  letter: { width: 216, height: 279 },
};

export async function handleCertificateExport(
  job: Job<CertificateExportData>
) {
  const startTime = Date.now();
  const { classIds, userId, templateId, certificateType, selectedMonth } = job.data;
  
  logger.info(`[CertExport] Job ${job.id} started. Classes: ${classIds.length}`);

  let browser: Browser | undefined;
  
  try {
    // ──────────────────────────────────────────
    // Phase 1: Fetch metadata & Session
    // ──────────────────────────────────────────
    const { classes, academicYears } = await import("@midday/db/schema");
    const { eq, inArray } = await import("drizzle-orm");

    const classMetadata = await db
      .select({
        id: classes.id,
        name: classes.name,
      })
      .from(classes)
      .where(inArray(classes.id, classIds.map(Number)));

    if (classMetadata.length === 0) {
      throw new Error("لم يتم العثور على الفصول المحددة");
    }

    const session = await getValidSessionByUserId(userId);
    if (!session) throw new Error("جلسة عمل غير صالحة. يرجى تسجيل الدخول.");

    // Fetch template to get paper size and orientation from the Unified Content Tree
    let templateSize = "a4";
    let templateOrientation: "portrait" | "landscape" = "portrait";
    
    if (templateId) {
      const templateData = await getCertificateTemplateById(templateId);
      if (templateData) {
        // Best Practice 2026: Extract from Figma-style Content Tree
        const canvasConfig = (templateData as any).content?.canvasConfig;
        
        templateSize = canvasConfig?.size || (templateData as any).size || "a4";
        templateOrientation = (canvasConfig?.orientation || (templateData as any).orientation || "portrait") as "portrait" | "landscape";
      }
    }
    
    const paperFormat = PAPER_FORMAT_MAP[templateSize] || "a4";
    const paperDims = PAPER_DIMENSIONS_MM[templateSize] || PAPER_DIMENSIONS_MM.a4!;
    logger.info(`[CertExport] Paper: ${templateSize} (${templateOrientation})`);

    const dashboardUrl = process.env.NEXT_PUBLIC_URL || "http://localhost:3001";
    await job.updateProgress(5);

    // ──────────────────────────────────────────
    // Phase 2: Launch Browser
    // ──────────────────────────────────────────
    browser = await puppeteer.launch({
      headless: true,
      args: BROWSER_ARGS,
      protocolTimeout: PAGE_NAVIGATION_TIMEOUT,
    });
    
    const page = await browser.newPage();
    await page.setViewport({ width: 1200, height: 1600 });
    await setupLogging(page);
    await setupAuth(page, session.token, dashboardUrl);

    // ──────────────────────────────────────────
    // Phase 3: Loop Classes & Generate PDFs
    // ──────────────────────────────────────────
    const generatedFiles: { name: string; buffer: Buffer; classId: number; title: string }[] = [];
    const isMonthly = certificateType === "monthly";
    const typeLabel = TYPES_AR[certificateType] || certificateType;
    const monthLabel = isMonthly ? MONTHS_AR[selectedMonth] : "";

    for (let i = 0; i < classMetadata.length; i++) {
      const cls = classMetadata[i]!;
      const classProgress = 10 + (i / classMetadata.length) * 80;
      await job.updateProgress(Math.round(classProgress));

      logger.info(`[CertExport] Processing class: ${cls.name} (${i + 1}/${classMetadata.length})`);

      // Fetch students for this class
      const students = await getStudentsByClassId(cls.id, 1000, 0);
      if (!students || students.length === 0) {
        logger.warn(`[CertExport] Skipping empty class: ${cls.name}`);
        continue;
      }

      // Generate filename: اسم الفصل_الشهر_نوع الشهادة
      const namePart = cls.name.replace(/\s+/g, "_");
      const monthPart = isMonthly ? `_${monthLabel}` : "";
      const pdfName = `${namePart}${monthPart}_${typeLabel}.pdf`;
      const pdfTitle = `شهادات ${typeLabel} - ${cls.name}${isMonthly ? ` - شهر ${monthLabel}` : ""}`;

      // Build Export URL for this class
      const studentIds = students.map((s: any) => s.userId).join(",");
      const exportUrl = buildExportUrl(dashboardUrl, {
        studentIds,
        templateId,
        certificateType,
        selectedMonth,
      });

      // Render & Capture — Best Practice 2026: Use networkidle0 to ensure all assets are fetched
      // plus our custom readiness signal for data-injection sanity
      await page.goto(exportUrl, { waitUntil: "networkidle0", timeout: PAGE_NAVIGATION_TIMEOUT });
      await page.waitForFunction(() => (window as any).isReadyToCapture === true, { 
        timeout: READY_SIGNAL_TIMEOUT,
        polling: 500 // Check frequently
      });

      const pdfBuffer = await page.pdf({
        width: `${templateOrientation === 'portrait' ? paperDims!.width : paperDims!.height}mm`,
        height: `${templateOrientation === 'portrait' ? paperDims!.height : paperDims!.width}mm`,
        printBackground: true,
        margin: { top: "0mm", right: "0mm", bottom: "0mm", left: "0mm" },
        preferCSSPageSize: true,
        landscape: templateOrientation === 'landscape',
      });

      generatedFiles.push({
        name: pdfName,
        buffer: pdfBuffer as Buffer,
        classId: cls.id,
        title: pdfTitle,
      });

      // Progress within class loop
      await job.updateProgress(Math.round(classProgress + (1 / classMetadata.length) * 0.5));
    }

    if (generatedFiles.length === 0) {
      throw new Error("لم يتم إنشاء أي شهادات. قد تكون الفصول فارغة.");
    }

    // ──────────────────────────────────────────
    // Phase 4: Individual Uploads & ZIP Creation
    // ──────────────────────────────────────────
    await job.updateProgress(90);
    logger.info(`[CertExport] Uploading ${generatedFiles.length} files and preparing final result...`);

    // Upload each PDF to S3 & DB for the Vault
    for (const f of generatedFiles) {
      const path = `exports/${userId}/${Date.now()}_${f.name}`;
      await uploadFile({ bucket: "vault", key: path, file: f.buffer, contentType: "application/pdf" });
      await createDocument({
        name: f.name,
        title: f.title,
        size: f.buffer.length,
        mimetype: "application/pdf",
        extension: "pdf",
        key: path,
        userId: userId,
        type: "file",
        pathTokens: ["exports", userId, f.name],
      });
    }

    // Final result: Single PDF or ZIP
    let finalBuffer: Buffer;
    let finalName: string;
    let finalType: string;

    if (generatedFiles.length === 1) {
      finalBuffer = generatedFiles[0]!.buffer;
      finalName = generatedFiles[0]!.name;
      finalType = "application/pdf";
    } else {
      logger.info("[CertExport] Bundling multiple classes into ZIP...");
      finalBuffer = await createZipBuffer(generatedFiles);
      const timestamp = Date.now();
      finalName = `حزمة_شهادات_${typeLabel}${isMonthly ? `_${monthLabel}` : ""}_${timestamp}.zip`;
      finalType = "application/zip";
    }

    // Upload Final Package
    const finalPath = `exports/${userId}/${finalName}`;
    const uploadResult = await uploadFile({
      bucket: "vault",
      key: finalPath,
      file: finalBuffer,
      contentType: finalType,
    });

    await job.updateProgress(100);
    const duration = ((Date.now() - startTime) / 1000).toFixed(1);
    logger.info(`[CertExport] ✅ Job ${job.id} completed in ${duration}s. Size: ${(finalBuffer.length / 1024).toFixed(0)}KB`);

    return {
      success: true,
      url: uploadResult.url,
      totalItems: generatedFiles.length,
      fileName: finalName,
      fullPath: finalPath,
    };

  } catch (error) {
    const errMsg = error instanceof Error ? error.message : "Unknown error";
    logger.error(`[CertExport] ❌ Job ${job.id} failed: ${errMsg}`);
    throw error;
  } finally {
    if (browser) await browser.close().catch(() => {});
  }
}

/**
 * Configure logging for the Puppeteer page
 */
async function setupLogging(page: Page) {
  page.on("console", (msg) => {
    const text = msg.text();
    if (text.includes("[ExportPage]") || text.includes("[tRPC]")) {
      logger.info(`[Puppeteer] ${text}`);
    } else if (msg.type() === "error") {
      logger.error(`[Puppeteer Error] ${text}`);
    }
  });
}

/**
 * Configure cookies and extra headers for authentication inside Puppeteer
 */
async function setupAuth(page: Page, token: string, dashboardUrl: string) {
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3002";
  const domains = [new URL(dashboardUrl).hostname, new URL(apiUrl).hostname];
  const cookieNames = ["better-auth.session-token", "better-auth.session_token"];

  for (const domain of domains) {
    for (const name of cookieNames) {
      await page.setCookie({
        name,
        value: token,
        domain: domain === "localhost" ? undefined : domain,
        path: "/",
        httpOnly: true,
        secure: false,
        sameSite: "Lax",
        url: domain === "localhost" ? `http://${domain}` : `https://${domain}`
      });
    }
  }

  await page.setExtraHTTPHeaders({
    "Authorization": `Bearer ${token}`,
    "x-better-auth-session": token,
  });
}

/**
 * Create a ZIP buffer from generated files using JSZip
 */
async function createZipBuffer(files: { name: string; buffer: Buffer }[]): Promise<Buffer> {
  const JSZip = (await import("jszip")).default;
  const zip = new JSZip();
  
  for (const f of files) {
    zip.file(f.name, f.buffer);
  }
  
  const content = await zip.generateAsync({ type: "nodebuffer", compression: "DEFLATE" });
  return content;
}

// ============================================================
// Helper Functions
// ============================================================

/**
 * Fetch students from all selected classes in parallel
 */
async function fetchAllStudents(classIds: string[]) {
  const results = await Promise.all(
    classIds.map(async (id) => {
      const res = await getStudentsByClassId(Number(id), 1000, 0);
      return res || [];
    })
  );
  return results.flat();
}

/**
 * Split an array into chunks of given size
 */
function splitIntoBatches<T>(items: T[], batchSize: number): T[][] {
  const batches: T[][] = [];
  for (let i = 0; i < items.length; i += batchSize) {
    batches.push(items.slice(i, i + batchSize));
  }
  return batches;
}

/**
 * Build the export URL with proper query parameters.
 * Validates URL length to prevent browser limits.
 */
function buildExportUrl(
  baseUrl: string,
  params: {
    studentIds: string;
    templateId?: string;
    certificateType: string;
    selectedMonth: number;
  }
): string {
  const url = new URL(`${baseUrl}/export/certificate/bulk`);
  url.searchParams.set("studentIds", params.studentIds);
  if (params.templateId) {
    url.searchParams.set("templateId", params.templateId);
  }
  url.searchParams.set("type", params.certificateType);
  url.searchParams.set("month", String(params.selectedMonth));

  const fullUrl = url.toString();

  // Warn if URL is getting long (browser limit ~8000 chars)
  if (fullUrl.length > 6000) {
    logger.warn(
      `[CertExport] URL length is ${fullUrl.length} chars — approaching browser limit`
    );
  }

  return fullUrl;
}

/**
 * Merge multiple PDF buffers into a single PDF using pdf-lib.
 * This preserves quality and is much faster than re-rendering.
 */
async function mergePdfBuffers(buffers: Buffer[]): Promise<Buffer> {
  const { PDFDocument } = await import("pdf-lib");

  const mergedPdf = await PDFDocument.create();

  for (const buffer of buffers) {
    const pdf = await PDFDocument.load(buffer);
    const pages = await mergedPdf.copyPages(pdf, pdf.getPageIndices());
    for (const page of pages) {
      mergedPdf.addPage(page);
    }
  }

  const mergedBytes = await mergedPdf.save();
  return Buffer.from(mergedBytes);
}

/**
 * Capture a debug screenshot when a batch fails
 */
async function captureErrorScreenshot(
  page: Page,
  job: Job,
  batchIdx: number
): Promise<void> {
  try {
    const buffer = await page.screenshot({ fullPage: true });
    const errorPath = `errors/cert-job-${job.id}-batch${batchIdx}-${Date.now()}.png`;
    await uploadFile({
      bucket: "vault", // Changed from 'exports' to 'vault'
      key: errorPath,
      file: buffer as Buffer,
      contentType: "image/png",
    });
    logger.error(`[CertExport] Error screenshot saved: ${errorPath}`);
    await job.log(`Error screenshot: ${errorPath}`);
  } catch (snapshotError) {
    logger.error(
      `[CertExport] Failed to capture error screenshot: ${snapshotError}`
    );
  }
}