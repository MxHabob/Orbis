
import { z } from "zod";
import { createTRPCRouter, protectedProcedure } from "../init";
import { users } from "@midday/db/schema";
import { eq } from "drizzle-orm";


// Default School Metrics Widgets
const DEFAULT_PRIMARY_WIDGETS = [
  "student-count",
  "teacher-count",
  "attendance-rate",
  "gender-distribution",
  "class-distribution",
  "recent-students",
  // "runway", // legacy
  // "revenue-summary", // legacy
];

const DEFAULT_AVAILABLE_WIDGETS: string[] = [];

export const widgetsRouter = createTRPCRouter({
  getWidgetPreferences: protectedProcedure.query(async ({ ctx }) => {
    const user = await ctx.db.query.users.findFirst({
      where: eq(users.id, ctx.session.user.id),
    });

    const preferences = (user?.metadata as any)?.widgets as {
      primaryWidgets: string[];
      availableWidgets: string[];
    } | undefined;

    return {
      primaryWidgets: preferences?.primaryWidgets ?? DEFAULT_PRIMARY_WIDGETS,
      availableWidgets: preferences?.availableWidgets ?? DEFAULT_AVAILABLE_WIDGETS,
    };
  }),

  updateWidgetPreferences: protectedProcedure
    .input(
      z.object({
        primaryWidgets: z.array(z.string()).optional(),
        availableWidgets: z.array(z.string()).optional(),
      })
    )
    .mutation(async ({ ctx, input }) => {
      const user = await ctx.db.query.users.findFirst({
        where: eq(users.id, ctx.session.user.id),
      });

      const currentMetadata = (user?.metadata as any) || {};
      const currentWidgets = currentMetadata.widgets || {
        primaryWidgets: DEFAULT_PRIMARY_WIDGETS,
        availableWidgets: DEFAULT_AVAILABLE_WIDGETS,
      };

      const newWidgets = {
        primaryWidgets: input.primaryWidgets ?? currentWidgets.primaryWidgets,
        availableWidgets: input.availableWidgets ?? currentWidgets.availableWidgets,
      };

      await ctx.db
        .update(users)
        .set({
            metadata: {
                ...currentMetadata,
                widgets: newWidgets
            }
        })
        .where(eq(users.id, ctx.session.user.id));

      return { success: true };
    }),
});
