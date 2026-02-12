import { Queue, Worker, type Job, type QueueOptions } from "bullmq";
import IORedis from "ioredis";

const redisUrl = process.env.REDIS_URL || "redis://localhost:6379";

export const connection = new IORedis(redisUrl, {
  maxRetriesPerRequest: null,
});

export const QUEUE_NAME = "midday-jobs";

export const jobQueue = new Queue(QUEUE_NAME, {
  connection,
  defaultJobOptions: {
    attempts: 3,
    backoff: {
      type: "exponential",
      delay: 1000,
    },
    removeOnComplete: true,
    removeOnFail: false,
  },
});

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
