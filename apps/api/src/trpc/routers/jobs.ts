import { jobQueue } from "@orbis/workers/queue";
import { TRPCError } from "@trpc/server";
import { z } from "zod";
import {
  createTRPCRouter,
  protectedProcedure,
  teacherProcedure,
} from "../init";

export const jobsRouter = createTRPCRouter({
  /**
   * Get the status of a background job by ID.
   * Returns status, progress, result, and error details.
   */
  getStatus: teacherProcedure
    .input(z.object({ jobId: z.string() }))
    .query(async ({ input }) => {
      const job = await jobQueue.getJob(input.jobId);

      if (!job) {
        throw new TRPCError({
          code: "NOT_FOUND",
          message: "Job not found",
        });
      }

      const state = await job.getState();
      const progress = job.progress;
      const result = job.returnvalue;
      const error = job.failedReason;
      const logs = await job.log(undefined as any); // Get all logs

      return {
        id: job.id,
        status: state,
        progress: typeof progress === "number" ? progress : undefined,
        result,
        error,
        attemptsMade: job.attemptsMade,
        timestamp: job.timestamp,
        processedOn: job.processedOn,
        finishedOn: job.finishedOn,
      };
    }),

  /**
   * List recent jobs for the current user.
   * Note: In production, consider storing job metadata in the DB for efficient querying.
   */
  list: protectedProcedure.query(async ({ ctx }) => {
    return [];
  }),
});
