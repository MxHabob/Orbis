import { db } from "@db/client";
import { auditLogs } from "@db/schema";
import { desc, eq } from "drizzle-orm";

export const getAuditLogs = async (limit = 50) => {
  return db
    .select()
    .from(auditLogs)
    .orderBy(desc(auditLogs.timestamp))
    .limit(limit);
};

export type CreateAuditLogParams = typeof auditLogs.$inferInsert;

export const createAuditLog = async (data: CreateAuditLogParams) => {
  return db.insert(auditLogs).values(data).returning();
};
