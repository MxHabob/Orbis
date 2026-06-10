import {
  uuid,
  varchar,
  text,
  boolean,
  timestamp,
  integer,
  smallint,
  jsonb,
  index,
} from "drizzle-orm/pg-core";
import { sql } from "drizzle-orm";
import { auditSchema } from "./types";

// ============================================================
// AUDIT LOG (Partitioned by Month)
// ============================================================

export const auditLog = auditSchema.table(
  "audit_log",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id"),

    occurredAt: timestamp("occurred_at").defaultNow().notNull(),

    // Actor
    userId: uuid("user_id"),
    userRole: varchar("user_role", { length: 20 }),
    impersonatedBy: uuid("impersonated_by"),

    // Action
    action: varchar("action", { length: 20 }).notNull(),
    entityType: varchar("entity_type", { length: 30 }).notNull(),
    entityId: uuid("entity_id"),

    // Changes (compact diff)
    changes: jsonb("changes").$type<
      Record<string, { old: unknown; new: unknown }>
    >(),

    // Context
    description: text("description"),
    reason: varchar("reason", { length: 200 }),

    // Request context
    ip: varchar("ip", { length: 45 }),
    userAgent: varchar("user_agent", { length: 200 }),
    requestId: varchar("request_id", { length: 50 }),
    sessionId: varchar("session_id", { length: 100 }),

    // Location
    location: jsonb("location").$type<{
      country?: string;
      city?: string;
      lat?: number;
      lng?: number;
    }>(),

    // Device
    device: jsonb("device").$type<{
      type?: string;
      os?: string;
      browser?: string;
    }>(),

    // Performance
    durationMs: integer("duration_ms"),

    // Result
    status: varchar("status", { length: 10 }).default("success").notNull(),
    errorCode: varchar("error_code", { length: 50 }),
    errorMessage: text("error_message"),

    // Data snapshot (for critical entities)
    snapshot: jsonb("snapshot").$type<Record<string, unknown>>(),
  },
  (t) => ({
    entityIdx: index("audit_entity_idx").on(
      t.entityType,
      t.entityId,
      t.occurredAt
    ),
    userIdx: index("audit_user_idx").on(t.userId, t.occurredAt),
    orgIdx: index("audit_org_idx").on(t.orgId, t.occurredAt),
    actionIdx: index("audit_action_idx").on(
      t.orgId,
      t.action,
      t.occurredAt
    ),
    failedIdx: index("audit_failed_idx")
      .on(t.orgId, t.status, t.occurredAt)
      .where(sql`${t.status} != 'success'`),
    securityIdx: index("audit_security_idx")
      .on(t.orgId, t.action, t.status)
      .where(
        sql`${t.action} IN ('login', 'logout', 'delete', 'approve') AND ${t.status} = 'failed'`
      ),
    ttlIdx: index("audit_ttl_idx").on(t.occurredAt),
  })
);

// ============================================================
// SYSTEM LOG
// ============================================================

export const systemLog = auditSchema.table(
  "system_log",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id"),

    level: varchar("level", { length: 10 }).notNull(),
    category: varchar("category", { length: 30 }).notNull(),

    message: text("message").notNull(),
    details: jsonb("details").default("{}").$type<Record<string, unknown>>(),
    stackTrace: text("stack_trace"),

    userId: uuid("user_id"),
    requestId: varchar("request_id", { length: 50 }),
    endpoint: varchar("endpoint", { length: 100 }),
    method: varchar("method", { length: 10 }),

    durationMs: integer("duration_ms"),
    memoryUsageMb: integer("memory_usage_mb"),
    cpuUsagePercent: smallint("cpu_usage_percent"),

    timestamp: timestamp("timestamp").defaultNow().notNull(),
  },
  (t) => ({
    orgLevelIdx: index("syslog_org_level_idx").on(
      t.orgId,
      t.level,
      t.timestamp
    ),
    categoryIdx: index("syslog_category_idx").on(t.category, t.timestamp),
    errorIdx: index("syslog_error_idx")
      .on(t.level, t.timestamp)
      .where(sql`${t.level} IN ('error', 'fatal')`),
    endpointIdx: index("syslog_endpoint_idx").on(t.endpoint, t.timestamp),
    requestIdx: index("syslog_request_idx").on(t.requestId),
    slowIdx: index("syslog_slow_idx")
      .on(t.endpoint, t.durationMs)
      .where(sql`${t.durationMs} > 1000`),
  })
);

// ============================================================
// DATA TRANSFERS (Import/Export)
// ============================================================

export const dataTransfers = auditSchema.table(
  "data_transfers",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    type: varchar("type", { length: 20 }).notNull(),
    entityType: varchar("entity_type", { length: 30 }).notNull(),

    fileName: varchar("file_name", { length: 255 }),
    fileUrl: text("file_url"),
    fileSize: integer("file_size"),
    fileFormat: varchar("file_format", { length: 20 }),

    totalRecords: integer("total_records").default(0),
    successRecords: integer("success_records").default(0),
    failedRecords: integer("failed_records").default(0),
    skippedRecords: integer("skipped_records").default(0),

    errorLog: jsonb("error_log").$type<
      Array<{ row: number; field: string; error: string; value: unknown }>
    >().default("[]"),
    warningLog: jsonb("warning_log").$type<
      Array<{ row: number; field: string; warning: string; value: unknown }>
    >().default("[]"),

    status: varchar("status", { length: 20 }).default("pending").notNull(),
    startedAt: timestamp("started_at"),
    completedAt: timestamp("completed_at"),
    cancelledAt: timestamp("cancelled_at"),
    cancelReason: varchar("cancel_reason", { length: 200 }),

    initiatedBy: uuid("initiated_by"),

    config: jsonb("config").default("{}").$type<{
      delimiter?: string;
      encoding?: string;
      headers?: boolean;
      skipRows?: number;
    }>(),
    mapping: jsonb("mapping").default("{}").$type<Record<string, string>>(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgTypeIdx: index("transfer_org_type_idx").on(t.orgId, t.type),
    entityIdx: index("transfer_entity_idx").on(t.orgId, t.entityType),
    statusIdx: index("transfer_status_idx").on(t.orgId, t.status),
    userIdx: index("transfer_user_idx").on(t.initiatedBy),
  })
);
