import {
  createNotificationSchema,
  updateNotificationSchema,
} from "@api/schemas/notifications";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  createNotification,
  getUnreadCount,
  getUserNotifications,
  markAsRead,
} from "@midday/db/queries";
import { z } from "zod";

export const notificationsRouter = createTRPCRouter({
  list: protectedProcedure
    .input(
      z.object({
        limit: z.number().min(1).max(50).optional(),
      })
    )
    .query(async ({ ctx: { session }, input }) => {
      return getUserNotifications(session.user.id, input.limit);
    }),

  getUnreadCount: protectedProcedure.query(async ({ ctx: { session } }) => {
    return getUnreadCount(session.user.id);
  }),

  create: protectedProcedure
    .input(createNotificationSchema)
    .mutation(async ({ input }) => {
      return createNotification(input);
    }),

  markRead: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ ctx: { session }, input }) => {
      return markAsRead(input.id, session.user.id);
    }),
});
