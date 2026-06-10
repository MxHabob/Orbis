import { startWorker } from "@orbis/jobs/worker";
import { logger } from "@orbis/logger";

/**
 * API Worker Process
 * 
 * Runs independently of the Hono API to process background jobs.
 */
async function main() {
  logger.info("Starting background worker...");
  
  try {
    const worker = startWorker();

    if (!worker) {
      logger.info("Worker is disabled because REDIS_URL is not configured.");
      return;
    }

    // Graceful shutdown
    process.on("SIGTERM", async () => {
      logger.info("SIGTERM received. Shutting down worker...");
      await worker.close();
      process.exit(0);
    });

    process.on("SIGINT", async () => {
      logger.info("SIGINT received. Shutting down worker...");
      await worker.close();
      process.exit(0);
    });

    logger.info("Worker is running and listening for jobs");
  } catch (error) {
    logger.error("Failed to start worker:", error as Error);
    process.exit(1);
  }
}

main();
