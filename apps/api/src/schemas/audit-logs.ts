import { z } from "@hono/zod-openapi";

export const auditLogSchema = z.object({
  id: z.number().openapi({
    description: "Unique identifier of the log entry",
    example: 1,
  }),
  action: z.string().max(50).openapi({
    description: "Action performed",
    example: "UPDATE",
  }),
  tableName: z.string().max(50).openapi({
    description: "Table affected",
    example: "students",
  }),
  recordId: z.string().max(100).openapi({
    description: "ID of the record affected",
    example: "123",
  }),
  oldData: z.record(z.string(), z.any()).nullable().openapi({
    description: "Data before change",
  }),
  newData: z.record(z.string(), z.any()).nullable().openapi({
    description: "Data after change",
  }),
  userId: z.string().uuid().nullable().openapi({
    description: "User who performed the action",
  }),
  timestamp: z.string().openapi({
    description: "Timestamp of the action",
  }),
});
