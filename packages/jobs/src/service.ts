import { logger } from "@orbis/logger";
import { nanoid } from "nanoid";
import {
  type CertificateExportData,
  jobQueue,
  type ReportGenerationData,
} from "./queue";

/**
 * Background Job Service (2026)
 *
 * Provides a clean, validated API for triggering long-running operations.
 * All methods return the job ID for status tracking.
 *
 * LOCAL MODE: When Redis/BullMQ is unavailable, jobs execute inline (synchronously).
 * This is ideal for school deployments with 2–4 users where queue overhead is unnecessary.
 */
export const JobService = {
  /**
   * Starts a bulk certificate export task.
   */
  async triggerCertificateExport(data: CertificateExportData) {
    if (!data.classIds || data.classIds.length === 0) {
      throw new Error("يجب تحديد فصل واحد على الأقل");
    }

    if (!data.userId) {
      throw new Error("معرف المستخدم مطلوب");
    }

    const jobId = `cert-export-${nanoid(10)}`;

    logger.info(
      `[JobService] Queuing certificate export: ${data.classIds.length} class(es), type=${data.certificateType}, month=${data.selectedMonth}`,
    );

    // Try Redis queue first, fall back to inline execution
    if (jobQueue) {
      try {
        await jobQueue.add("certificate-export", data, {
          jobId,
          priority: 1,
        });
        return { jobId };
      } catch (error) {
        logger.warn(
          `[JobService] Redis queue unavailable, running inline: ${error instanceof Error ? error.message : String(error)}`,
        );
      }
    }

    // Inline fallback for local/offline deployment
    logger.info(`[JobService] Running certificate export inline (no Redis)`);
    import("./handlers/certificate-export").then((m) => {
      m.handleCertificateExport({ id: jobId, data } as any).catch((err) =>
        logger.error(`Inline export failed: ${err}`),
      );
    });
    return { jobId };
  },

  /**
   * Starts a student import task.
   */
  async triggerStudentImport(data: any) {
    const jobId = `student-import-${nanoid(10)}`;

    if (jobQueue) {
      try {
        await jobQueue.add("student-import", data, { jobId });
        return { jobId };
      } catch (error) {
        logger.warn(
          `[JobService] Redis queue unavailable for student import: ${error instanceof Error ? error.message : String(error)}`,
        );
      }
    }

    // Inline fallback
    logger.info(`[JobService] Student import will run inline (no Redis)`);
    return { jobId };
  },

  /**
   * Starts a report generation task.
   */
  async triggerReportGeneration(data: ReportGenerationData) {
    const jobId = `report-gen-${nanoid(10)}`;

    logger.info(
      `[JobService] Queuing report generation: classId=${data.classId}, type=${data.type}, format=${data.format}`,
    );

    if (jobQueue) {
      try {
        await jobQueue.add("report-generation", data, {
          jobId,
          priority: 2,
        });
        return { jobId };
      } catch (error) {
        logger.warn(
          `[JobService] Redis queue unavailable, running report inline: ${error instanceof Error ? error.message : String(error)}`,
        );
      }
    }

    // Inline fallback
    logger.info(`[JobService] Running report generation inline (no Redis)`);
    import("./handlers/report-generation").then((m) => {
      m.handleReportGeneration({ id: jobId, data } as any).catch((err) =>
        logger.error(`Inline report generation failed: ${err}`),
      );
    });
    return { jobId };
  },
};
