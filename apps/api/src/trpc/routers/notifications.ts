import {
  createNotificationSchema,
  updateNotificationSchema,
} from "@api/schemas/notifications";
import { createTRPCRouter, protectedProcedure, teacherProcedure } from "@api/trpc/init";
import {
  createNotification,
  getUnreadCount,
  getUserNotifications,
  markAsRead,
  updateAllNotificationsStatus,
  updateNotificationStatus,
} from "@midday/db/queries";
import { z } from "zod";

export const notificationsRouter = createTRPCRouter({
  list: protectedProcedure
    .input(
      z.object({
        limit: z.number().min(1).max(50).optional(),
        pageSize: z.number().optional(), // Map pageSize to limit
        status: z.union([z.string(), z.array(z.string())]).optional(),
        maxPriority: z.number().optional(),
      })
    )
    .query(async ({ ctx: { session }, input }) => {
      return getUserNotifications(session.user.id, {
        limit: input.pageSize || input.limit,
        status: input.status,
        maxPriority: input.maxPriority,
      });
    }),

  getUnreadCount: protectedProcedure.query(async ({ ctx: { session } }) => {
    return getUnreadCount(session.user.id);
  }),

  create: teacherProcedure
    .input(createNotificationSchema)
    .mutation(async ({ input }) => {
      return createNotification(input);
    }),

  updateStatus: protectedProcedure
    .input(z.object({ activityId: z.number(), status: z.enum(['unread', 'read', 'archived']) }))
    .mutation(async ({ ctx: { session }, input }) => {
      return updateNotificationStatus(session.user.id, input.activityId, input.status);
    }),

  updateAllStatus: protectedProcedure
    .input(z.object({ status: z.enum(['unread', 'read', 'archived']) }))
    .mutation(async ({ ctx: { session }, input }) => {
      return updateAllNotificationsStatus(session.user.id, input.status);
    }),

  markRead: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ ctx: { session }, input }) => {
      return markAsRead(input.id, session.user.id);
    }),
});
