import { z } from "@hono/zod-openapi";

export const notificationTypeEnum = z.enum([
  'grade_published', 'new_assessment', 'attendance_alert', 'announcement', 
  'message', 'parent_meeting', 'fee_reminder', 'account_action', 
  'behavior_alert', 'system'
]);

export const notificationSchema = z.object({
  id: z.number().openapi({
    description: "Unique identifier of the notification",
    example: 1,
  }),
  senderId: z.string().uuid().nullable().openapi({
    description: "Sender User ID",
  }),
  recipientId: z.string().uuid().openapi({
    description: "Recipient User ID",
  }),
  classId: z.number().nullable().openapi({
    description: "Class ID",
  }),
  academicTermId: z.number().nullable().openapi({
    description: "Academic Term ID",
  }),
  type: notificationTypeEnum.openapi({
    description: "Type of notification",
    example: "system",
  }),
  title: z.string().max(255).openapi({
    description: "Notification title",
    example: "Welcome",
  }),
  message: z.string().openapi({
    description: "Notification message",
    example: "Welcome to the system",
  }),
  relatedOrbisType: z.string().max(50).nullable().openapi({
    description: "Type of related orbis",
  }),
  relatedOrbisId: z.string().max(100).nullable().openapi({
    description: "ID of related orbis",
  }),
  isRead: z.boolean().openapi({
    description: "Read status",
    example: false,
  }),
  isArchived: z.boolean().openapi({
    description: "Archived status",
    example: false,
  }),
  isHighPriority: z.boolean().openapi({
    description: "Priority status",
    example: false,
  }),
  createdAt: z.string().openapi({
    description: "Creation timestamp",
  }),
  deliveredAt: z.string().nullable().openapi({
    description: "Delivery timestamp",
  }),
  metadata: z.record(z.string(), z.any()).nullable().openapi({
    description: "Additional metadata",
  }),
});

export const createNotificationSchema = notificationSchema.omit({ 
  id: true, 
  createdAt: true, 
  deliveredAt: true 
});
export const updateNotificationSchema = notificationSchema.partial().required({ id: true });

