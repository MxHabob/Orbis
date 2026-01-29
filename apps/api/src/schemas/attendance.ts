import { z } from "@hono/zod-openapi";

export const attendanceStatusEnum = z.enum([
  'present',
  'absent',
  'late',
  'excused',
  'sick',
  'authorized_leave',
  'field_trip'
]);

export const attendanceRecordSchema = z.object({
  id: z.number().openapi({ example: 1 }),
  studentId: z.string().uuid().openapi({ example: "550e8400-e29b-41d4-a716-446655440000" }),
  scheduledLessonId: z.number().openapi({ example: 1 }),
  date: z.string().openapi({ example: "2025-01-01" }),
  status: attendanceStatusEnum.openapi({ example: "present" }),
  arrivalTime: z.string().nullable().openapi({ example: "08:05:00" }),
  notes: z.string().nullable().openapi({ example: "Student was late due to traffic" }),
  markedById: z.string().uuid().nullable().openapi({ example: "550e8400-e29b-41d4-a716-446655440001" }),
  isVerified: z.boolean().default(false).openapi({ example: false }),
});

export const markAttendanceSchema = attendanceRecordSchema.omit({ id: true });

export const bulkMarkAttendanceSchema = z.array(markAttendanceSchema);
