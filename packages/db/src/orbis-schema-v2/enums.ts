import { pgEnum } from "drizzle-orm/pg-core";

export const userStatusEnum = pgEnum("user_status", [
  "active",
  "inactive",
  "suspended",
  "pending",
  "banned",
]);

export const roleEnum = pgEnum("app_role", [
  "super_admin",
  "admin",
  "principal",
  "teacher",
  "student",
  "parent",
  "staff",
  "accountant",
  "librarian",
]);

export const academicStatusEnum = pgEnum("academic_status", [
  "active",
  "graduated",
  "transferred",
  "withdrawn",
  "suspended",
  "on_leave",
]);

export const attendanceEnum = pgEnum("attendance_status", [
  "present",
  "absent",
  "late",
  "excused",
  "sick",
]);

export const paymentStatusEnum = pgEnum("payment_status", [
  "pending",
  "paid",
  "partial",
  "overdue",
  "waived",
  "cancelled",
]);

export const notificationChannelEnum = pgEnum("notification_channel", [
  "in_app",
  "email",
  "sms",
  "push",
  "whatsapp",
]);
