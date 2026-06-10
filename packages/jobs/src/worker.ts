import { Worker, type Job } from "bullmq";
import { connection, QUEUE_NAME, type CertificateExportData, type ReportGenerationData } from "./queue";
import { handleCertificateExport } from "./handlers/certificate-export";
import { handleReportGeneration } from "./handlers/report-generation";
import { logger } from "@orbis/logger";

/**
 * Job Worker (Production Grade - 2026)
 *
 * Features:
 * - Concurrency of 2 (each Puppeteer uses ~200MB RAM, so 2 concurrent = ~400MB max)
 * - Graceful shutdown support
 * - Retry-aware logging
 * - Lock duration extended for long PDF generation
 */
export const startWorker = () => {
  if (!process.env.REDIS_URL) {
    logger.info("[Worker] REDIS_URL not provided. Worker disabled for local/offline mode.");
    return null;
  }

  const worker = new Worker(
    QUEUE_NAME,
    async (job: Job) => {
      const attemptNum = job.attemptsMade + 1;
      logger.info(
        `[Worker] Processing job ${job.id} (type: ${job.name}, attempt: ${attemptNum})`
      );

      switch (job.name) {
        case "certificate-export":
          return handleCertificateExport(
            job as Job<CertificateExportData>
          );

        case "report-generation":
          return handleReportGeneration(
            job as Job<ReportGenerationData>
          );

        case "student-import":
          // TODO: implement
          return { success: true };

        default:
          throw new Error(`Unknown job type: ${job.name}`);
      }
    },
    {
      connection: connection as any,
      // Limit concurrency: each Puppeteer job uses ~200-400MB RAM
      concurrency: 2,
      // Extend lock duration for long-running PDF jobs (5 minutes)
      lockDuration: 300_000,
      // Renew lock every 30 seconds
      lockRenewTime: 30_000,
      removeOnComplete: { count: 100 },
      removeOnFail: { count: 500 },
    }
  );

  worker.on("completed", (job) => {
    logger.info(`[Worker] ✅ Job ${job.id} completed successfully`);
  });

  worker.on("failed", (job, err) => {
    logger.error(
      `[Worker] ❌ Job ${job?.id} failed (attempt ${job?.attemptsMade}): ${err.message}`
    );
  });

  worker.on("stalled", (jobId) => {
    logger.warn(`[Worker] ⚠️ Job ${jobId} stalled — will be retried`);
  });

  worker.on("error", (err) => {
    logger.error(`[Worker] Worker error: ${err.message}`);
  });

  // Graceful shutdown
  const shutdown = async () => {
    logger.info("[Worker] Shutting down gracefully...");
    await worker.close();
    process.exit(0);
  };

  process.on("SIGTERM", shutdown);
  process.on("SIGINT", shutdown);

  logger.info(
    `[Worker] Started with concurrency=2, lockDuration=5min`
  );

  return worker;
};

// Start the worker if run directly
if (import.meta.main || process.argv[1]?.includes("worker.ts")) {
  startWorker();
}