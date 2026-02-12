import { z } from "zod";
import { createTRPCRouter, protectedProcedure } from "../init";
import { jobQueue, connection } from "@midday/jobs/queue";
import { TRPCError } from "@trpc/server";

export const jobsRouter = createTRPCRouter({
  getStatus: protectedProcedure
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

      return {
        id: job.id,
        status: state, // 'active' | 'waiting' | 'completed' | 'failed' | 'delayed'
        progress: typeof progress === 'number' ? progress : undefined,
        result,
        error,
      };
    }),

  /**
   * List recent jobs for the user
   */
  list: protectedProcedure.query(async ({ ctx }) => {
    // This is more complex since BullMQ doesn't easily filter by user in Redis
    // In a real system, we'd store job metadata in the DB
    return [];
  }),
});
