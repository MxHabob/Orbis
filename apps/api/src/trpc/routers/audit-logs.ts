import { createTRPCRouter, adminProcedure } from "@api/trpc/init";
import { getAuditLogs } from "@midday/db/queries";
import { z } from "zod";

export const auditLogsRouter = createTRPCRouter({
  getRecent: adminProcedure
    .input(z.object({ limit: z.number().min(1).max(100).optional() }))
    .query(async ({ input }) => {
      return getAuditLogs(input?.limit);
    }),
});
