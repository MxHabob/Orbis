import { jobQueue, type CertificateExportData } from "./queue";
import { nanoid } from "nanoid";
import { logger } from "@midday/logger";

/**
 * Background Job Service (2026)
 *
 * Provides a clean, validated API for triggering long-running operations.
 * All methods return the job ID for status tracking.
 */
export const JobService = {
  /**
   * Starts a bulk certificate export task.
   *
   * @param data - Export configuration including class IDs, template, and type
   * @returns Job ID for status polling
   */
  async triggerCertificateExport(data: CertificateExportData) {
    // Validate input
    if (!data.classIds || data.classIds.length === 0) {
      throw new Error("يجب تحديد فصل واحد على الأقل");
    }

    if (!data.userId) {
      throw new Error("معرف المستخدم مطلوب");
    }

    const jobId = `cert-export-${nanoid(10)}`;

    logger.info(
      `[JobService] Queuing certificate export: ${data.classIds.length} class(es), type=${data.certificateType}, month=${data.selectedMonth}`
    );

    try {
      if (!process.env.REDIS_URL) {
         throw new Error("No REDIS_URL configured, bypassing queue");
      }
      await jobQueue.add("certificate-export", data, {
        jobId,
        priority: 1,
      });
    } catch (error) {
      logger.warn(`[JobService] Redis queue unavailable, running inline: ${error instanceof Error ? error.message : String(error)}`);
      
      // Fallback to inline execution for offline/local development
      import("./handlers/certificate-export").then((m) => {
        m.handleCertificateExport({ id: jobId, data } as any)
         .catch((err) => logger.error(`Inline export failed: ${err}`));
      });
      return { jobId };
    }

    return { jobId };
  },

  /**
   * Starts a student import task.
   */
  async triggerStudentImport(data: any) {
    const jobId = `student-import-${nanoid(10)}`;
    try {
      if (!process.env.REDIS_URL) {
         throw new Error("No REDIS_URL configured, bypassing queue");
      }
      await jobQueue.add("student-import", data, { jobId });
    } catch (error) {
      logger.warn(`[JobService] Failed to queue student import, queue unavailable: ${error instanceof Error ? error.message : String(error)}`);
      throw new Error("فشل في إضافة الطلب إلى قائمة الانتظار. يرجى التحقق من اتصال Redis.");
    }
    return { jobId };
  },
};
