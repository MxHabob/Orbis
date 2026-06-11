import { Queue, type QueueOptions } from "bullmq";
import IORedis from "ioredis";

const redisUrl = process.env.REDIS_URL || "";

// Only create Redis connection if URL is provided
const hasRedis = Boolean(redisUrl);

let connection: IORedis | null = null;

if (hasRedis) {
  connection = new IORedis(redisUrl, {
    maxRetriesPerRequest: null,
    lazyConnect: true,
    retryStrategy: (times: number) => {
      if (times > 50) return null;
      return Math.min(times * 200, 3000);
    },
  });
}

export { connection };

export const QUEUE_NAME = "orbis-jobs";

// Only create queue if Redis is available
export const jobQueue = hasRedis
  ? new Queue(QUEUE_NAME, {
      connection: connection as any,
      defaultJobOptions: {
        attempts: 3,
        backoff: {
          type: "exponential",
          delay: 2000,
        },
        removeOnComplete: { age: 600, count: 200 },
        removeOnFail: { age: 86400, count: 500 },
      },
    })
  : (null as unknown as Queue);

// ============================================================
// Job Type Definitions
// ============================================================

export type JobType =
  | "certificate-export"
  | "student-import"
  | "report-generation";

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
  type:
    | "monthly"
    | "half-yearly"
    | "final"
    | "attendance_sheet"
    | "students_parents_data"
    | "grades_grid"
    | "control_staff"
    | "teacher_grading"
    | "control_comparison";
  month?: number;
  monthText?: string;
  format: "xlsx" | "csv";
  subjectId?: number;
  termId?: number;
  academicYearId?: number;
}
