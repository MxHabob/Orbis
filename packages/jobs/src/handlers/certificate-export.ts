import type { Job } from "bullmq";
import type { CertificateExportData } from "../queue";
import { getStudentsByClassId } from "@midday/db/queries";
import { uploadFile } from "@midday/storage";
import { logger } from "@midday/logger";
import { auth } from "@midday/auth";
import puppeteer from "puppeteer";

/**
 * Certificate Export Handler (Production Grade)
 * 
 * Orchestrates: Data fetching -> Puppeteer Rendering -> Multi-page PDF -> S3 Upload
 */
export async function handleCertificateExport(job: Job<CertificateExportData>) {
  const { classIds, userId, templateId, certificateType, selectedMonth } = job.data;
  const dashboardUrl = process.env.NEXT_PUBLIC_URL || "http://localhost:3001";

  let browser;
  try {
    // 1. Fetch Student IDs
    await job.updateProgress(5);
    const students = await fetchAllStudents(classIds);
    if (students.length === 0) throw new Error("لم يتم العثور على طلاب للتصدير");

    const studentIds = students.map(s => s.userId).join(",");
    
    // 2. Setup Authentication for Puppeteer
    // We create a temporary session for Puppeteer to bypass login
    const session = await auth.api.createSession({
      userId: userId,
    });

    if (!session) throw new Error("فشل في إنشاء جلسة عمل للمحرك");

    await job.updateProgress(10);

    // 3. Launch Puppeteer
    browser = await puppeteer.launch({
      headless: true,
      args: ["--no-sandbox", "--disable-setuid-sandbox", "--disable-dev-shm-usage"],
    });

    const page = await browser.newPage();
    
    // Set the session cookie
    await page.setCookie({
      name: "better-auth.session-token",
      value: session.token,
      domain: new URL(dashboardUrl).hostname,
      path: "/",
      httpOnly: true,
      secure: process.env.NODE_ENV === "production",
    });

    // 4. Navigate to the Bulk Export Page
    const exportUrl = `${dashboardUrl}/export/certificate/bulk?studentIds=${studentIds}&templateId=${templateId}&type=${certificateType}&month=${selectedMonth}`;
    
    logger.info(`Navigating to export URL: ${exportUrl}`);
    await page.goto(exportUrl, { waitUntil: "networkidle0", timeout: 60000 });

    // 5. Wait for the "Ready" signal from the page
    // The page sets window.isReadyToCapture = true when GradesTable is loaded
    await page.waitForFunction(() => (window as any).isReadyToCapture === true, {
      timeout: 30000,
    });

    await job.updateProgress(50);

    // 6. Generate PDF
    const pdfBuffer = await page.pdf({
      format: "A4",
      printBackground: true,
      margin: { top: "0px", right: "0px", bottom: "0px", left: "0px" },
      preferCSSPageSize: true,
    });

    await job.updateProgress(85);

    // 7. Upload to Storage
    const timestamp = Date.now();
    const fileName = `شهادات_${selectedMonth}_${timestamp}.pdf`;
    const path = `exports/${userId}/${fileName}`;

    const result = await uploadFile({
      bucket: "exports",
      key: path,
      file: pdfBuffer as Buffer,
      contentType: "application/pdf",
    });

    await job.updateProgress(100);
    
    return {
      success: true,
      url: result.url,
      totalItems: students.length,
      fileName,
      filePath: path,
      fullPath: path,
    };
  } catch (error) {
    logger.error(`Error in certificate-export handler: ${error instanceof Error ? error.message : "Unknown error"}`);
    throw error;
  } finally {
    if (browser) await browser.close();
  }
}

async function fetchAllStudents(classIds: string[]) {
  const results = await Promise.all(
    classIds.map(async (id) => {
      // Use a generous limit for bulk export
      const res = await getStudentsByClassId(Number(id), 1000, 0);
      return res || [];
    })
  );
  return results.flat();
}
