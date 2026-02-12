import { jobQueue, type CertificateExportData } from "./queue";
import { nanoid } from "nanoid";

/**
 * Background Job Service
 * 
 * Provides a clean API for triggering long-running operations.
 */
export const JobService = {
  /**
   * Starts a bulk certificate export task
   */
  async triggerCertificateExport(data: CertificateExportData) {
    const jobId = `cert-export-${nanoid(10)}`;
    
    await jobQueue.add("certificate-export", data, {
      jobId,
      priority: 1,
    });

    return { jobId };
  },

  /**
   * Starts a student import task
   */
  async triggerStudentImport(data: any) {
    const jobId = `student-import-${nanoid(10)}`;
    await jobQueue.add("student-import", data, { jobId });
    return { jobId };
  }
};
