import { Queue, type QueueOptions } from "bullmq";
import IORedis from "ioredis";

const redisUrl = process.env.REDIS_URL || "redis://localhost:6379";

export const connection = new IORedis(redisUrl, {
  maxRetriesPerRequest: null,
  // Enable lazy connect to avoid blocking startup for local offline environments
  lazyConnect: true,
  // Reconnection strategy
  retryStrategy: (times: number) => {
    if (times > 50) return null; // Stop retrying after 50 attempts (~2.5 minutes)
    return Math.min(times * 200, 3000);
  },
});

export const QUEUE_NAME = "midday-jobs";

export const jobQueue = new Queue(QUEUE_NAME, {
  connection: connection as any,
  defaultJobOptions: {
    attempts: 3,
    backoff: {
      type: "exponential",
      delay: 2000,
    },
    // Keep completed jobs for 10 minutes, max 200
    removeOnComplete: { age: 600, count: 200 },
    removeOnFail: { age: 86400, count: 500 }, // Keep failed for 24h for debugging
  },
});

// ============================================================
// Job Type Definitions
// ============================================================

export type JobType = "certificate-export" | "student-import" | "report-generation";

export interface BaseJobData {
  userId: string;
  teamId?: string;
}

export interface CertificateExportData extends BaseJobData {
  classIds: string[];
  templateId?: string;
  certificateType: string;
  selectedMonth: number;
}

export interface ReportGenerationData extends BaseJobData {
  classId: number;
  type: "monthly" | "half-yearly" | "final" | "attendance_sheet";
  month?: number;
  monthText?: string;
  format: "xlsx" | "csv";
}