import {
  getSuggestedActionsSchema,
  trackSuggestedActionUsageSchema,
} from "@api/schemas/suggested-actions";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { suggestedActionsCache } from "@midday/cache/suggested-actions-cache";

// School System Suggested Actions
const SUGGESTED_ACTIONS_CONFIG = [
  {
    id: "generate-report",
    toolName: "generateReport",
    toolParams: { showCanvas: true },
  },
  {
    id: "check-attendance",
    toolName: "getAttendance",
    toolParams: { pageSize: 10 },
  },
  {
    id: "view-schedule",
    toolName: "getSchedule",
    toolParams: { showCanvas: true },
  },
  {
    id: "record-grades",
    toolName: "recordGrades",
    toolParams: { showCanvas: true },
  },
  {
    id: "student-performance",
    toolName: "getStudentPerformance",
    toolParams: { showCanvas: true },
  },
  {
    id: "class-performance",
    toolName: "getClassPerformance",
    toolParams: { showCanvas: true },
  },
  {
    id: "overdue-payments",
    toolName: "getOverduePayments",
    toolParams: { pageSize: 10 },
  },
] as const;

export const suggestedActionsRouter = createTRPCRouter({
  list: protectedProcedure
    .input(getSuggestedActionsSchema)
    .query(async ({ ctx: { session }, input }) => {
      const userId = session.user.id;
      const teamId = "school"; // Placeholder for tenant ID

      const allUsage = await suggestedActionsCache.getAllUsage(teamId, userId);

      const actionsWithUsage = SUGGESTED_ACTIONS_CONFIG.map((action) => {
        const usage = allUsage[action.id];

        return {
          id: action.id,
          toolName: action.toolName,
          toolParams: action.toolParams,
          usageCount: usage?.count || 0,
          lastUsed: usage?.lastUsed ? new Date(usage.lastUsed) : null,
        };
      })
        .sort((a, b) => {
          if (a.usageCount !== b.usageCount) {
            return b.usageCount - a.usageCount;
          }
          if (a.lastUsed && b.lastUsed) {
            return b.lastUsed.getTime() - a.lastUsed.getTime();
          }
          if (a.lastUsed && !b.lastUsed) return -1;
          if (!a.lastUsed && b.lastUsed) return 1;
          return 0;
        })
        .slice(0, input.limit);

      return {
        actions: actionsWithUsage,
        total: SUGGESTED_ACTIONS_CONFIG.length,
      };
    }),

  trackUsage: protectedProcedure
    .input(trackSuggestedActionUsageSchema)
    .mutation(async ({ ctx: { session }, input }) => {
      const userId = session.user.id;
      const teamId = "school"; // Placeholder for tenant ID

      await suggestedActionsCache.incrementUsage(
        teamId,
        userId,
        input.actionId,
      );

      return { success: true };
    }),
});
