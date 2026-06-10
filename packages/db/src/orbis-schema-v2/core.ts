
import {
  pgTable,
  serial,
  integer,
  smallint,
  varchar,
  text,
  boolean,
  timestamp,
  date,
  time,
  index,
  unique,
  uuid,
  jsonb,
  primaryKey,
  pgSchema,
} from "drizzle-orm/pg-core";
import { sql } from "drizzle-orm/sql";
import { coreSchema } from "./types";
import { userStatusEnum, roleEnum } from "./enums";

// ============================================================
// ORGANIZATIONS
// ============================================================

export const organizations = coreSchema.table(
  "organizations",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    name: varchar("name", { length: 100 }).notNull(),
    slug: varchar("slug", { length: 50 }).unique().notNull(),
    code: varchar("code", { length: 20 }).unique(),

    timezone: varchar("timezone", { length: 50 })
      .default("Asia/Riyadh")
      .notNull(),
    locale: varchar("locale", { length: 10 }).default("ar-SA").notNull(),
    currency: varchar("currency", { length: 3 }).default("SAR").notNull(),
    dateFormat: varchar("date_format", { length: 20 }).default("DD/MM/YYYY"),
    hijriEnabled: boolean("hijri_enabled").default(true).notNull(),

    maxUsers: smallint("max_users").default(1000).notNull(),
    maxStudents: smallint("max_students").default(5000).notNull(),
    maxStorageGb: smallint("max_storage_gb").default(100).notNull(),

    settings: jsonb("settings")
      .default("{}")
      .$type<{
        gradingSystem?: "percentage" | "gpa_4" | "gpa_5" | "letter";
        passingGrade?: number;
        maxGrade?: number;
        allowLateRegistration?: boolean;
        autoPromote?: boolean;
        attendanceThreshold?: number;
        behaviorPointsEnabled?: boolean;
      }>(),

    features: jsonb("features")
      .default("{}")
      .$type<{
        finance?: boolean;
        transport?: boolean;
        cafeteria?: boolean;
        library?: boolean;
        clinic?: boolean;
        onlinePayments?: boolean;
        parentPortal?: boolean;
        teacherPortal?: boolean;
        mobileApp?: boolean;
      }>(),

    subscriptionPlan: varchar("subscription_plan", { length: 20 })
      .default("free")
      .notNull(),
    subscriptionExpiresAt: timestamp("subscription_expires_at"),

    logo: varchar("logo", { length: 500 }),
    favicon: varchar("favicon", { length: 500 }),
    primaryColor: varchar("primary_color", { length: 7 }).default("#6366f1"),

    isActive: boolean("is_active").default(true).notNull(),
    isVerified: boolean("is_verified").default(false).notNull(),
    isDeleted: boolean("is_deleted").default(false).notNull(),

    deletedAt: timestamp("deleted_at"),
    deletedBy: uuid("deleted_by"),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    slugIdx: index("org_slug_idx")
      .on(t.slug)
      .where(sql`${t.isDeleted} = false`),
    activeIdx: index("org_active_idx").on(
      t.isActive,
      t.subscriptionPlan,
      t.isDeleted
    ),
    planExpiryIdx: index("org_plan_expiry_idx").on(
      t.subscriptionPlan,
      t.subscriptionExpiresAt
    ),
  })
);

// ============================================================
// USERS (Unified Identity + Denormalized Profile)
// ============================================================

export const users = coreSchema.table(
  "users",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    email: varchar("email", { length: 254 }),
    emailVerified: boolean("email_verified").default(false).notNull(),
    emailVerifiedAt: timestamp("email_verified_at"),

    phone: varchar("phone", { length: 15 }),
    phoneVerified: boolean("phone_verified").default(false).notNull(),
    phoneVerifiedAt: timestamp("phone_verified_at"),

    passwordHash: varchar("password_hash", { length: 255 }),
    passwordChangedAt: timestamp("password_changed_at"),
    passwordExpiresAt: timestamp("password_expires_at"),

    mfaEnabled: boolean("mfa_enabled").default(false).notNull(),
    mfaSecret: varchar("mfa_secret", { length: 255 }),
    mfaBackupCodes: jsonb("mfa_backup_codes").$type<string[]>(),

    fullName: varchar("full_name", { length: 100 }).notNull(),
    nameAr: varchar("name_ar", { length: 100 }),
    nameEn: varchar("name_en", { length: 100 }),
    avatar: varchar("avatar", { length: 500 }),

    sex: varchar("sex", { length: 10 }),
    dateOfBirth: date("date_of_birth"),
    placeOfBirth: varchar("place_of_birth", { length: 50 }),
    nationality: varchar("nationality", { length: 50 }),
    nationalId: varchar("national_id", { length: 50 }),
    passportNumber: varchar("passport_number", { length: 50 }),

    address: text("address"),
    city: varchar("city", { length: 50 }),
    country: varchar("country", { length: 50 }),
    postalCode: varchar("postal_code", { length: 20 }),

    emergencyContact: jsonb("emergency_contact").$type<{
      name: string;
      phone: string;
      relation: string;
      email?: string;
    }>(),

    bloodType: varchar("blood_type", { length: 5 }),
    allergies: jsonb("allergies").$type<string[]>(),
    medicalConditions: jsonb("medical_conditions").$type<string[]>(),
    medicalNotes: text("medical_notes"),

    socialLinks: jsonb("social_links").$type<Record<string, string>>(),
    bio: text("bio"),

    primaryRole: roleEnum("primary_role").notNull(),
    status: userStatusEnum("status").default("pending").notNull(),

    lastLoginAt: timestamp("last_login_at"),
    lastLoginIp: varchar("last_login_ip", { length: 45 }),
    loginCount: integer("login_count").default(0),
    failedLogins: smallint("failed_logins").default(0),
    lockedUntil: timestamp("locked_until"),

    prefs: jsonb("prefs")
      .default("{}")
      .$type<{
        theme?: "light" | "dark" | "system";
        lang?: "ar" | "en" | "fr";
        rtl?: boolean;
        notifications?: Record<string, boolean>;
        dashboardLayout?: string[];
        dateFormat?: string;
        timezone?: string;
      }>(),

    meta: jsonb("meta").default("{}").$type<Record<string, unknown>>(),

    isDeleted: boolean("is_deleted").default(false).notNull(),
    deletedAt: timestamp("deleted_at"),
    deletedBy: uuid("deleted_by"),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
    createdBy: uuid("created_by"),
  },
  (t) => ({
    orgRoleStatusIdx: index("user_org_role_status_idx")
      .on(t.orgId, t.primaryRole, t.status)
      .where(sql`${t.isDeleted} = false`),

    emailIdx: unique("user_email_idx")
      .on(t.orgId, t.email)
      .where(
        sql`${t.email} IS NOT NULL AND ${t.isDeleted} = false`
      ),

    phoneIdx: unique("user_phone_idx")
      .on(t.orgId, t.phone)
      .where(
        sql`${t.phone} IS NOT NULL AND ${t.isDeleted} = false`
      ),

    nameSearchIdx: index("user_name_search_idx").using(
      "gin",
      sql`to_tsvector('arabic', COALESCE(${t.fullName}, '') || ' ' || COALESCE(${t.nameAr}, '') || ' ' || COALESCE(${t.nameEn}, ''))`
    ),

    nationalIdIdx: index("user_national_id_idx")
      .on(t.orgId, t.nationalId)
      .where(sql`${t.nationalId} IS NOT NULL`),

    statusLockIdx: index("user_status_lock_idx")
      .on(t.status, t.lockedUntil)
      .where(
        sql`${t.status} = 'suspended' OR ${t.lockedUntil} IS NOT NULL`
      ),

    lastLoginIdx: index("user_last_login_idx")
      .on(t.orgId, t.lastLoginAt)
      .where(sql`${t.lastLoginAt} IS NOT NULL`),
  })
);

// ============================================================
// OAUTH ACCOUNTS
// ============================================================

export const userAccounts = coreSchema.table(
  "user_accounts",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    userId: uuid("user_id").notNull(),

    provider: varchar("provider", { length: 20 }).notNull(),
    providerAccountId: varchar("provider_account_id", { length: 255 }).notNull(),

    providerEmail: varchar("provider_email", { length: 254 }),
    providerName: varchar("provider_name", { length: 100 }),
    providerPicture: varchar("provider_picture", { length: 500 }),

    accessToken: text("access_token"),
    refreshToken: text("refresh_token"),
    idToken: text("id_token"),
    expiresAt: timestamp("expires_at"),
    scope: text("scope"),
    tokenType: varchar("token_type", { length: 20 }),

    meta: jsonb("meta").default("{}").$type<Record<string, unknown>>(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    providerUnique: unique("account_provider_unique").on(
      t.provider,
      t.providerAccountId
    ),
    userIdx: index("account_user_idx").on(t.userId),
    providerIdx: index("account_provider_idx").on(t.provider, t.userId),
  })
);

// ============================================================
// SESSIONS (with Device Fingerprint)
// ============================================================

export const userSessions = coreSchema.table(
  "user_sessions",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    userId: uuid("user_id").notNull(),

    token: varchar("token", { length: 255 }).unique().notNull(),
    refreshToken: varchar("refresh_token", { length: 255 }).unique(),

    deviceId: varchar("device_id", { length: 255 }),
    deviceType: varchar("device_type", { length: 20 }),
    deviceName: varchar("device_name", { length: 100 }),
    os: varchar("os", { length: 50 }),
    osVersion: varchar("os_version", { length: 30 }),
    browser: varchar("browser", { length: 50 }),
    browserVersion: varchar("browser_version", { length: 30 }),

    ipAddress: varchar("ip_address", { length: 45 }),
    userAgent: text("user_agent"),
    location: jsonb("location").$type<{
      country?: string;
      countryCode?: string;
      city?: string;
      region?: string;
      lat?: number;
      lng?: number;
      timezone?: string;
    }>(),

    isCurrent: boolean("is_current").default(true).notNull(),
    isTrusted: boolean("is_trusted").default(false).notNull(),
    mfaVerified: boolean("mfa_verified").default(false).notNull(),

    expiresAt: timestamp("expires_at").notNull(),
    refreshExpiresAt: timestamp("refresh_expires_at"),
    lastActiveAt: timestamp("last_active_at").defaultNow(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    userCurrentIdx: index("session_user_current_idx").on(t.userId, t.isCurrent),
    tokenIdx: index("session_token_idx").on(t.token),
    deviceIdx: index("session_device_idx")
      .on(t.deviceId)
      .where(sql`${t.deviceId} IS NOT NULL`),
    expiresIdx: index("session_expires_idx")
      .on(t.expiresAt)
      .where(sql`${t.expiresAt} < NOW() + INTERVAL '7 days'`),
    userActiveIdx: index("session_user_active_idx")
      .on(t.userId, t.lastActiveAt)
      .where(sql`${t.isCurrent} = true`),
  })
);

// ============================================================
// VERIFICATION CODES
// ============================================================

export const verifications = coreSchema.table(
  "verifications",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    userId: uuid("user_id"),

    identifier: varchar("identifier", { length: 255 }).notNull(),
    value: varchar("value", { length: 255 }).notNull(),
    type: varchar("type", { length: 30 }).notNull(),

    attempts: smallint("attempts").default(0),
    maxAttempts: smallint("max_attempts").default(5),

    expiresAt: timestamp("expires_at").notNull(),
    usedAt: timestamp("used_at"),

    ipAddress: varchar("ip_address", { length: 45 }),
    userAgent: text("user_agent"),

    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    identifierTypeIdx: index("verify_identifier_type_idx").on(
      t.identifier,
      t.type
    ),
    userIdx: index("verify_user_idx").on(t.userId),
    expiresIdx: index("verify_expires_idx").on(t.expiresAt),
  })
);

// ============================================================
// RBAC - USER ROLES
// ============================================================

export const userRoles = coreSchema.table(
  "user_roles",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    userId: uuid("user_id").notNull(),
    role: roleEnum("role").notNull(),
    orgId: uuid("org_id").notNull(),

    scopeType: varchar("scope_type", { length: 20 })
      .default("org")
      .notNull(),
    scopeId: uuid("scope_id"),

    validFrom: date("valid_from"),
    validUntil: date("valid_until"),

    isPrimary: boolean("is_primary").default(false).notNull(),

    assignedBy: uuid("assigned_by"),
    assignedAt: timestamp("assigned_at").defaultNow().notNull(),
  },
  (t) => ({
    userRoleUnique: unique("ur_user_role_scope_unique").on(
      t.userId,
      t.role,
      t.scopeType,
      t.scopeId
    ),
    userIdx: index("ur_user_idx").on(t.userId),
    scopeIdx: index("ur_scope_idx").on(t.scopeType, t.scopeId),
    orgRoleIdx: index("ur_org_role_idx").on(t.orgId, t.role),
    validIdx: index("ur_valid_idx")
      .on(t.validFrom, t.validUntil)
      .where(
        sql`${t.validUntil} IS NOT NULL AND ${t.validUntil} > NOW()`
      ),
  })
);

// ============================================================
// PERMISSIONS (Code-Based, Cacheable)
// ============================================================

export const permissions = coreSchema.table(
  "permissions",
  {
    id: serial("id").primaryKey(),
    code: varchar("code", { length: 50 }).unique().notNull(),
    name: varchar("name", { length: 100 }).notNull(),
    nameAr: varchar("name_ar", { length: 100 }),
    description: text("description"),

    module: varchar("module", { length: 30 }).notNull(),
    action: varchar("action", { length: 20 }).notNull(),
    resource: varchar("resource", { length: 30 }).notNull(),

    requiresScope: boolean("requires_scope").default(false).notNull(),
    scopeTypes: jsonb("scope_types").$type<string[]>(),

    isSystem: boolean("is_system").default(true).notNull(),
    isActive: boolean("is_active").default(true).notNull(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    codeIdx: index("perm_code_idx").on(t.code),
    moduleIdx: index("perm_module_idx").on(t.module, t.isActive),
    resourceIdx: index("perm_resource_idx").on(t.resource, t.action),
  })
);

export const rolePermissions = coreSchema.table(
  "role_permissions",
  {
    role: roleEnum("role").notNull(),
    permissionId: integer("permission_id").notNull(),

    conditions: jsonb("conditions")
      .default("{}")
      .$type<{
        ownOnly?: boolean;
        scopeTypes?: string[];
        classLevels?: string[];
      }>(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    pk: primaryKey({ columns: [t.role, t.permissionId] }),
    permissionIdx: index("rp_permission_idx").on(t.permissionId),
  })
);

export const userPermissions = coreSchema.table(
  "user_permissions",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    userId: uuid("user_id").notNull(),
    permissionId: integer("permission_id").notNull(),

    grantType: varchar("grant_type", { length: 10 })
      .default("allow")
      .notNull(),

    conditions: jsonb("conditions").default("{}").$type<{
      scopeType?: string;
      scopeId?: string;
    }>(),

    validFrom: date("valid_from"),
    validUntil: date("valid_until"),

    reason: text("reason"),
    grantedBy: uuid("granted_by"),
    grantedAt: timestamp("granted_at").defaultNow().notNull(),

    isActive: boolean("is_active").default(true).notNull(),
  },
  (t) => ({
    userPermUnique: unique("up_user_perm_unique").on(t.userId, t.permissionId),
    userIdx: index("up_user_idx").on(t.userId),
    activeIdx: index("up_active_idx")
      .on(t.userId, t.isActive)
      .where(sql`${t.isActive} = true`),
  })
);

// ============================================================
// NOTIFICATIONS (Cross-Schema)
// ============================================================

export const notifications = coreSchema.table(
  "notifications",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    senderId: uuid("sender_id"),
    senderType: varchar("sender_type", { length: 20 }).default("system").notNull(),

    recipientId: uuid("recipient_id").notNull(),
    recipientType: varchar("recipient_type", { length: 20 }).default("user").notNull(),

    type: varchar("type", { length: 30 }).notNull(),
    priority: varchar("priority", { length: 10 }).default("normal").notNull(),

    title: varchar("title", { length: 200 }).notNull(),
    titleAr: varchar("title_ar", { length: 200 }),
    body: text("body").notNull(),
    bodyAr: text("body_ar"),

    actionUrl: varchar("action_url", { length: 500 }),
    actionText: varchar("action_text", { length: 50 }),
    actionTextAr: varchar("action_text_ar", { length: 50 }),

    entityType: varchar("entity_type", { length: 30 }),
    entityId: uuid("entity_id"),

    cycleId: uuid("cycle_id"),
    classId: uuid("class_id"),
    subjectId: uuid("subject_id"),
    enrollmentId: uuid("enrollment_id"),

    channels: jsonb("channels").$type<string[]>().default('["in_app"]'),

    isRead: boolean("is_read").default(false).notNull(),
    readAt: timestamp("read_at"),

    isArchived: boolean("is_archived").default(false).notNull(),
    archivedAt: timestamp("archived_at"),

    deliveredChannels: jsonb("delivered_channels").$type<string[]>().default("[]"),
    failedChannels: jsonb("failed_channels").$type<string[]>().default("[]"),
    deliveredAt: timestamp("delivered_at"),

    expiresAt: timestamp("expires_at"),

    meta: jsonb("meta").default("{}").$type<Record<string, unknown>>(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    recipientIdx: index("notif_recipient_idx").on(t.recipientId, t.createdAt),
    recipientUnreadIdx: index("notif_recipient_unread_idx")
      .on(t.recipientId, t.isRead)
      .where(sql`${t.isRead} = false`),
    orgTypeIdx: index("notif_org_type_idx").on(t.orgId, t.type),
    priorityIdx: index("notif_priority_idx")
      .on(t.orgId, t.priority)
      .where(sql`${t.priority} IN ('urgent', 'critical')`),
    entityIdx: index("notif_entity_idx").on(t.entityType, t.entityId),
    enrollmentIdx: index("notif_enrollment_idx").on(t.enrollmentId),
    deliveryIdx: index("notif_delivery_idx")
      .on(t.orgId, t.deliveredAt)
      .where(sql`${t.deliveredAt} IS NULL AND ${t.createdAt} < NOW() - INTERVAL '5 minutes'`),
    expiryIdx: index("notif_expiry_idx").on(t.expiresAt),
  })
);

export const notificationPreferences = coreSchema.table(
  "notification_preferences",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    userId: uuid("user_id").notNull(),

    notificationType: varchar("notification_type", { length: 30 }).notNull(),

    inAppEnabled: boolean("in_app_enabled").default(true).notNull(),
    emailEnabled: boolean("email_enabled").default(true).notNull(),
    smsEnabled: boolean("sms_enabled").default(false).notNull(),
    pushEnabled: boolean("push_enabled").default(true).notNull(),
    whatsappEnabled: boolean("whatsapp_enabled").default(false).notNull(),

    quietHoursEnabled: boolean("quiet_hours_enabled").default(false).notNull(),
    quietHoursStart: time("quiet_hours_start"),
    quietHoursEnd: time("quiet_hours_end"),

    digestMode: varchar("digest_mode", { length: 20 }).default("immediate").notNull(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    unique: unique("notif_pref_unique_idx").on(t.userId, t.notificationType),
    userIdx: index("notif_pref_user_idx").on(t.userId),
  })
);

// ============================================================
// DOCUMENTS & FILES (Cross-Schema)
// ============================================================

export const documents = coreSchema.table(
  "documents",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    ownerId: uuid("owner_id").notNull(),
    uploadedBy: uuid("uploaded_by"),

    name: varchar("name", { length: 255 }).notNull(),
    title: varchar("title", { length: 255 }),
    description: text("description"),

    type: varchar("type", { length: 20 }).default("file").notNull(),
    visibility: varchar("visibility", { length: 20 }).default("private").notNull(),

    size: integer("size"),
    mimeType: varchar("mime_type", { length: 100 }),
    extension: varchar("extension", { length: 20 }),

    storageProvider: varchar("storage_provider", { length: 20 }).default("s3").notNull(),
    bucket: varchar("bucket", { length: 100 }),
    key: text("key").notNull(),
    url: text("url"),
    thumbnailUrl: text("thumbnail_url"),

    parentId: uuid("parent_id"),
    path: text("path"),
    depth: smallint("depth").default(0),

    sharedWith: jsonb("shared_with").$type<Array<{ userId: string; permission: "read" | "write" | "admin" }>>().default("[]"),
    shareLink: text("share_link"),
    shareLinkExpiresAt: timestamp("share_link_expires_at"),
    sharePassword: varchar("share_password", { length: 100 }),

    version: smallint("version").default(1),
    previousVersionId: uuid("previous_version_id"),
    isLatestVersion: boolean("is_latest_version").default(true).notNull(),

    processingStatus: varchar("processing_status", { length: 20 }).default("completed"),
    ocrText: text("ocr_text"),
    extractedData: jsonb("extracted_data").default("{}").$type<Record<string, unknown>>(),

    isPinned: boolean("is_pinned").default(false).notNull(),
    isStarred: boolean("is_starred").default(false).notNull(),
    isTemplate: boolean("is_template").default(false).notNull(),

    tags: jsonb("tags").$type<string[]>().default("[]"),
    meta: jsonb("meta").default("{}").$type<Record<string, unknown>>(),

    deletedAt: timestamp("deleted_at"),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgOwnerIdx: index("doc_org_owner_idx").on(t.orgId, t.ownerId),
    parentIdx: index("doc_parent_idx").on(t.parentId),
    typeIdx: index("doc_type_idx").on(t.orgId, t.type),
    visibilityIdx: index("doc_visibility_idx").on(t.orgId, t.visibility),
    pathIdx: index("doc_path_idx").on(t.path),
    pinnedIdx: index("doc_pinned_idx")
      .on(t.ownerId, t.isPinned)
      .where(sql`${t.isPinned} = true`),
    latestIdx: index("doc_latest_idx")
      .on(t.ownerId, t.isLatestVersion)
      .where(sql`${t.isLatestVersion} = true`),
    shareLinkIdx: unique("doc_share_link_idx").on(t.shareLink).where(
      sql`${t.shareLink} IS NOT NULL`
    ),
  })
);
