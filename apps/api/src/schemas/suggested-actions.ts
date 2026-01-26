import { z } from "zod";

export const getSuggestedActionsSchema = z.object({
  limit: z.number().min(1).max(100).default(10),
});

export const trackSuggestedActionUsageSchema = z.object({
  actionId: z.string(),
});
