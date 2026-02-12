import { Worker, type Job } from "bullmq";
import { connection, QUEUE_NAME, type CertificateExportData } from "./queue";
import { handleCertificateExport } from "./handlers/certificate-export";
import { logger } from "@midday/logger";

/**
 * Job Worker
 * 
 * Scalable process that handles different job types.
 */
export const startWorker = () => {
  const worker = new Worker(
    QUEUE_NAME,
    async (job: Job) => {
      logger.info(`Processing job ${job.id} of type ${job.name}`);

      switch (job.name) {
        case "certificate-export":
          return handleCertificateExport(job as Job<CertificateExportData>);
        
        case "student-import":
          // handleStudentImport(job);
          return { success: true };

        default:
          throw new Error(`Unknown job type: ${job.name}`);
      }
    },
    { 
      connection,
      concurrency: 5,
      removeOnComplete: { count: 100 },
      removeOnFail: { count: 500 }
    }
  );

  worker.on("completed", (job) => {
    logger.info(`Job ${job.id} completed successfully`);
  });

  worker.on("failed", (job, err) => {
    logger.error(`Job ${job?.id} failed: ${err.message}`);
  });

  return worker;
};
