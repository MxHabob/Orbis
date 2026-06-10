
// ============================================================
// PART 1: 1 ENUMS CORE
// ============================================================

import {
  pgTable,
  serial,
  integer,
  varchar,
  text,
  boolean,
  timestamp,
  index,
  uuid,
  pgEnum,
  unique,
  numeric,
  date,
  jsonb,
  time,
  smallint,
  check,
  primaryKey,
  pgPolicy,
  AnyPgColumn,
} from 'drizzle-orm/pg-core';
import { relations, sql } from 'drizzle-orm';

// ==========================================
// SECTION 1: ENUMS (Static Values)
// ==========================================

// User & Auth Enums
export const roleEnum = pgEnum('app_role', [
  'super_admin',
  'admin', 
  'student', 
  'parent', 
  'teacher', 
  'staff',
  'accountant',
  'librarian',
  'guardian'
]);

export const userSexEnum = pgEnum('user_sex', ['unspecified', 'male', 'female']);
export const userStatusEnum = pgEnum('user_status', ['active', 'inactive', 'suspended', 'pending_verification', 'banned']);
export const identityProviderEnum = pgEnum('identity_provider', ['local', 'google', 'microsoft', 'apple', 'sso']);

// Academic Enums
export const assessmentTypeEnum = pgEnum('assessment_type', [
  'exam', 'quiz', 'assignment', 'participation', 
  'oral', 'attendance', 'behavior', 'homework', 'project', 'practical'
]);

export const classTypeEnum = pgEnum('class_type', [
  'classroom', 'laboratory', 'library', 'computer_lab', 
  'music_room', 'exam_hall', 'workshop', 'multi_purpose_room', 'sports_field'
]);

export const studentStatusEnum = pgEnum('student_status', [
  'active', 'graduated', 'transferred', 'suspended', 
  'auditor', 'homeschooled', 'withdrawn', 'expelled'
]);

export const enrollmentStatusEnum = pgEnum('enrollment_status', [
  'active', 'completed', 'withdrawn', 'transferred', 'suspended', 'on_leave'
]);

export const termStatusEnum = pgEnum('term_status', ['upcoming', 'active', 'completed', 'archived']);

// Attendance & Behavior Enums
export const attendanceStatusEnum = pgEnum('attendance_status', [
  'present', 'absent', 'late', 'excused', 'sick', 
  'authorized_leave', 'field_trip', 'early_departure'
]);

export const behaviorTypeEnum = pgEnum('behavior_type', [
  'positive', 'negative', 'neutral', 'award', 'violation'
]);

export const behaviorSeverityEnum = pgEnum('behavior_severity', [
  'minor', 'moderate', 'major', 'critical'
]);

// Notification Enums
export const notificationTypeEnum = pgEnum('notification_type', [
  'grade_published', 'new_assessment', 'attendance_alert', 
  'announcement', 'message', 'parent_meeting', 'fee_reminder', 
  'account_action', 'behavior_alert', 'system', 'promotion', 
  'timetable_change', 'emergency'
]);

export const notificationChannelEnum = pgEnum('notification_channel', [
  'in_app', 'email', 'sms', 'push', 'whatsapp'
]);

export const notificationPriorityEnum = pgEnum('notification_priority', [
  'low', 'normal', 'high', 'urgent', 'critical'
]);

// Document Enums
export const documentTypeEnum = pgEnum('document_type', ['file', 'folder', 'link', 'template']);
export const documentVisibilityEnum = pgEnum('document_visibility', ['private', 'shared', 'public', 'restricted']);

// Financial Enums
export const feeFrequencyEnum = pgEnum('fee_frequency', [
  'one_time', 'monthly', 'termly', 'yearly', 'quarterly', 'custom'
]);

export const feeStatusEnum = pgEnum('fee_status', [
  'pending', 'partial', 'paid', 'overdue', 'waived', 'cancelled'
]);

export const paymentMethodEnum = pgEnum('payment_method', [
  'cash', 'bank_transfer', 'cheque', 'card', 'online', 
  'mobile_money', 'installment', 'scholarship', 'discount'
]);

export const paymentStatusEnum = pgEnum('payment_status', [
  'pending', 'completed', 'failed', 'refunded', 'partially_refunded', 'cancelled'
]);

export const treasuryTypeEnum = pgEnum('treasury_type', ['cash', 'bank', 'wallet', 'online_gateway']);

// Promotion & Grading Enums
export const promotionStatusEnum = pgEnum('promotion_status', [
  'promoted', 'retained', 'transferred', 'graduated', 
  'conditional', 'expelled', 'withdrawn'
]);

export const promotionDecisionEnum = pgEnum('promotion_decision', ['auto', 'manual', 'override', 'committee']);

export const gradingSystemEnum = pgEnum('grading_system', [
  'percentage', 'gpa_4', 'gpa_5', 'letter', 'points'
]);

// Permission & Locking Enums
export const permissionScopeEnum = pgEnum('permission_scope', [
  'global', 'school', 'department', 'class', 'subject', 'student', 'own'
]);

export const lockTypeEnum = pgEnum('lock_type', [
  'monthly', 'mid_term', 'final_exam', 'year_end', 
  'grade_entry', 'attendance', 'fee', 'custom'
]);

export const lockStatusEnum = pgEnum('lock_status', ['locked', 'unlocked', 'pending', 'overridden']);

// Staff & HR Enums
export const employmentTypeEnum = pgEnum('employment_type', [
  'full_time', 'part_time', 'contract', 'intern', 'volunteer', 'substitute'
]);

export const employmentStatusEnum = pgEnum('employment_status', [
  'active', 'on_leave', 'suspended', 'terminated', 'resigned', 'retired'
]);

export const leaveTypeEnum = pgEnum('leave_type', [
  'annual', 'sick', 'emergency', 'maternity', 'paternity', 
  'unpaid', 'study', 'hajj', 'compassionate'
]);

export const leaveStatusEnum = pgEnum('leave_status', [
  'pending', 'approved', 'rejected', 'cancelled', 'taken'
]);

// Timetable Enums
export const dayOfWeekEnum = pgEnum('day_of_week', [
  'sunday', 'monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday'
]);

export const lessonStatusEnum = pgEnum('lesson_status', [
  'scheduled', 'active', 'completed', 'cancelled', 'substituted', 'rescheduled'
]);

// Certificate & Report Enums
export const templateCategoryEnum = pgEnum('template_category', [
  'certificate', 'report_card', 'transcript', 'id_card', 
  'letter', 'invoice', 'receipt', 'custom'
]);

export const reportTypeEnum = pgEnum('report_type', [
  'academic', 'attendance', 'financial', 'behavior', 
  'teacher_performance', 'class_analytics', 'custom'
]);

export const reportFormatEnum = pgEnum('report_format', [
  'pdf', 'excel', 'csv', 'json', 'html'
]);

// ==========================================
// SECTION 2: CORE IDENTITY TABLES
// ==========================================

// Organizations/Multi-tenancy Support
export const organizations = pgTable('organizations', {
  id: uuid('id').defaultRandom().primaryKey(),
  name: varchar('name', { length: 255 }).notNull(),
  code: varchar('code', { length: 50 }).unique(),
  slug: varchar('slug', { length: 100 }).unique().notNull(),
  logo: varchar('logo', { length: 500 }),
  address: text('address'),
  phone: varchar('phone', { length: 50 }),
  email: varchar('email', { length: 255 }),
  website: varchar('website', { length: 255 }),
  timezone: varchar('timezone', { length: 100 }).default('Asia/Aden').notNull(),
  locale: varchar('locale', { length: 10 }).default('ar').notNull(),
  currency: varchar('currency', { length: 3 }).default('YER').notNull(),
  taxNumber: varchar('tax_number', { length: 50 }),
  registrationNumber: varchar('registration_number', { length: 50 }),
  settings: jsonb('settings').default('{}'),
  features: jsonb('features').default('{}'),
  isActive: boolean('is_active').default(true).notNull(),
  isVerified: boolean('is_verified').default(false).notNull(),
  subscriptionPlan: varchar('subscription_plan', { length: 50 }).default('free'),
  subscriptionExpiresAt: timestamp('subscription_expires_at'),
  maxUsers: integer('max_users').default(100),
  maxStudents: integer('max_students').default(500),
  maxStorageGb: integer('max_storage_gb').default(10),
  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  orgSlugIdx: index('org_slug_idx').on(table.slug),
  orgActiveIdx: index('org_active_idx').on(table.isActive),
}));

// Unified Users Table - Identity Only
export const users = pgTable('users', {
  id: uuid('id').defaultRandom().primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  // Authentication
  email: varchar('email', { length: 255 }),
  emailVerified: boolean('email_verified').default(false).notNull(),
  emailVerifiedAt: timestamp('email_verified_at'),
  phone: varchar('phone', { length: 50 }),
  phoneVerified: boolean('phone_verified').default(false).notNull(),
  phoneVerifiedAt: timestamp('phone_verified_at'),

  // Password & Security
  passwordHash: text('password_hash'),
  passwordChangedAt: timestamp('password_changed_at'),
  passwordExpiresAt: timestamp('password_expires_at'),
  mfaEnabled: boolean('mfa_enabled').default(false).notNull(),
  mfaSecret: text('mfa_secret'),
  mfaBackupCodes: jsonb('mfa_backup_codes').$type<string[]>(),

  // Profile
  displayName: varchar('display_name', { length: 255 }),
  avatar: varchar('avatar', { length: 500 }),
  locale: varchar('locale', { length: 10 }).default('ar'),
  timezone: varchar('timezone', { length: 100 }).default('Asia/Aden'),

  // Status & Tracking
  status: userStatusEnum('status').default('pending_verification').notNull(),
  lastLoginAt: timestamp('last_login_at'),
  lastLoginIp: varchar('last_login_ip', { length: 45 }),
  loginCount: integer('login_count').default(0),
  failedLoginAttempts: integer('failed_login_attempts').default(0),
  lockedUntil: timestamp('locked_until'),

  // Metadata
  preferences: jsonb('preferences').default('{}'),
  metadata: jsonb('metadata').default('{}'),

  // Audit
  createdById: uuid('created_by_id').references((): AnyPgColumn => users.id, { onDelete: 'set null' }),
  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  userOrgIdx: index('user_org_idx').on(table.organizationId),
  userEmailIdx: index('user_email_idx').on(table.email).where(sql`${table.email} IS NOT NULL`),
  userPhoneIdx: index('user_phone_idx').on(table.phone).where(sql`${table.phone} IS NOT NULL`),
  userStatusIdx: index('user_status_idx').on(table.status),
  userOrgStatusIdx: index('user_org_status_idx').on(table.organizationId, table.status),
  userDeletedIdx: index('user_deleted_idx').on(table.deletedAt),
}));

// User Profiles - Separate from Identity for Flexibility
export const userProfiles = pgTable('user_profiles', {
  id: uuid('id').defaultRandom().primaryKey(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull().unique(),

  // Personal Info
  firstName: varchar('first_name', { length: 100 }),
  middleName: varchar('middle_name', { length: 100 }),
  lastName: varchar('last_name', { length: 100 }),
  fullName: varchar('full_name', { length: 255 }),
  nameAr: varchar('name_ar', { length: 255 }),
  nameEn: varchar('name_en', { length: 255 }),

  // Demographics
  sex: userSexEnum('sex').default('unspecified').notNull(),
  dateOfBirth: date('date_of_birth'),
  placeOfBirth: varchar('place_of_birth', { length: 255 }),
  nationality: varchar('nationality', { length: 100 }),
  nationalId: varchar('national_id', { length: 50 }),
  passportNumber: varchar('passport_number', { length: 50 }),

  // Contact
  address: text('address'),
  city: varchar('city', { length: 100 }),
  country: varchar('country', { length: 100 }),
  postalCode: varchar('postal_code', { length: 20 }),
  emergencyContactName: varchar('emergency_contact_name', { length: 255 }),
  emergencyContactPhone: varchar('emergency_contact_phone', { length: 50 }),
  emergencyContactRelation: varchar('emergency_contact_relation', { length: 50 }),

  // Additional
  bio: text('bio'),
  bloodType: varchar('blood_type', { length: 10 }),
  allergies: jsonb('allergies').$type<string[]>(),
  medicalConditions: jsonb('medical_conditions').$type<string[]>(),

  // Social
  socialLinks: jsonb('social_links').default('{}'),

  // Audit
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  profileNameIdx: index('profile_name_idx').on(table.fullName),
  profileNationalIdIdx: index('profile_national_id_idx').on(table.nationalId).where(sql`${table.nationalId} IS NOT NULL`),
}));

// External Accounts (OAuth, SSO)
export const userAccounts = pgTable('user_accounts', {
  id: uuid('id').defaultRandom().primaryKey(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  provider: identityProviderEnum('provider').notNull(),
  providerAccountId: varchar('provider_account_id', { length: 255 }).notNull(),
  providerAccountEmail: varchar('provider_account_email', { length: 255 }),
  providerAccountName: varchar('provider_account_name', { length: 255 }),
  providerAccountPicture: varchar('provider_account_picture', { length: 500 }),
  accessToken: text('access_token'),
  refreshToken: text('refresh_token'),
  idToken: text('id_token'),
  tokenType: varchar('token_type', { length: 50 }),
  scope: text('scope'),
  expiresAt: timestamp('expires_at'),
  metadata: jsonb('metadata').default('{}'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  accountProviderUnique: unique('account_provider_unique').on(table.provider, table.providerAccountId),
  accountUserIdx: index('account_user_idx').on(table.userId),
}));

// Sessions with Enhanced Security
export const userSessions = pgTable('user_sessions', {
  id: uuid('id').defaultRandom().primaryKey(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  token: varchar('token', { length: 255 }).unique().notNull(),
  refreshToken: varchar('refresh_token', { length: 255 }).unique(),

  // Device & Location
  deviceId: varchar('device_id', { length: 255 }),
  deviceType: varchar('device_type', { length: 50 }),
  deviceName: varchar('device_name', { length: 255 }),
  os: varchar('os', { length: 100 }),
  browser: varchar('browser', { length: 100 }),
  ipAddress: varchar('ip_address', { length: 45 }),
  userAgent: text('user_agent'),
  location: jsonb('location').default('{}'),

  // Security
  isCurrent: boolean('is_current').default(true).notNull(),
  isTrusted: boolean('is_trusted').default(false).notNull(),
  mfaVerified: boolean('mfa_verified').default(false).notNull(),

  // Timing
  expiresAt: timestamp('expires_at').notNull(),
  refreshExpiresAt: timestamp('refresh_expires_at'),
  lastActiveAt: timestamp('last_active_at').defaultNow(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  sessionUserIdx: index('session_user_idx').on(table.userId),
  sessionTokenIdx: index('session_token_idx').on(table.token),
  sessionDeviceIdx: index('session_device_idx').on(table.deviceId),
  sessionCurrentIdx: index('session_current_idx').on(table.userId, table.isCurrent),
}));

// Verification Codes (Email, Phone, Reset)
export const verifications = pgTable('verifications', {
  id: uuid('id').defaultRandom().primaryKey(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }),
  identifier: varchar('identifier', { length: 255 }).notNull(),
  value: varchar('value', { length: 255 }).notNull(),
  type: varchar('type', { length: 50 }).notNull(), // email_verification, phone_verification, password_reset, mfa_setup
  attempts: integer('attempts').default(0),
  maxAttempts: integer('max_attempts').default(5),
  expiresAt: timestamp('expires_at').notNull(),
  usedAt: timestamp('used_at'),
  ipAddress: varchar('ip_address', { length: 45 }),
  userAgent: text('user_agent'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
}, (table) => ({
  verificationIdentifierIdx: index('verification_identifier_idx').on(table.identifier, table.type),
  verificationUserIdx: index('verification_user_idx').on(table.userId),
}));


// ============================================================
// PART 2: 2 RBAC
// ============================================================

// ==========================================
// SECTION 3: RBAC PERMISSION SYSTEM
// ==========================================

// Permissions Master List
export const permissions = pgTable('permissions', {
  id: serial('id').primaryKey(),
  code: varchar('code', { length: 100 }).unique().notNull(),
  name: varchar('name', { length: 255 }).notNull(),
  description: text('description'),
  module: varchar('module', { length: 50 }).notNull(), // academics, finance, attendance, etc.
  action: varchar('action', { length: 50 }).notNull(), // create, read, update, delete, approve, export
  resource: varchar('resource', { length: 50 }).notNull(), // students, grades, fees, etc.
  isSystem: boolean('is_system').default(true).notNull(),
  isActive: boolean('is_active').default(true).notNull(),
  metadata: jsonb('metadata').default('{}'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
}, (table) => ({
  permissionCodeIdx: index('permission_code_idx').on(table.code),
  permissionModuleIdx: index('permission_module_idx').on(table.module),
  permissionResourceIdx: index('permission_resource_idx').on(table.resource),
}));

// Roles (Can be custom per organization)
export const roles = pgTable('roles', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  name: varchar('name', { length: 100 }).notNull(),
  displayName: varchar('display_name', { length: 255 }).notNull(),
  description: text('description'),
  color: varchar('color', { length: 20 }).default('#6366f1'),
  icon: varchar('icon', { length: 50 }),

  // Hierarchy
  level: integer('level').default(1).notNull(), // 1=lowest, 100=super_admin
  isSystem: boolean('is_system').default(false).notNull(),
  isDefault: boolean('is_default').default(false).notNull(),
  isActive: boolean('is_active').default(true).notNull(),

  // Permissions snapshot for fast lookup
  permissionsSnapshot: jsonb('permissions_snapshot').default('[]'),

  metadata: jsonb('metadata').default('{}'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
  deletedAt: timestamp('deleted_at'),
}, (table) => ({
  roleOrgIdx: index('role_org_idx').on(table.organizationId),
  roleOrgNameIdx: unique('role_org_name_idx').on(table.organizationId, table.name),
  roleLevelIdx: index('role_level_idx').on(table.level),
}));

// Role-Permission Mapping with Conditions
export const rolePermissions = pgTable('role_permissions', {
  id: serial('id').primaryKey(),
  roleId: integer('role_id').references(() => roles.id, { onDelete: 'cascade' }).notNull(),
  permissionId: integer('permission_id').references(() => permissions.id, { onDelete: 'cascade' }).notNull(),

  // Conditional Access
  conditions: jsonb('conditions').default('{}'), // { "classId": [1,2,3], "subjectId": [5,6] }
  scope: permissionScopeEnum('scope').default('global').notNull(),

  // Time-based
  validFrom: date('valid_from'),
  validUntil: date('valid_until'),

  isActive: boolean('is_active').default(true).notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
}, (table) => ({
  rolePermUnique: unique('role_permission_unique').on(table.roleId, table.permissionId),
  rolePermRoleIdx: index('role_perm_role_idx').on(table.roleId),
}));

// User-Role Assignment with Scope
export const userRoles = pgTable('user_roles', {
  id: serial('id').primaryKey(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  roleId: integer('role_id').references(() => roles.id, { onDelete: 'cascade' }).notNull(),

  // Scope Context
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }),
  schoolId: integer('school_id'), // For multi-school organizations
  departmentId: integer('department_id'),
  classId: integer('class_id'),
  subjectId: integer('subject_id'),

  // Scope Data
  scopeData: jsonb('scope_data').default('{}'), // Flexible scope definition

  // Time-based
  validFrom: date('valid_from'),
  validUntil: date('valid_until'),

  isPrimary: boolean('is_primary').default(false).notNull(),
  isActive: boolean('is_active').default(true).notNull(),
  assignedById: uuid('assigned_by_id').references(() => users.id, { onDelete: 'set null' }),
  assignedAt: timestamp('assigned_at').defaultNow().notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
}, (table) => ({
  userRoleUnique: unique('user_role_unique').on(table.userId, table.roleId, table.organizationId),
  userRoleUserIdx: index('user_role_user_idx').on(table.userId),
  userRoleRoleIdx: index('user_role_role_idx').on(table.roleId),
  userRoleOrgIdx: index('user_role_org_idx').on(table.organizationId),
  userRoleClassIdx: index('user_role_class_idx').on(table.classId),
}));

// Direct User Permissions (Override/Exceptions)
export const userPermissions = pgTable('user_permissions', {
  id: serial('id').primaryKey(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  permissionId: integer('permission_id').references(() => permissions.id, { onDelete: 'cascade' }).notNull(),

  grantType: varchar('grant_type', { length: 20 }).default('allow').notNull(), // allow, deny
  conditions: jsonb('conditions').default('{}'),
  scope: permissionScopeEnum('scope').default('global').notNull(),
  scopeData: jsonb('scope_data').default('{}'),

  validFrom: date('valid_from'),
  validUntil: date('valid_until'),

  reason: text('reason'),
  grantedById: uuid('granted_by_id').references(() => users.id, { onDelete: 'set null' }),
  grantedAt: timestamp('granted_at').defaultNow().notNull(),
  isActive: boolean('is_active').default(true).notNull(),
}, (table) => ({
  userPermUnique: unique('user_permission_unique').on(table.userId, table.permissionId),
  userPermUserIdx: index('user_perm_user_idx').on(table.userId),
}));

// Permission Audit Log
export const permissionAuditLogs = pgTable('permission_audit_logs', {
  id: serial('id').primaryKey(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'set null' }),
  targetUserId: uuid('target_user_id').references(() => users.id, { onDelete: 'set null' }),
  roleId: integer('role_id'),
  permissionId: integer('permission_id'),
  action: varchar('action', { length: 50 }).notNull(), // granted, revoked, modified
  oldData: jsonb('old_data'),
  newData: jsonb('new_data'),
  reason: text('reason'),
  ipAddress: varchar('ip_address', { length: 45 }),
  userAgent: text('user_agent'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
}, (table) => ({
  permAuditUserIdx: index('perm_audit_user_idx').on(table.userId),
  permAuditTargetIdx: index('perm_audit_target_idx').on(table.targetUserId),
  permAuditCreatedIdx: index('perm_audit_created_idx').on(table.createdAt),
}));


// ============================================================
// PART 3: 3 ACADEMIC
// ============================================================

// ==========================================
// SECTION 4: ACADEMIC STRUCTURE
// ==========================================

// Schools (For Multi-School Organizations)
export const schools = pgTable('schools', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  name: varchar('name', { length: 255 }).notNull(),
  nameAr: varchar('name_ar', { length: 255 }),
  nameEn: varchar('name_en', { length: 255 }),
  code: varchar('code', { length: 50 }),
  type: varchar('type', { length: 50 }).default('main'), // main, branch, virtual
  level: varchar('level', { length: 50 }), // primary, middle, high, university
  address: text('address'),
  phone: varchar('phone', { length: 50 }),
  email: varchar('email', { length: 255 }),
  principalId: uuid('principal_id').references(() => users.id, { onDelete: 'set null' }),
  logo: varchar('logo', { length: 500 }),
  settings: jsonb('settings').default('{}'),
  isActive: boolean('is_active').default(true).notNull(),
  sortOrder: integer('sort_order').default(0),
  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  schoolOrgIdx: index('school_org_idx').on(table.organizationId),
  schoolCodeIdx: unique('school_code_idx').on(table.organizationId, table.code),
  schoolPrincipalIdx: index('school_principal_idx').on(table.principalId),
}));

// Departments (Academic & Administrative)
export const departments = pgTable('departments', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  schoolId: integer('school_id').references(() => schools.id, { onDelete: 'cascade' }),

  name: varchar('name', { length: 255 }).notNull(),
  nameAr: varchar('name_ar', { length: 255 }),
  code: varchar('code', { length: 50 }),
  type: varchar('type', { length: 50 }).notNull(), // academic, administrative, support

  headId: uuid('head_id').references(() => users.id, { onDelete: 'set null' }),
  parentDepartmentId: integer('parent_department_id').references((): AnyPgColumn => departments.id, { onDelete: 'set null' }),

  description: text('description'),
  goals: text('goals'),
  budget: numeric('budget', { precision: 15, scale: 2 }),

  isActive: boolean('is_active').default(true).notNull(),
  sortOrder: integer('sort_order').default(0),
  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  deptOrgIdx: index('dept_org_idx').on(table.organizationId),
  deptSchoolIdx: index('dept_school_idx').on(table.schoolId),
  deptParentIdx: index('dept_parent_idx').on(table.parentDepartmentId),
  deptHeadIdx: index('dept_head_idx').on(table.headId),
}));

// Academic Years
export const academicYears = pgTable('academic_years', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  schoolId: integer('school_id').references(() => schools.id, { onDelete: 'cascade' }),

  name: varchar('name', { length: 50 }).notNull(), // e.g., "2025-2026"
  nameAr: varchar('name_ar', { length: 50 }),
  code: varchar('code', { length: 20 }),

  startDate: date('start_date').notNull(),
  endDate: date('end_date').notNull(),

  // Status
  status: termStatusEnum('status').default('upcoming').notNull(),
  isCurrent: boolean('is_current').default(false).notNull(),
  isEnrollmentOpen: boolean('is_enrollment_open').default(false).notNull(),

  // Configuration
  gradingSystem: gradingSystemEnum('grading_system').default('percentage').notNull(),
  passingGrade: numeric('passing_grade', { precision: 5, scale: 2 }).default('50.00'),
  maxGrade: numeric('max_grade', { precision: 5, scale: 2 }).default('100.00'),

  // Financial
  registrationFee: numeric('registration_fee', { precision: 10, scale: 2 }),
  lateRegistrationFee: numeric('late_registration_fee', { precision: 10, scale: 2 }),

  settings: jsonb('settings').default('{}'),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  yearOrgIdx: index('year_org_idx').on(table.organizationId),
  yearSchoolIdx: index('year_school_idx').on(table.schoolId),
  yearCurrentIdx: index('year_current_idx').on(table.organizationId, table.isCurrent),
  yearStatusIdx: index('year_status_idx').on(table.status),
  yearDateRangeIdx: index('year_date_range_idx').on(table.startDate, table.endDate),
}));

// Academic Terms (Semesters/Quarters)
export const academicTerms = pgTable('academic_terms', {
  id: serial('id').primaryKey(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  name: varchar('name', { length: 50 }).notNull(), // "First Term", "Second Term"
  nameAr: varchar('name_ar', { length: 50 }),
  code: varchar('code', { length: 20 }),

  startDate: date('start_date').notNull(),
  endDate: date('end_date').notNull(),

  // Exam Period
  examStartDate: date('exam_start_date'),
  examEndDate: date('exam_end_date'),

  // Status
  status: termStatusEnum('status').default('upcoming').notNull(),
  isCurrent: boolean('is_current').default(false).notNull(),

  // Weight in final grade
  weight: numeric('weight', { precision: 5, scale: 2 }).default('50.00').notNull(),

  // Sequence
  sequence: integer('sequence').default(1).notNull(),

  settings: jsonb('settings').default('{}'),
  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  termYearIdx: index('term_year_idx').on(table.academicYearId),
  termCurrentIdx: index('term_current_idx').on(table.academicYearId, table.isCurrent),
  termStatusIdx: index('term_status_idx').on(table.status),
  termDateIdx: index('term_date_idx').on(table.startDate, table.endDate),
}));

// Grading Scales (A: 90-100, B: 80-89...)
export const gradingScales = pgTable('grading_scales', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }),

  name: varchar('name', { length: 100 }).notNull(),
  system: gradingSystemEnum('system').default('percentage').notNull(),

  minScore: numeric('min_score', { precision: 6, scale: 2 }).notNull(),
  maxScore: numeric('max_score', { precision: 6, scale: 2 }).notNull(),
  grade: varchar('grade', { length: 10 }).notNull(), // A, B, C... or 4.0, 3.0...
  gradePoints: numeric('grade_points', { precision: 4, scale: 2 }),
  label: varchar('label', { length: 100 }), // "Excellent", "Very Good"
  labelAr: varchar('label_ar', { length: 100 }),
  color: varchar('color', { length: 20 }).default('#10b981'),

  isPassing: boolean('is_passing').default(true).notNull(),
  isDefault: boolean('is_default').default(false).notNull(),
  sortOrder: integer('sort_order').default(0),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  scaleOrgIdx: index('scale_org_idx').on(table.organizationId),
  scaleYearIdx: index('scale_year_idx').on(table.academicYearId),
  scaleGradeIdx: index('scale_grade_idx').on(table.grade),
}));

// Subjects
export const subjects = pgTable('subjects', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  departmentId: integer('department_id').references(() => departments.id, { onDelete: 'set null' }),

  name: varchar('name', { length: 255 }).notNull(),
  nameAr: varchar('name_ar', { length: 255 }),
  nameEn: varchar('name_en', { length: 255 }),
  code: varchar('code', { length: 50 }),

  description: text('description'),
  color: varchar('color', { length: 20 }).default('#6366f1'),
  icon: varchar('icon', { length: 50 }),

  // Academic Properties
  isCore: boolean('is_core').default(true).notNull(),
  isElective: boolean('is_elective').default(false).notNull(),
  creditHours: numeric('credit_hours', { precision: 4, scale: 2 }).default('1.00'),

  // Configuration
  requiresPractical: boolean('requires_practical').default(false).notNull(),
  requiresLab: boolean('requires_lab').default(false).notNull(),
  maxStudentsPerClass: integer('max_students_per_class'),

  prerequisites: jsonb('prerequisites').default('[]'), // [subject_id, subject_id]
  learningOutcomes: jsonb('learning_outcomes').default('[]'),

  sortOrder: integer('sort_order').default(0).notNull(),
  isActive: boolean('is_active').default(true).notNull(),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  subjectOrgIdx: index('subject_org_idx').on(table.organizationId),
  subjectCodeIdx: unique('subject_code_idx').on(table.organizationId, table.code),
  subjectDeptIdx: index('subject_dept_idx').on(table.departmentId),
  subjectActiveIdx: index('subject_active_idx').on(table.isActive),
  subjectSortIdx: index('subject_sort_idx').on(table.sortOrder),
}));

// Classes/Classrooms (Physical & Virtual)
export const classes = pgTable('classes', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  schoolId: integer('school_id').references(() => schools.id, { onDelete: 'cascade' }),

  name: varchar('name', { length: 100 }).notNull(),
  nameAr: varchar('name_ar', { length: 100 }),
  code: varchar('code', { length: 50 }),

  type: classTypeEnum('type').default('classroom').notNull(),
  level: varchar('level', { length: 50 }), // Grade 1, Grade 2...
  division: varchar('division', { length: 20 }), // A, B, C...

  // Capacity
  capacity: integer('capacity').default(30),
  minStudents: integer('min_students').default(5),

  // Hierarchy
  parentClassId: integer('parent_class_id').references((): AnyPgColumn => classes.id, { onDelete: 'set null' }),

  // Homeroom Teacher
  homeroomTeacherId: uuid('homeroom_teacher_id').references(() => users.id, { onDelete: 'set null' }),

  // Room Details
  roomNumber: varchar('room_number', { length: 50 }),
  floor: integer('floor'),
  building: varchar('building', { length: 100 }),

  // Visual
  color: varchar('color', { length: 20 }).default('#e5e7eb'),
  coverImage: varchar('cover_image', { length: 500 }),

  // Academic Year Context
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),

  settings: jsonb('settings').default('{}'),
  sortOrder: integer('sort_order').default(0).notNull(),
  isActive: boolean('is_active').default(true).notNull(),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  classOrgIdx: index('class_org_idx').on(table.organizationId),
  classSchoolIdx: index('class_school_idx').on(table.schoolId),
  classCodeIdx: unique('class_code_idx').on(table.organizationId, table.code),
  classParentIdx: index('class_parent_idx').on(table.parentClassId),
  classTeacherIdx: index('class_teacher_idx').on(table.homeroomTeacherId),
  classYearIdx: index('class_year_idx').on(table.academicYearId),
  classActiveIdx: index('class_active_idx').on(table.isActive),
  classSortIdx: index('class_sort_idx').on(table.sortOrder),
}));

// Class-Subject Assignments (What subjects are taught in each class)
export const classSubjects = pgTable('class_subjects', {
  id: serial('id').primaryKey(),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'cascade' }).notNull(),
  subjectId: integer('subject_id').references(() => subjects.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  // Teacher Assignment
  primaryTeacherId: uuid('primary_teacher_id').references(() => users.id, { onDelete: 'set null' }),
  secondaryTeacherId: uuid('secondary_teacher_id').references(() => users.id, { onDelete: 'set null' }),

  // Weekly Hours
  weeklyHours: numeric('weekly_hours', { precision: 4, scale: 2 }).default('2.00'),
  theoryHours: numeric('theory_hours', { precision: 4, scale: 2 }).default('1.50'),
  practicalHours: numeric('practical_hours', { precision: 4, scale: 2 }).default('0.50'),

  // Assessment Configuration
  assessmentConfig: jsonb('assessment_config').default('{}'), // { exams: 40%, quizzes: 20%, homework: 15%... }

  isActive: boolean('is_active').default(true).notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  classSubjectUnique: unique('class_subject_year_unique').on(table.classId, table.subjectId, table.academicYearId),
  classSubjectClassIdx: index('cs_class_idx').on(table.classId),
  classSubjectSubjectIdx: index('cs_subject_idx').on(table.subjectId),
  classSubjectYearIdx: index('cs_year_idx').on(table.academicYearId),
  classSubjectTeacherIdx: index('cs_teacher_idx').on(table.primaryTeacherId),
}));


// ============================================================
// PART 4: 4 STUDENTS
// ============================================================

// ==========================================
// SECTION 5: STUDENTS & ENROLLMENTS
// ==========================================

// Students Table - Optimized for Scale
export const students = pgTable('students', {
  userId: uuid('user_id').primaryKey().references(() => users.id, { onDelete: 'cascade' }),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  // Student Identity
  studentNumber: varchar('student_number', { length: 50 }).notNull(),
  previousStudentNumber: varchar('previous_student_number', { length: 50 }),

  // Security
  secretNumber: varchar('secret_number', { length: 50 }),
  seatingNumber: varchar('seating_number', { length: 50 }),
  examNumber: varchar('exam_number', { length: 50 }),

  // Academic Status
  currentClassId: integer('current_class_id').references(() => classes.id, { onDelete: 'set null' }),
  currentAcademicYearId: integer('current_academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
  status: studentStatusEnum('status').default('active').notNull(),
  enrollmentDate: date('enrollment_date'),

  // Academic History
  previousSchool: varchar('previous_school', { length: 255 }),
  previousSchoolAddress: text('previous_school_address'),
  transferDate: date('transfer_date'),
  transferReason: text('transfer_reason'),

  // Guardian Info
  guardianName: varchar('guardian_name', { length: 255 }),
  guardianPhone: varchar('guardian_phone', { length: 50 }),
  guardianRelation: varchar('guardian_relation', { length: 50 }),
  guardianEmail: varchar('guardian_email', { length: 255 }),

  // Medical & Emergency
  bloodType: varchar('blood_type', { length: 10 }),
  allergies: jsonb('allergies').$type<string[]>(),
  medicalConditions: jsonb('medical_conditions').$type<string[]>(),
  emergencyContact: jsonb('emergency_contact').default('{}'),

  // Transportation
  transportType: varchar('transport_type', { length: 50 }), // bus, private, walk
  transportRoute: varchar('transport_route', { length: 100 }),
  transportFee: numeric('transport_fee', { precision: 10, scale: 2 }),

  // Documents
  documents: jsonb('documents').default('{}'), // {birth_cert: "url", photo: "url"}

  // Notes & Metadata
  notes: text('notes'),
  internalNotes: text('internal_notes'), // Not visible to parents
  stats: jsonb('stats').default('{}'), // Cached statistics
  metadata: jsonb('metadata').default('{}'),

  // Soft Delete
  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  // Primary Lookups
  studentOrgIdx: index('student_org_idx').on(table.organizationId),
  studentNumIdx: unique('student_num_org_unique').on(table.organizationId, table.studentNumber),
  studentClassIdx: index('student_class_idx').on(table.currentClassId),
  studentYearIdx: index('student_year_idx').on(table.currentAcademicYearId),
  studentStatusIdx: index('student_status_idx').on(table.status),

  // Security Lookups
  studentSecretIdx: index('student_secret_idx').on(table.secretNumber).where(sql`${table.secretNumber} IS NOT NULL`),
  studentSeatingIdx: index('student_seating_idx').on(table.seatingNumber).where(sql`${table.seatingNumber} IS NOT NULL`),
  studentExamIdx: index('student_exam_idx').on(table.examNumber).where(sql`${table.examNumber} IS NOT NULL`),

  // GIN indexes for JSONB
  studentMetadataIdx: index('student_metadata_gin_idx').using('gin', table.metadata),
  studentStatsIdx: index('student_stats_gin_idx').using('gin', table.stats),

  // Composite for common queries
  studentOrgStatusIdx: index('student_org_status_idx').on(table.organizationId, table.status),
  studentClassStatusIdx: index('student_class_status_idx').on(table.currentClassId, table.status),
}));

// Student Enrollments (Historical)
export const enrollments = pgTable('enrollments', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  // Enrollment Details
  enrollmentNumber: varchar('enrollment_number', { length: 50 }),
  enrollmentDate: date('enrollment_date').notNull(),
  enrollmentType: varchar('enrollment_type', { length: 50 }).default('new').notNull(), // new, transfer, re_enrollment

  // Status
  status: enrollmentStatusEnum('status').default('active').notNull(),
  withdrawalDate: date('withdrawal_date'),
  withdrawalReason: text('withdrawal_reason'),

  // Academic Results for this enrollment
  finalGrade: numeric('final_grade', { precision: 6, scale: 2 }),
  finalGpa: numeric('final_gpa', { precision: 4, scale: 2 }),
  finalRank: integer('final_rank'),
  totalAbsences: integer('total_absences').default(0),
  totalLates: integer('total_lates').default(0),

  // Promotion
  isPromoted: boolean('is_promoted'),
  promotionDate: date('promotion_date'),
  promotionNotes: text('promotion_notes'),

  // Metadata
  metadata: jsonb('metadata').default('{}'),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  // Unique constraint
  enrollmentUnique: unique('enrollment_unique').on(table.studentId, table.classId, table.academicYearId),

  // Indexes
  enrollmentStudentIdx: index('enrollment_student_idx').on(table.studentId),
  enrollmentClassIdx: index('enrollment_class_idx').on(table.classId),
  enrollmentYearIdx: index('enrollment_year_idx').on(table.academicYearId),
  enrollmentStatusIdx: index('enrollment_status_idx').on(table.status),
  enrollmentDateIdx: index('enrollment_date_idx').on(table.enrollmentDate),

  // Composite
  enrollmentClassYearIdx: index('enrollment_class_year_idx').on(table.classId, table.academicYearId),
  enrollmentStudentYearIdx: index('enrollment_student_year_idx').on(table.studentId, table.academicYearId),
}));

// Student Guardians/Parents (Many-to-Many with Relationship Types)
export const studentGuardians = pgTable('student_guardians', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  guardianId: uuid('guardian_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),

  // Relationship
  relationship: varchar('relationship', { length: 50 }).default('parent').notNull(),
  relationshipAr: varchar('relationship_ar', { length: 50 }),
  isPrimary: boolean('is_primary').default(false).notNull(),
  isEmergencyContact: boolean('is_emergency_contact').default(false).notNull(),
  isAuthorizedPickup: boolean('is_authorized_pickup').default(true).notNull(),

  // Contact Preferences
  canViewGrades: boolean('can_view_grades').default(true).notNull(),
  canViewAttendance: boolean('can_view_attendance').default(true).notNull(),
  canViewBehavior: boolean('can_view_behavior').default(true).notNull(),
  canViewFinance: boolean('can_view_finance').default(true).notNull(),
  canReceiveNotifications: boolean('can_receive_notifications').default(true).notNull(),

  // Financial Responsibility
  isPayer: boolean('is_payer').default(false).notNull(),
  paymentSharePercent: numeric('payment_share_percent', { precision: 5, scale: 2 }).default('100.00'),

  // Employment
  occupation: varchar('occupation', { length: 255 }),
  employer: varchar('employer', { length: 255 }),
  workPhone: varchar('work_phone', { length: 50 }),

  notes: text('notes'),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  guardianUnique: unique('student_guardian_unique').on(table.studentId, table.guardianId),
  guardianStudentIdx: index('guardian_student_idx').on(table.studentId),
  guardianGuardianIdx: index('guardian_guardian_idx').on(table.guardianId),
  guardianPrimaryIdx: index('guardian_primary_idx').on(table.studentId, table.isPrimary),
  guardianPayerIdx: index('guardian_payer_idx').on(table.studentId, table.isPayer),
}));

// Student Documents & Records
export const studentDocuments = pgTable('student_documents', {
  id: uuid('id').defaultRandom().primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),

  name: varchar('name', { length: 255 }).notNull(),
  type: varchar('type', { length: 50 }).notNull(), // birth_certificate, id_card, photo, medical_report...

  fileUrl: text('file_url'),
  fileKey: text('file_key'),
  fileSize: integer('file_size'),
  mimeType: varchar('mime_type', { length: 100 }),

  issuedBy: varchar('issued_by', { length: 255 }),
  issuedDate: date('issued_date'),
  expiryDate: date('expiry_date'),

  isVerified: boolean('is_verified').default(false).notNull(),
  verifiedById: uuid('verified_by_id').references(() => users.id, { onDelete: 'set null' }),
  verifiedAt: timestamp('verified_at'),

  notes: text('notes'),
  metadata: jsonb('metadata').default('{}'),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  docStudentIdx: index('doc_student_idx').on(table.studentId),
  docTypeIdx: index('doc_type_idx').on(table.type),
  docVerifiedIdx: index('doc_verified_idx').on(table.isVerified),
}));


// ============================================================
// PART 5: 5 STAFF
// ============================================================

// ==========================================
// SECTION 6: STAFF, TEACHERS & HR
// ==========================================

// Staff Members (Teachers, Administrators, Support)
export const staffMembers = pgTable('staff_members', {
  id: serial('id').primaryKey(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull().unique(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  schoolId: integer('school_id').references(() => schools.id, { onDelete: 'set null' }),
  departmentId: integer('department_id').references(() => departments.id, { onDelete: 'set null' }),

  // Employment
  employeeNumber: varchar('employee_number', { length: 50 }).notNull(),
  employmentType: employmentTypeEnum('employment_type').default('full_time').notNull(),
  employmentStatus: employmentStatusEnum('employment_status').default('active').notNull(),

  // Position
  jobTitle: varchar('job_title', { length: 255 }).notNull(),
  jobTitleAr: varchar('job_title_ar', { length: 255 }),
  jobLevel: varchar('job_level', { length: 50 }), // junior, senior, lead...

  // Dates
  hireDate: date('hire_date').notNull(),
  contractStartDate: date('contract_start_date'),
  contractEndDate: date('contract_end_date'),
  probationEndDate: date('probation_end_date'),
  terminationDate: date('termination_date'),
  terminationReason: text('termination_reason'),

  // Qualifications
  highestDegree: varchar('highest_degree', { length: 100 }),
  fieldOfStudy: varchar('field_of_study', { length: 255 }),
  university: varchar('university', { length: 255 }),
  graduationYear: integer('graduation_year'),
  certifications: jsonb('certifications').$type<string[]>(),
  skills: jsonb('skills').$type<string[]>(),

  // Financial
  baseSalary: numeric('base_salary', { precision: 12, scale: 2 }),
  currency: varchar('currency', { length: 3 }).default('YER'),
  bankAccountNumber: varchar('bank_account_number', { length: 100 }),
  bankName: varchar('bank_name', { length: 255 }),
  taxId: varchar('tax_id', { length: 50 }),

  // Benefits
  healthInsurance: boolean('health_insurance').default(false),
  socialSecurity: boolean('social_security').default(false),
  pension: boolean('pension').default(false),

  // Work Schedule
  workSchedule: jsonb('work_schedule').default('{}'), // {days: [1,2,3,4,5], hours: 8}
  maxTeachingHours: integer('max_teaching_hours').default(24),

  // Emergency
  emergencyContactName: varchar('emergency_contact_name', { length: 255 }),
  emergencyContactPhone: varchar('emergency_contact_phone', { length: 50 }),
  emergencyContactRelation: varchar('emergency_contact_relation', { length: 50 }),

  // Documents
  documents: jsonb('documents').default('{}'),

  notes: text('notes'),
  metadata: jsonb('metadata').default('{}'),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  staffOrgIdx: index('staff_org_idx').on(table.organizationId),
  staffSchoolIdx: index('staff_school_idx').on(table.schoolId),
  staffDeptIdx: index('staff_dept_idx').on(table.departmentId),
  staffEmpNumIdx: unique('staff_emp_num_unique').on(table.organizationId, table.employeeNumber),
  staffStatusIdx: index('staff_status_idx').on(table.employmentStatus),
  staffTypeIdx: index('staff_type_idx').on(table.employmentType),
  staffHireDateIdx: index('staff_hire_date_idx').on(table.hireDate),
}));

// Teacher Specializations & Assignments
export const teacherSubjects = pgTable('teacher_subjects', {
  id: serial('id').primaryKey(),
  staffId: integer('staff_id').references(() => staffMembers.id, { onDelete: 'cascade' }).notNull(),
  subjectId: integer('subject_id').references(() => subjects.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  // Competency
  competencyLevel: varchar('competency_level', { length: 50 }).default('intermediate'), // beginner, intermediate, expert
  isPrimary: boolean('is_primary').default(true).notNull(),
  isQualified: boolean('is_qualified').default(true).notNull(),

  // Assignment Details
  maxClasses: integer('max_classes').default(5),
  notes: text('notes'),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  teacherSubUnique: unique('teacher_subject_year_unique').on(table.staffId, table.subjectId, table.academicYearId),
  teacherSubStaffIdx: index('ts_staff_idx').on(table.staffId),
  teacherSubSubjectIdx: index('ts_subject_idx').on(table.subjectId),
  teacherSubYearIdx: index('ts_year_idx').on(table.academicYearId),
}));

// Teacher-Class Assignments (Homeroom, Subject Teaching)
export const teacherClasses = pgTable('teacher_classes', {
  id: serial('id').primaryKey(),
  staffId: integer('staff_id').references(() => staffMembers.id, { onDelete: 'cascade' }).notNull(),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  // Role in Class
  role: varchar('role', { length: 50 }).default('subject_teacher').notNull(), // homeroom, subject_teacher, assistant, substitute
  subjectId: integer('subject_id').references(() => subjects.id, { onDelete: 'set null' }),

  // Schedule
  weeklyHours: numeric('weekly_hours', { precision: 4, scale: 2 }).default('0'),

  isPrimary: boolean('is_primary').default(false).notNull(),
  isActive: boolean('is_active').default(true).notNull(),

  assignedAt: timestamp('assigned_at').defaultNow().notNull(),
  assignedById: uuid('assigned_by_id').references(() => users.id, { onDelete: 'set null' }),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  teacherClassUnique: unique('teacher_class_year_unique').on(table.staffId, table.classId, table.academicYearId, table.role),
  teacherClassStaffIdx: index('tc_staff_idx').on(table.staffId),
  teacherClassClassIdx: index('tc_class_idx').on(table.classId),
  teacherClassYearIdx: index('tc_year_idx').on(table.academicYearId),
  teacherClassSubjectIdx: index('tc_subject_idx').on(table.subjectId),
  teacherClassRoleIdx: index('tc_role_idx').on(table.role),
}));

// Leave Management
export const staffLeaves = pgTable('staff_leaves', {
  id: serial('id').primaryKey(),
  staffId: integer('staff_id').references(() => staffMembers.id, { onDelete: 'cascade' }).notNull(),

  leaveType: leaveTypeEnum('leave_type').notNull(),
  status: leaveStatusEnum('status').default('pending').notNull(),

  startDate: date('start_date').notNull(),
  endDate: date('end_date').notNull(),
  days: numeric('days', { precision: 5, scale: 2 }).notNull(),

  reason: text('reason').notNull(),
  attachmentUrl: text('attachment_url'),

  // Approval Workflow
  requestedAt: timestamp('requested_at').defaultNow().notNull(),
  requestedById: uuid('requested_by_id').references(() => users.id, { onDelete: 'set null' }),

  reviewedById: uuid('reviewed_by_id').references(() => users.id, { onDelete: 'set null' }),
  reviewedAt: timestamp('reviewed_at'),
  reviewNotes: text('review_notes'),

  approvedById: uuid('approved_by_id').references(() => users.id, { onDelete: 'set null' }),
  approvedAt: timestamp('approved_at'),
  approvalNotes: text('approval_notes'),

  isPaid: boolean('is_paid').default(true).notNull(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  leaveStaffIdx: index('leave_staff_idx').on(table.staffId),
  leaveStatusIdx: index('leave_status_idx').on(table.status),
  leaveTypeIdx: index('leave_type_idx').on(table.leaveType),
  leaveDateIdx: index('leave_date_idx').on(table.startDate, table.endDate),
  leaveRequestedIdx: index('leave_requested_idx').on(table.requestedAt),
}));

// Leave Balance (Annual Tracking)
export const leaveBalances = pgTable('leave_balances', {
  id: serial('id').primaryKey(),
  staffId: integer('staff_id').references(() => staffMembers.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  leaveType: leaveTypeEnum('leave_type').notNull(),

  entitledDays: numeric('entitled_days', { precision: 5, scale: 2 }).default('0').notNull(),
  usedDays: numeric('used_days', { precision: 5, scale: 2 }).default('0').notNull(),
  remainingDays: numeric('remaining_days', { precision: 5, scale: 2 }).default('0').notNull(),
  carriedOverDays: numeric('carried_over_days', { precision: 5, scale: 2 }).default('0'),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  balanceUnique: unique('leave_balance_unique').on(table.staffId, table.academicYearId, table.leaveType),
  balanceStaffIdx: index('balance_staff_idx').on(table.staffId),
  balanceYearIdx: index('balance_year_idx').on(table.academicYearId),
}));

// Staff Attendance
export const staffAttendance = pgTable('staff_attendance', {
  id: serial('id').primaryKey(),
  staffId: integer('staff_id').references(() => staffMembers.id, { onDelete: 'cascade' }).notNull(),

  date: date('date').notNull(),
  status: attendanceStatusEnum('status').default('present').notNull(),

  checkIn: time('check_in'),
  checkOut: time('check_out'),

  workHours: numeric('work_hours', { precision: 4, scale: 2 }),
  overtimeHours: numeric('overtime_hours', { precision: 4, scale: 2 }).default('0'),

  isLate: boolean('is_late').default(false).notNull(),
  lateMinutes: integer('late_minutes').default(0),
  isEarlyDeparture: boolean('is_early_departure').default(false).notNull(),

  notes: text('notes'),
  markedById: uuid('marked_by_id').references(() => users.id, { onDelete: 'set null' }),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  staffAttUnique: unique('staff_attendance_unique').on(table.staffId, table.date),
  staffAttStaffIdx: index('staff_att_staff_idx').on(table.staffId),
  staffAttDateIdx: index('staff_att_date_idx').on(table.date),
  staffAttStatusIdx: index('staff_att_status_idx').on(table.status),
}));


// ============================================================
// PART 6: 6 ASSESSMENT
// ============================================================

// ==========================================
// SECTION 7: ASSESSMENT & GRADING SYSTEM (OPTIMIZED)
// ==========================================

// Grade Groups (Exam: 40%, Quiz: 20%, Homework: 15%, etc.)
export const gradeGroups = pgTable('grade_groups', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  name: varchar('name', { length: 100 }).notNull(),
  nameAr: varchar('name_ar', { length: 100 }),
  code: varchar('code', { length: 50 }),

  // Weight in final grade (must sum to 100% across all groups for a subject)
  weight: numeric('weight', { precision: 5, scale: 2 }).notNull(),

  // Configuration
  dropLowest: integer('drop_lowest').default(0), // Drop lowest N scores
  minItems: integer('min_items').default(1),
  maxItems: integer('max_items'),

  // Aggregation Method
  aggregationMethod: varchar('aggregation_method', { length: 50 }).default('average').notNull(), // average, sum, weighted_average, best_of

  isActive: boolean('is_active').default(true).notNull(),
  sortOrder: integer('sort_order').default(0),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  ggOrgIdx: index('gg_org_idx').on(table.organizationId),
  ggYearIdx: index('gg_year_idx').on(table.academicYearId),
  ggCodeIdx: unique('gg_code_unique').on(table.organizationId, table.academicYearId, table.code),
}));

// Class-Subject-GradeGroup Configuration
export const classSubjectGradeGroups = pgTable('class_subject_grade_groups', {
  id: serial('id').primaryKey(),
  classSubjectId: integer('class_subject_id').references(() => classSubjects.id, { onDelete: 'cascade' }).notNull(),
  gradeGroupId: integer('grade_group_id').references(() => gradeGroups.id, { onDelete: 'cascade' }).notNull(),

  // Override weight for this specific class-subject
  weight: numeric('weight', { precision: 5, scale: 2 }),

  isActive: boolean('is_active').default(true).notNull(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  csggUnique: unique('csgg_unique').on(table.classSubjectId, table.gradeGroupId),
  csggClassSubIdx: index('csgg_class_sub_idx').on(table.classSubjectId),
  csggGroupIdx: index('csgg_group_idx').on(table.gradeGroupId),
}));

// Assessment Items (Exams, Quizzes, etc.)
export const assessmentItems = pgTable('assessment_items', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  // Academic Context
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'cascade' }).notNull(),

  // Subject & Class
  classSubjectId: integer('class_subject_id').references(() => classSubjects.id, { onDelete: 'cascade' }).notNull(),
  gradeGroupId: integer('grade_group_id').references(() => gradeGroups.id, { onDelete: 'set null' }),

  // Details
  title: varchar('title', { length: 255 }).notNull(),
  titleAr: varchar('title_ar', { length: 255 }),
  description: text('description'),
  type: assessmentTypeEnum('type').notNull(),

  // Scoring
  maxScore: numeric('max_score', { precision: 6, scale: 2 }).notNull(),
  passingScore: numeric('passing_score', { precision: 6, scale: 2 }),
  weight: numeric('weight', { precision: 5, scale: 2 }).default('1.0').notNull(),

  // Schedule
  assessmentDate: date('assessment_date'),
  dueDate: date('due_date'),

  // Status
  isPublished: boolean('is_published').default(false).notNull(),
  publishedAt: timestamp('published_at'),
  publishedById: uuid('published_by_id').references(() => users.id, { onDelete: 'set null' }),

  isLocked: boolean('is_locked').default(false).notNull(),
  lockedAt: timestamp('locked_at'),
  lockedById: uuid('locked_by_id').references(() => users.id, { onDelete: 'set null' }),

  // Grading
  gradingMethod: varchar('grading_method', { length: 50 }).default('manual').notNull(), // manual, auto, rubric
  rubricId: integer('rubric_id'), // Reference to rubric if used

  // Statistics (Cached)
  stats: jsonb('stats').default('{}'), // {mean: 75.5, median: 78, stdDev: 12.3, highest: 100, lowest: 20, count: 45}

  // Metadata
  instructions: text('instructions'),
  attachments: jsonb('attachments').default('[]'),
  metadata: jsonb('metadata').default('{}'),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  // Primary lookups
  assessOrgIdx: index('assess_org_idx').on(table.organizationId),
  assessYearIdx: index('assess_year_idx').on(table.academicYearId),
  assessTermIdx: index('assess_term_idx').on(table.academicTermId),
  assessClassSubIdx: index('assess_class_sub_idx').on(table.classSubjectId),
  assessGroupIdx: index('assess_group_idx').on(table.gradeGroupId),

  // Status lookups
  assessPublishedIdx: index('assess_published_idx').on(table.isPublished),
  assessLockedIdx: index('assess_locked_idx').on(table.isLocked),
  assessTypeIdx: index('assess_type_idx').on(table.type),

  // Date lookups
  assessDateIdx: index('assess_date_idx').on(table.assessmentDate),

  // Composite for common queries
  assessYearTermIdx: index('assess_year_term_idx').on(table.academicYearId, table.academicTermId),
  assessClassSubTypeIdx: index('assess_class_sub_type_idx').on(table.classSubjectId, table.type),
}));

// ==========================================
// GRADE ENTRIES - OPTIMIZED FOR MASS ENTRY
// ==========================================

// Strategy: Partitioned by academic_year_id for performance
// Each assessment item gets its own grade entries

export const gradeEntries = pgTable('grade_entries', {
  id: serial('id').primaryKey(),

  // Partition key (academic year for sharding)
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  // Core References
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  assessmentItemId: integer('assessment_item_id').references(() => assessmentItems.id, { onDelete: 'cascade' }).notNull(),
  enrollmentId: integer('enrollment_id').references(() => enrollments.id, { onDelete: 'cascade' }),

  // Grade Data
  score: numeric('score', { precision: 6, scale: 2 }),
  percentage: numeric('percentage', { precision: 6, scale: 2 }),
  grade: varchar('grade', { length: 10 }), // A, B, C...
  gradePoints: numeric('grade_points', { precision: 4, scale: 2 }),

  // Status
  isAbsent: boolean('is_absent').default(false).notNull(),
  isExcused: boolean('is_excused').default(false).notNull(),
  isIncomplete: boolean('is_incomplete').default(false).notNull(),
  isLate: boolean('is_late').default(false).notNull(),

  // Comments & Feedback
  comments: text('comments'),
  teacherFeedback: text('teacher_feedback'),
  studentReflection: text('student_reflection'),

  // Grading Audit
  gradedById: uuid('graded_by_id').references(() => users.id, { onDelete: 'set null' }),
  gradedAt: timestamp('graded_at').defaultNow(),

  // Modification Tracking
  modifiedById: uuid('modified_by_id').references(() => users.id, { onDelete: 'set null' }),
  modifiedAt: timestamp('modified_at'),
  modificationReason: text('modification_reason'),

  // Version Control
  version: integer('version').default(1).notNull(),
  previousVersionId: integer('previous_version_id'), // Self-reference for history

  // Batch Entry
  batchId: varchar('batch_id', { length: 100 }), // For bulk imports
  importSource: varchar('import_source', { length: 50 }), // manual, csv, api, template

  // Metadata
  metadata: jsonb('metadata').default('{}'),

  // Soft Delete
  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  // CRITICAL: Unique constraint for data integrity
  gradeUnique: unique('grade_entry_unique').on(table.studentId, table.assessmentItemId),

  // Primary lookups - HIGHLY OPTIMIZED
  gradeYearIdx: index('grade_year_idx').on(table.academicYearId),
  gradeStudentIdx: index('grade_student_idx').on(table.studentId),
  gradeAssessmentIdx: index('grade_assessment_idx').on(table.assessmentItemId),
  gradeEnrollmentIdx: index('grade_enrollment_idx').on(table.enrollmentId),

  // Grading workflow
  gradeGradedByIdx: index('grade_graded_by_idx').on(table.gradedById),
  gradeBatchIdx: index('grade_batch_idx').on(table.batchId).where(sql`${table.batchId} IS NOT NULL`),
  gradeVersionIdx: index('grade_version_idx').on(table.version),

  // Status lookups
  gradeAbsentIdx: index('grade_absent_idx').on(table.isAbsent).where(sql`${table.isAbsent} = true`),
  gradeDeletedIdx: index('grade_deleted_idx').on(table.deletedAt),

  // Composite for reporting
  gradeYearStudentIdx: index('grade_year_student_idx').on(table.academicYearId, table.studentId),
  gradeAssessStudentIdx: index('grade_assess_student_idx').on(table.assessmentItemId, table.studentId),
}));

// Grade Entry History (Audit Trail for Changes)
export const gradeEntryHistory = pgTable('grade_entry_history', {
  id: serial('id').primaryKey(),
  gradeEntryId: integer('grade_entry_id').notNull(),

  // Snapshot of data
  score: numeric('score', { precision: 6, scale: 2 }),
  percentage: numeric('percentage', { precision: 6, scale: 2 }),
  grade: varchar('grade', { length: 10 }),
  isAbsent: boolean('is_absent').default(false),
  isExcused: boolean('is_excused').default(false),
  comments: text('comments'),

  // Change metadata
  changedById: uuid('changed_by_id').references(() => users.id, { onDelete: 'set null' }),
  changedAt: timestamp('changed_at').defaultNow().notNull(),
  changeReason: text('change_reason'),
  changeType: varchar('change_type', { length: 50 }).notNull(), // create, update, delete, restore

  // Full snapshot
  snapshot: jsonb('snapshot').notNull(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
}, (table) => ({
  historyEntryIdx: index('history_entry_idx').on(table.gradeEntryId),
  historyChangedByIdx: index('history_changed_by_idx').on(table.changedById),
  historyDateIdx: index('history_date_idx').on(table.changedAt),
}));

// Student Subject Averages (Materialized/Cached for Performance)
export const studentSubjectAverages = pgTable('student_subject_averages', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  classSubjectId: integer('class_subject_id').references(() => classSubjects.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'cascade' }),

  // Calculated Averages
  overallAverage: numeric('overall_average', { precision: 6, scale: 2 }),
  overallPercentage: numeric('overall_percentage', { precision: 6, scale: 2 }),
  overallGrade: varchar('overall_grade', { length: 10 }),
  overallGradePoints: numeric('overall_grade_points', { precision: 4, scale: 2 }),

  // Breakdown by Grade Group
  groupBreakdown: jsonb('group_breakdown').default('{}'), // { "exams": 85.5, "quizzes": 78.0, "homework": 92.0 }

  // Rankings
  classRank: integer('class_rank'),
  sectionRank: integer('section_rank'),
  overallRank: integer('overall_rank'),

  // Statistics
  totalAssessments: integer('total_assessments').default(0),
  completedAssessments: integer('completed_assessments').default(0),
  missedAssessments: integer('missed_assessments').default(0),

  // Status
  isPublished: boolean('is_published').default(false).notNull(),
  publishedAt: timestamp('published_at'),

  // Recalculation tracking
  lastCalculatedAt: timestamp('last_calculated_at').defaultNow(),
  calculationMethod: varchar('calculation_method', { length: 50 }).default('auto'),

  metadata: jsonb('metadata').default('{}'),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  ssaUnique: unique('ssa_unique').on(table.studentId, table.classSubjectId, table.academicYearId, table.academicTermId),
  ssaStudentIdx: index('ssa_student_idx').on(table.studentId),
  ssaClassSubIdx: index('ssa_class_sub_idx').on(table.classSubjectId),
  ssaYearIdx: index('ssa_year_idx').on(table.academicYearId),
  ssaTermIdx: index('ssa_term_idx').on(table.academicTermId),
  ssaPublishedIdx: index('ssa_published_idx').on(table.isPublished),
}));

// Rubrics for Advanced Grading
export const rubrics = pgTable('rubrics', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  name: varchar('name', { length: 255 }).notNull(),
  description: text('description'),

  // Criteria
  criteria: jsonb('criteria').notNull(), // [{name: "Content", weight: 40, levels: [{score: 4, label: "Excellent"}...]}]

  totalPoints: numeric('total_points', { precision: 6, scale: 2 }).notNull(),

  isShared: boolean('is_shared').default(false).notNull(),
  isActive: boolean('is_active').default(true).notNull(),

  createdById: uuid('created_by_id').references(() => users.id, { onDelete: 'set null' }),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  rubricOrgIdx: index('rubric_org_idx').on(table.organizationId),
  rubricActiveIdx: index('rubric_active_idx').on(table.isActive),
}));


// ============================================================
// PART 7: 7 TIMETABLE
// ============================================================

// ==========================================
// SECTION 8: TIMETABLE & ATTENDANCE
// ==========================================

// Time Slots (Periods)
export const timeSlots = pgTable('time_slots', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  slotNumber: integer('slot_number').notNull(),
  name: varchar('name', { length: 50 }), // "Period 1", "First Period"

  startTime: time('start_time').notNull(),
  endTime: time('end_time').notNull(),
  durationMinutes: integer('duration_minutes').notNull(),

  // Break/Activity
  isBreak: boolean('is_break').default(false).notNull(),
  breakType: varchar('break_type', { length: 50 }), // short_break, lunch, prayer

  // Schedule
  applicableDays: jsonb('applicable_days').$type<number[]>(), // [1,2,3,4,5] for Sun-Thu

  isActive: boolean('is_active').default(true).notNull(),
  sortOrder: integer('sort_order').default(0),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  slotOrgIdx: index('slot_org_idx').on(table.organizationId),
  slotYearIdx: index('slot_year_idx').on(table.academicYearId),
  slotNumberIdx: index('slot_number_idx').on(table.academicYearId, table.slotNumber),
  slotTimeIdx: index('slot_time_idx').on(table.startTime, table.endTime),
}));

// Scheduled Lessons (Timetable)
export const scheduledLessons = pgTable('scheduled_lessons', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'cascade' }),

  // Class & Subject
  classId: integer('class_id').references(() => classes.id, { onDelete: 'cascade' }).notNull(),
  classSubjectId: integer('class_subject_id').references(() => classSubjects.id, { onDelete: 'cascade' }),
  subjectId: integer('subject_id').references(() => subjects.id, { onDelete: 'restrict' }).notNull(),

  // Teacher
  teacherId: uuid('teacher_id').references(() => users.id, { onDelete: 'restrict' }).notNull(),
  substituteTeacherId: uuid('substitute_teacher_id').references(() => users.id, { onDelete: 'set null' }),

  // Time & Location
  timeSlotId: integer('time_slot_id').references(() => timeSlots.id, { onDelete: 'restrict' }).notNull(),
  dayOfWeek: dayOfWeekEnum('day_of_week').notNull(),

  roomId: integer('room_id').references(() => classes.id, { onDelete: 'set null' }),
  roomName: varchar('room_name', { length: 100 }), // Override room name

  // Status
  status: lessonStatusEnum('status').default('scheduled').notNull(),
  isSubstituted: boolean('is_substituted').default(false).notNull(),
  substitutionReason: text('substitution_reason'),

  // Lesson Details
  topic: varchar('topic', { length: 255 }),
  objectives: text('objectives'),
  materials: jsonb('materials').default('[]'),
  homework: text('homework'),

  // Attendance Tracking
  attendanceTaken: boolean('attendance_taken').default(false).notNull(),
  attendanceTakenAt: timestamp('attendance_taken_at'),
  attendanceTakenById: uuid('attendance_taken_by_id').references(() => users.id, { onDelete: 'set null' }),

  // Notes
  notes: text('notes'),
  cancellationReason: text('cancellation_reason'),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  // Unique constraint: One lesson per class per slot per day per year
  lessonUnique: unique('lesson_unique').on(table.classId, table.timeSlotId, table.dayOfWeek, table.academicYearId),

  // Primary lookups
  lessonOrgIdx: index('lesson_org_idx').on(table.organizationId),
  lessonYearIdx: index('lesson_year_idx').on(table.academicYearId),
  lessonTermIdx: index('lesson_term_idx').on(table.academicTermId),
  lessonClassIdx: index('lesson_class_idx').on(table.classId),
  lessonSubjectIdx: index('lesson_subject_idx').on(table.subjectId),
  lessonTeacherIdx: index('lesson_teacher_idx').on(table.teacherId),
  lessonSlotIdx: index('lesson_slot_idx').on(table.timeSlotId),
  lessonDayIdx: index('lesson_day_idx').on(table.dayOfWeek),

  // Composite for teacher schedule
  lessonTeacherDayIdx: index('lesson_teacher_day_idx').on(table.teacherId, table.dayOfWeek, table.timeSlotId),
  lessonClassDayIdx: index('lesson_class_day_idx').on(table.classId, table.dayOfWeek, table.timeSlotId),

  // Status
  lessonStatusIdx: index('lesson_status_idx').on(table.status),
  lessonSubstitutedIdx: index('lesson_substituted_idx').on(table.isSubstituted),
}));

// Student Attendance Records
export const attendanceRecords = pgTable('attendance_records', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  scheduledLessonId: integer('scheduled_lesson_id').references(() => scheduledLessons.id, { onDelete: 'cascade' }).notNull(),
  enrollmentId: integer('enrollment_id').references(() => enrollments.id, { onDelete: 'cascade' }),

  date: date('date').notNull(),
  status: attendanceStatusEnum('status').default('absent').notNull(),

  // Timing
  arrivalTime: time('arrival_time'),
  departureTime: time('departure_time'),

  // Details
  isLate: boolean('is_late').default(false).notNull(),
  lateMinutes: integer('late_minutes').default(0),
  isEarlyDeparture: boolean('is_early_departure').default(false).notNull(),

  // Excuse
  excuseType: varchar('excuse_type', { length: 50 }), // medical, family, official, other
  excuseDocument: text('excuse_document'),
  excuseNotes: text('excuse_notes'),

  // Verification
  isVerified: boolean('is_verified').default(false).notNull(),
  verifiedById: uuid('verified_by_id').references(() => users.id, { onDelete: 'set null' }),
  verifiedAt: timestamp('verified_at'),

  // Marking
  markedById: uuid('marked_by_id').references(() => users.id, { onDelete: 'set null' }),
  markedAt: timestamp('marked_at').defaultNow(),
  markMethod: varchar('mark_method', { length: 50 }).default('manual'), // manual, biometric, qr, auto

  // Device/Location
  deviceId: varchar('device_id', { length: 255 }),
  location: jsonb('location').default('{}'),

  notes: text('notes'),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  // Unique: One record per student per lesson
  attUnique: unique('attendance_unique').on(table.studentId, table.scheduledLessonId),

  // Primary lookups
  attOrgIdx: index('att_org_idx').on(table.organizationId),
  attYearIdx: index('att_year_idx').on(table.academicYearId),
  attStudentIdx: index('att_student_idx').on(table.studentId),
  attLessonIdx: index('att_lesson_idx').on(table.scheduledLessonId),
  attEnrollmentIdx: index('att_enrollment_idx').on(table.enrollmentId),

  // Date & Status
  attDateIdx: index('att_date_idx').on(table.date),
  attStatusIdx: index('att_status_idx').on(table.status),
  attLateIdx: index('att_late_idx').on(table.isLate).where(sql`${table.isLate} = true`),

  // Composite for reports
  attStudentDateIdx: index('att_student_date_idx').on(table.studentId, table.date),
  attLessonDateIdx: index('att_lesson_date_idx').on(table.scheduledLessonId, table.date),
  attStudentYearIdx: index('att_student_year_idx').on(table.studentId, table.academicYearId),
  attStudentStatusIdx: index('att_student_status_idx').on(table.studentId, table.status),
}));

// Attendance Summary (Cached/Materialized for Reports)
export const attendanceSummaries = pgTable('attendance_summaries', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  enrollmentId: integer('enrollment_id').references(() => enrollments.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'cascade' }),

  // Period
  periodType: varchar('period_type', { length: 50 }).notNull(), // monthly, termly, yearly
  periodStart: date('period_start').notNull(),
  periodEnd: date('period_end').notNull(),

  // Counts
  totalDays: integer('total_days').default(0),
  presentDays: integer('present_days').default(0),
  absentDays: integer('absent_days').default(0),
  lateDays: integer('late_days').default(0),
  excusedDays: integer('excused_days').default(0),
  sickDays: integer('sick_days').default(0),

  // Percentage
  attendanceRate: numeric('attendance_rate', { precision: 5, scale: 2 }),

  // Alerts
  hasAlert: boolean('has_alert').default(false).notNull(),
  alertType: varchar('alert_type', { length: 50 }), // low_attendance, consecutive_absence
  alertSentAt: timestamp('alert_sent_at'),

  lastCalculatedAt: timestamp('last_calculated_at').defaultNow(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  attSumUnique: unique('att_sum_unique').on(table.studentId, table.enrollmentId, table.periodType, table.periodStart),
  attSumStudentIdx: index('att_sum_student_idx').on(table.studentId),
  attSumEnrollmentIdx: index('att_sum_enrollment_idx').on(table.enrollmentId),
  attSumYearIdx: index('att_sum_year_idx').on(table.academicYearId),
  attSumAlertIdx: index('att_sum_alert_idx').on(table.hasAlert),
}));

// Behavior/Discipline Records
export const behaviorRecords = pgTable('behavior_records', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  enrollmentId: integer('enrollment_id').references(() => enrollments.id, { onDelete: 'cascade' }),

  // Incident Details
  type: behaviorTypeEnum('type').notNull(),
  severity: behaviorSeverityEnum('severity').default('minor').notNull(),
  category: varchar('category', { length: 100 }), // bullying, cheating, disrespect, excellence, helpfulness

  title: varchar('title', { length: 255 }).notNull(),
  description: text('description').notNull(),

  // Location & Time
  incidentDate: date('incident_date').notNull(),
  incidentTime: time('incident_time'),
  location: varchar('location', { length: 255 }),

  // Points System
  points: integer('points').default(0), // Positive or negative

  // Actions Taken
  actionTaken: varchar('action_taken', { length: 255 }), // detention, suspension, warning, reward
  actionDetails: text('action_details'),
  actionDate: date('action_date'),
  actionCompleted: boolean('action_completed').default(false),

  // Involved Parties
  reportedById: uuid('reported_by_id').references(() => users.id, { onDelete: 'set null' }),
  reviewedById: uuid('reviewed_by_id').references(() => users.id, { onDelete: 'set null' }),
  witnesses: jsonb('witnesses').default('[]'),

  // Parent Notification
  parentNotified: boolean('parent_notified').default(false).notNull(),
  parentNotifiedAt: timestamp('parent_notified_at'),
  parentResponse: text('parent_response'),

  // Attachments
  evidenceUrls: jsonb('evidence_urls').default('[]'),

  // Status
  status: varchar('status', { length: 50 }).default('active').notNull(), // active, resolved, appealed, escalated
  resolutionNotes: text('resolution_notes'),
  resolvedAt: timestamp('resolved_at'),
  resolvedById: uuid('resolved_by_id').references(() => users.id, { onDelete: 'set null' }),

  isPrivate: boolean('is_private').default(false).notNull(),

  metadata: jsonb('metadata').default('{}'),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  behaviorOrgIdx: index('behavior_org_idx').on(table.organizationId),
  behaviorYearIdx: index('behavior_year_idx').on(table.academicYearId),
  behaviorStudentIdx: index('behavior_student_idx').on(table.studentId),
  behaviorTypeIdx: index('behavior_type_idx').on(table.type),
  behaviorSeverityIdx: index('behavior_severity_idx').on(table.severity),
  behaviorDateIdx: index('behavior_date_idx').on(table.incidentDate),
  behaviorStatusIdx: index('behavior_status_idx').on(table.status),
  behaviorReporterIdx: index('behavior_reporter_idx').on(table.reportedById),
}));


// ============================================================
// PART 8: 8 FINANCE
// ============================================================

// ==========================================
// SECTION 9: FINANCIAL SYSTEM
// ==========================================

// Fee Categories
export const feeCategories = pgTable('fee_categories', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  name: varchar('name', { length: 255 }).notNull(),
  nameAr: varchar('name_ar', { length: 255 }),
  code: varchar('code', { length: 50 }),

  description: text('description'),
  color: varchar('color', { length: 20 }).default('#6366f1'),

  // Accounting
  incomeAccountId: integer('income_account_id'), // Link to chart of accounts
  receivableAccountId: integer('receivable_account_id'),

  isActive: boolean('is_active').default(true).notNull(),
  sortOrder: integer('sort_order').default(0),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  feeCatOrgIdx: index('fee_cat_org_idx').on(table.organizationId),
  feeCatCodeIdx: unique('fee_cat_code_idx').on(table.organizationId, table.code),
}));

// Fee Types (Tuition, Bus, Uniform, etc.)
export const feeTypes = pgTable('fee_types', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  categoryId: integer('category_id').references(() => feeCategories.id, { onDelete: 'set null' }),

  name: varchar('name', { length: 255 }).notNull(),
  nameAr: varchar('name_ar', { length: 255 }),
  code: varchar('code', { length: 50 }),
  description: text('description'),

  // Billing
  frequency: feeFrequencyEnum('frequency').default('yearly').notNull(),
  amount: numeric('amount', { precision: 12, scale: 2 }).notNull(),
  currency: varchar('currency', { length: 3 }).default('YER').notNull(),

  // Applicability
  applicableTo: jsonb('applicable_to').default('[]'), // ["all", "new", "returning", "specific_class"]
  applicableClasses: jsonb('applicable_classes').default('[]'), // [class_id, class_id]

  // Discounts
  isDiscountable: boolean('is_discountable').default(true).notNull(),
  maxDiscountPercent: numeric('max_discount_percent', { precision: 5, scale: 2 }).default('100.00'),

  // Penalty
  lateFeePercent: numeric('late_fee_percent', { precision: 5, scale: 2 }).default('0'),
  lateFeeAmount: numeric('late_fee_amount', { precision: 12, scale: 2 }),
  gracePeriodDays: integer('grace_period_days').default(0),

  // Dates
  effectiveFrom: date('effective_from'),
  effectiveUntil: date('effective_until'),

  isMandatory: boolean('is_mandatory').default(true).notNull(),
  isActive: boolean('is_active').default(true).notNull(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  feeTypeOrgIdx: index('fee_type_org_idx').on(table.organizationId),
  feeTypeCatIdx: index('fee_type_cat_idx').on(table.categoryId),
  feeTypeCodeIdx: unique('fee_type_code_idx').on(table.organizationId, table.code),
  feeTypeActiveIdx: index('fee_type_active_idx').on(table.isActive),
}));

// Student Fees (Generated per student per year/term)
export const studentFees = pgTable('student_fees', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'set null' }),

  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  enrollmentId: integer('enrollment_id').references(() => enrollments.id, { onDelete: 'cascade' }),
  feeTypeId: integer('fee_type_id').references(() => feeTypes.id, { onDelete: 'restrict' }).notNull(),

  // Amounts
  originalAmount: numeric('original_amount', { precision: 12, scale: 2 }).notNull(),
  discountAmount: numeric('discount_amount', { precision: 12, scale: 2 }).default('0'),
  discountReason: text('discount_reason'),
  discountApprovedById: uuid('discount_approved_by_id').references(() => users.id, { onDelete: 'set null' }),

  lateFeeAmount: numeric('late_fee_amount', { precision: 12, scale: 2 }).default('0'),

  totalAmount: numeric('total_amount', { precision: 12, scale: 2 }).notNull(),
  paidAmount: numeric('paid_amount', { precision: 12, scale: 2 }).default('0'),
  balance: numeric('balance', { precision: 12, scale: 2 }).notNull(),

  // Status
  status: feeStatusEnum('status').default('pending').notNull(),

  // Due Date
  dueDate: date('due_date').notNull(),
  isOverdue: boolean('is_overdue').default(false).notNull(),
  overdueSince: date('overdue_since'),

  // Installments
  isInstallment: boolean('is_installment').default(false).notNull(),
  parentFeeId: integer('parent_fee_id'), // Self-reference for installments
  installmentNumber: integer('installment_number'),
  totalInstallments: integer('total_installments'),

  // Notifications
  reminderSent: boolean('reminder_sent').default(false),
  reminderSentAt: timestamp('reminder_sent_at'),
  finalNoticeSent: boolean('final_notice_sent').default(false),

  // Locking
  isLocked: boolean('is_locked').default(false).notNull(),
  lockedAt: timestamp('locked_at'),
  lockedById: uuid('locked_by_id').references(() => users.id, { onDelete: 'set null' }),

  notes: text('notes'),
  metadata: jsonb('metadata').default('{}'),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  // Primary lookups
  sfOrgIdx: index('sf_org_idx').on(table.organizationId),
  sfYearIdx: index('sf_year_idx').on(table.academicYearId),
  sfStudentIdx: index('sf_student_idx').on(table.studentId),
  sfEnrollmentIdx: index('sf_enrollment_idx').on(table.enrollmentId),
  sfFeeTypeIdx: index('sf_fee_type_idx').on(table.feeTypeId),

  // Status & Financial
  sfStatusIdx: index('sf_status_idx').on(table.status),
  sfDueDateIdx: index('sf_due_date_idx').on(table.dueDate),
  sfOverdueIdx: index('sf_overdue_idx').on(table.isOverdue).where(sql`${table.isOverdue} = true`),
  sfLockedIdx: index('sf_locked_idx').on(table.isLocked),

  // Composite
  sfStudentYearIdx: index('sf_student_year_idx').on(table.studentId, table.academicYearId),
  sfStudentStatusIdx: index('sf_student_status_idx').on(table.studentId, table.status),
}));

// Fee Payments
export const feePayments = pgTable('fee_payments', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  studentFeeId: integer('student_fee_id').references(() => studentFees.id, { onDelete: 'restrict' }).notNull(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),

  // Payment Details
  amount: numeric('amount', { precision: 12, scale: 2 }).notNull(),
  currency: varchar('currency', { length: 3 }).default('YER').notNull(),
  exchangeRate: numeric('exchange_rate', { precision: 10, scale: 6 }).default('1'),

  method: paymentMethodEnum('method').notNull(),

  // Reference
  referenceNumber: varchar('reference_number', { length: 100 }),
  transactionId: varchar('transaction_id', { length: 255 }),
  receiptNumber: varchar('receipt_number', { length: 100 }),

  // Bank/Treasury
  treasuryId: integer('treasury_id').references(() => treasuries.id, { onDelete: 'set null' }),
  bankAccountId: integer('bank_account_id'),

  // Status
  status: paymentStatusEnum('status').default('pending').notNull(),

  // Dates
  paymentDate: date('payment_date').notNull(),
  clearedDate: date('cleared_date'),

  // Payer Info
  payerName: varchar('payer_name', { length: 255 }),
  payerPhone: varchar('payer_phone', { length: 50 }),
  payerRelation: varchar('payer_relation', { length: 50 }),

  // Refund
  isRefund: boolean('is_refund').default(false).notNull(),
  refundAmount: numeric('refund_amount', { precision: 12, scale: 2 }),
  refundReason: text('refund_reason'),
  refundApprovedById: uuid('refund_approved_by_id').references(() => users.id, { onDelete: 'set null' }),

  // Verification
  isVerified: boolean('is_verified').default(false).notNull(),
  verifiedById: uuid('verified_by_id').references(() => users.id, { onDelete: 'set null' }),
  verifiedAt: timestamp('verified_at'),

  // Attachments
  receiptUrl: text('receipt_url'),
  proofUrl: text('proof_url'),

  notes: text('notes'),
  metadata: jsonb('metadata').default('{}'),

  // Audit
  createdById: uuid('created_by_id').references(() => users.id, { onDelete: 'set null' }),
  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  payOrgIdx: index('pay_org_idx').on(table.organizationId),
  payStudentFeeIdx: index('pay_student_fee_idx').on(table.studentFeeId),
  payStudentIdx: index('pay_student_idx').on(table.studentId),
  payStatusIdx: index('pay_status_idx').on(table.status),
  payMethodIdx: index('pay_method_idx').on(table.method),
  payDateIdx: index('pay_date_idx').on(table.paymentDate),
  payReceiptIdx: index('pay_receipt_idx').on(table.receiptNumber),
  payRefIdx: index('pay_ref_idx').on(table.referenceNumber),
  payVerifiedIdx: index('pay_verified_idx').on(table.isVerified),
}));

// Fee Discounts & Scholarships
export const feeDiscounts = pgTable('fee_discounts', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  name: varchar('name', { length: 255 }).notNull(),
  nameAr: varchar('name_ar', { length: 255 }),
  code: varchar('code', { length: 50 }),
  description: text('description'),

  // Discount Type
  type: varchar('type', { length: 50 }).notNull(), // percentage, fixed_amount, sibling, staff, academic, need_based
  value: numeric('value', { precision: 12, scale: 2 }).notNull(), // 20% or 5000 YER

  // Applicability
  applicableFeeTypes: jsonb('applicable_fee_types').default('[]'), // [fee_type_id]
  applicableClasses: jsonb('applicable_classes').default('[]'),

  // Limits
  maxDiscountAmount: numeric('max_discount_amount', { precision: 12, scale: 2 }),
  maxRecipients: integer('max_recipients'),

  // Academic Requirements
  minGpa: numeric('min_gpa', { precision: 4, scale: 2 }),
  maxGpa: numeric('max_gpa', { precision: 4, scale: 2 }),
  minAttendanceRate: numeric('min_attendance_rate', { precision: 5, scale: 2 }),

  // Validity
  validFrom: date('valid_from'),
  validUntil: date('valid_until'),

  isActive: boolean('is_active').default(true).notNull(),
  isAutoApply: boolean('is_auto_apply').default(false).notNull(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  discountOrgIdx: index('discount_org_idx').on(table.organizationId),
  discountCodeIdx: unique('discount_code_idx').on(table.organizationId, table.code),
  discountTypeIdx: index('discount_type_idx').on(table.type),
  discountActiveIdx: index('discount_active_idx').on(table.isActive),
}));

// Student Discount Applications
export const studentDiscounts = pgTable('student_discounts', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  discountId: integer('discount_id').references(() => feeDiscounts.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),

  // Application
  appliedAmount: numeric('applied_amount', { precision: 12, scale: 2 }).notNull(),
  appliedToFeeId: integer('applied_to_fee_id').references(() => studentFees.id, { onDelete: 'set null' }),

  // Approval
  status: varchar('status', { length: 50 }).default('pending').notNull(), // pending, approved, rejected, revoked
  appliedById: uuid('applied_by_id').references(() => users.id, { onDelete: 'set null' }),
  appliedAt: timestamp('applied_at').defaultNow(),
  approvedById: uuid('approved_by_id').references(() => users.id, { onDelete: 'set null' }),
  approvedAt: timestamp('approved_at'),
  approvalNotes: text('approval_notes'),

  reason: text('reason'),
  evidenceUrls: jsonb('evidence_urls').default('[]'),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  studDiscUnique: unique('stud_disc_unique').on(table.studentId, table.discountId, table.academicYearId),
  studDiscStudentIdx: index('stud_disc_student_idx').on(table.studentId),
  studDiscDiscountIdx: index('stud_disc_discount_idx').on(table.discountId),
  studDiscStatusIdx: index('stud_disc_status_idx').on(table.status),
}));

// Treasuries & Bank Accounts
export const treasuries = pgTable('treasuries', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  schoolId: integer('school_id').references(() => schools.id, { onDelete: 'set null' }),

  name: varchar('name', { length: 255 }).notNull(),
  nameAr: varchar('name_ar', { length: 255 }),
  code: varchar('code', { length: 50 }),

  type: treasuryTypeEnum('type').default('cash').notNull(),

  // Bank Details (if applicable)
  bankName: varchar('bank_name', { length: 255 }),
  bankBranch: varchar('bank_branch', { length: 255 }),
  accountNumber: varchar('account_number', { length: 100 }),
  accountName: varchar('account_name', { length: 255 }),
  iban: varchar('iban', { length: 50 }),
  swiftCode: varchar('swift_code', { length: 20 }),

  // Balance
  openingBalance: numeric('opening_balance', { precision: 15, scale: 2 }).default('0'),
  currentBalance: numeric('current_balance', { precision: 15, scale: 2 }).default('0'),
  currency: varchar('currency', { length: 3 }).default('YER'),

  // Permissions
  authorizedUsers: jsonb('authorized_users').default('[]'), // [user_id]

  isDefault: boolean('is_default').default(false).notNull(),
  isActive: boolean('is_active').default(true).notNull(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  treasuryOrgIdx: index('treasury_org_idx').on(table.organizationId),
  treasurySchoolIdx: index('treasury_school_idx').on(table.schoolId),
  treasuryCodeIdx: unique('treasury_code_idx').on(table.organizationId, table.code),
  treasuryTypeIdx: index('treasury_type_idx').on(table.type),
}));

// Treasury Transactions (Journal)
export const treasuryTransactions = pgTable('treasury_transactions', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  treasuryId: integer('treasury_id').references(() => treasuries.id, { onDelete: 'restrict' }).notNull(),

  // Transaction Details
  type: varchar('type', { length: 50 }).notNull(), // income, expense, transfer
  category: varchar('category', { length: 100 }).notNull(), // fee, salary, utility, transfer

  amount: numeric('amount', { precision: 15, scale: 2 }).notNull(),
  currency: varchar('currency', { length: 3 }).default('YER'),
  exchangeRate: numeric('exchange_rate', { precision: 10, scale: 6 }).default('1'),

  // References
  referenceType: varchar('reference_type', { length: 50 }), // fee_payment, salary, expense
  referenceId: integer('reference_id'),

  // Accounting
  debitAccountId: integer('debit_account_id'),
  creditAccountId: integer('credit_account_id'),

  // Description
  description: text('description').notNull(),
  notes: text('notes'),

  // Attachments
  attachmentUrl: text('attachment_url'),

  // Audit
  createdById: uuid('created_by_id').references(() => users.id, { onDelete: 'set null' }),
  approvedById: uuid('approved_by_id').references(() => users.id, { onDelete: 'set null' }),
  approvedAt: timestamp('approved_at'),

  transactionDate: date('transaction_date').notNull(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  transOrgIdx: index('trans_org_idx').on(table.organizationId),
  transTreasuryIdx: index('trans_treasury_idx').on(table.treasuryId),
  transTypeIdx: index('trans_type_idx').on(table.type),
  transCategoryIdx: index('trans_category_idx').on(table.category),
  transDateIdx: index('trans_date_idx').on(table.transactionDate),
  transRefIdx: index('trans_ref_idx').on(table.referenceType, table.referenceId),
  transCreatedByIdx: index('trans_created_by_idx').on(table.createdById),
}));


// ============================================================
// PART 9: 9 LOCKS PROMOTIONS
// ============================================================

// ==========================================
// SECTION 10: PERIOD LOCKS & ACADEMIC CONTROLS
// ==========================================

// Period Locks (Monthly, Mid-term, Final, etc.)
export const periodLocks = pgTable('period_locks', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'cascade' }),

  // Lock Details
  type: lockTypeEnum('type').notNull(),
  name: varchar('name', { length: 255 }).notNull(),
  description: text('description'),

  // Period
  startDate: date('start_date').notNull(),
  endDate: date('end_date').notNull(),

  // Status
  status: lockStatusEnum('status').default('locked').notNull(),

  // What is locked
  lockGrades: boolean('lock_grades').default(true).notNull(),
  lockAttendance: boolean('lock_attendance').default(false).notNull(),
  lockBehavior: boolean('lock_behavior').default(false).notNull(),
  lockFees: boolean('lock_fees').default(false).notNull(),
  lockEnrollments: boolean('lock_enrollments').default(false).notNull(),
  lockTimetable: boolean('lock_timetable').default(false).notNull(),

  // Scope
  scopeType: varchar('scope_type', { length: 50 }).default('organization').notNull(), // organization, school, class, subject
  scopeId: integer('scope_id'), // ID based on scopeType

  // Override
  isOverridden: boolean('is_overridden').default(false).notNull(),
  overriddenById: uuid('overridden_by_id').references(() => users.id, { onDelete: 'set null' }),
  overriddenAt: timestamp('overridden_at'),
  overrideReason: text('override_reason'),
  overrideExpiresAt: timestamp('override_expires_at'),

  // Notification
  notifyBeforeDays: integer('notify_before_days').default(7),
  notificationSent: boolean('notification_sent').default(false),

  createdById: uuid('created_by_id').references(() => users.id, { onDelete: 'set null' }),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  lockOrgIdx: index('lock_org_idx').on(table.organizationId),
  lockYearIdx: index('lock_year_idx').on(table.academicYearId),
  lockTermIdx: index('lock_term_idx').on(table.academicTermId),
  lockTypeIdx: index('lock_type_idx').on(table.type),
  lockStatusIdx: index('lock_status_idx').on(table.status),
  lockDateIdx: index('lock_date_idx').on(table.startDate, table.endDate),
  lockScopeIdx: index('lock_scope_idx').on(table.scopeType, table.scopeId),
}));

// Lock Exceptions (For specific students/teachers)
export const lockExceptions = pgTable('lock_exceptions', {
  id: serial('id').primaryKey(),
  periodLockId: integer('period_lock_id').references(() => periodLocks.id, { onDelete: 'cascade' }).notNull(),

  // Exception For
  exceptionType: varchar('exception_type', { length: 50 }).notNull(), // student, teacher, class, assessment
  exceptionId: varchar('exception_id', { length: 100 }).notNull(), // UUID or ID as string

  // What is allowed
  allowGradeEntry: boolean('allow_grade_entry').default(false).notNull(),
  allowAttendance: boolean('allow_attendance').default(false).notNull(),
  allowBehavior: boolean('allow_behavior').default(false).notNull(),

  // Validity
  validFrom: timestamp('valid_from').defaultNow(),
  validUntil: timestamp('valid_until'),

  reason: text('reason').notNull(),

  grantedById: uuid('granted_by_id').references(() => users.id, { onDelete: 'set null' }),
  grantedAt: timestamp('granted_at').defaultNow(),

  isActive: boolean('is_active').default(true).notNull(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
}, (table) => ({
  excLockIdx: index('exc_lock_idx').on(table.periodLockId),
  excTypeIdx: index('exc_type_idx').on(table.exceptionType, table.exceptionId),
  excActiveIdx: index('exc_active_idx').on(table.isActive),
}));

// ==========================================
// SECTION 11: STUDENT PROMOTIONS
// ==========================================

export const studentPromotions = pgTable('student_promotions', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  enrollmentId: integer('enrollment_id').references(() => enrollments.id, { onDelete: 'cascade' }),

  // From
  fromClassId: integer('from_class_id').references(() => classes.id, { onDelete: 'restrict' }).notNull(),
  fromAcademicYearId: integer('from_academic_year_id').references(() => academicYears.id, { onDelete: 'restrict' }).notNull(),

  // To
  toClassId: integer('to_class_id').references(() => classes.id, { onDelete: 'restrict' }),
  toAcademicYearId: integer('to_academic_year_id').references(() => academicYears.id, { onDelete: 'restrict' }).notNull(),

  // Academic Results
  finalScore: numeric('final_score', { precision: 6, scale: 2 }),
  finalGpa: numeric('final_gpa', { precision: 4, scale: 2 }),
  finalGrade: varchar('final_grade', { length: 10 }),
  classRank: integer('class_rank'),
  overallRank: integer('overall_rank'),

  // Subject-wise Results
  subjectResults: jsonb('subject_results').default('[]'), // [{subjectId, score, grade, status}]

  // Attendance
  attendanceRate: numeric('attendance_rate', { precision: 5, scale: 2 }),
  behaviorScore: integer('behavior_score'),

  // Financial
  feesPaid: boolean('fees_paid').default(true).notNull(),
  feesOutstanding: numeric('fees_outstanding', { precision: 12, scale: 2 }).default('0'),

  // Decision
  status: promotionStatusEnum('status').notNull(),
  decision: promotionDecisionEnum('decision').default('auto').notNull(),

  // Manual Override
  decisionById: uuid('decision_by_id').references(() => users.id, { onDelete: 'set null' }),
  decisionAt: timestamp('decision_at'),
  decisionNotes: text('decision_notes'),

  // Committee (if applicable)
  committeeReviewed: boolean('committee_reviewed').default(false),
  committeeNotes: text('committee_notes'),
  committeeMembers: jsonb('committee_members').default('[]'),

  // Parent Notification
  parentNotified: boolean('parent_notified').default(false).notNull(),
  parentNotifiedAt: timestamp('parent_notified_at'),
  parentAcknowledged: boolean('parent_acknowledged').default(false),

  // Documents
  certificateUrl: text('certificate_url'),
  transcriptUrl: text('transcript_url'),

  notes: text('notes'),
  metadata: jsonb('metadata').default('{}'),

  promotedAt: timestamp('promoted_at').defaultNow().notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  promoOrgIdx: index('promo_org_idx').on(table.organizationId),
  promoStudentIdx: index('promo_student_idx').on(table.studentId),
  promoEnrollmentIdx: index('promo_enrollment_idx').on(table.enrollmentId),
  promoFromYearIdx: index('promo_from_year_idx').on(table.fromAcademicYearId),
  promoToYearIdx: index('promo_to_year_idx').on(table.toAcademicYearId),
  promoStatusIdx: index('promo_status_idx').on(table.status),
  promoDecisionIdx: index('promo_decision_idx').on(table.decision),
  promoUnique: unique('promo_unique').on(table.studentId, table.fromAcademicYearId),
}));

// ==========================================
// SECTION 12: NOTIFICATIONS SYSTEM
// ==========================================

export const notifications = pgTable('notifications', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  // Sender
  senderId: uuid('sender_id').references(() => users.id, { onDelete: 'set null' }),
  senderType: varchar('sender_type', { length: 50 }).default('system'), // system, user, automation

  // Recipient
  recipientId: uuid('recipient_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  recipientType: varchar('recipient_type', { length: 50 }).default('user'), // user, role, group, all

  // Content
  type: notificationTypeEnum('type').notNull(),
  priority: notificationPriorityEnum('priority').default('normal').notNull(),

  title: varchar('title', { length: 255 }).notNull(),
  titleAr: varchar('title_ar', { length: 255 }),
  message: text('message').notNull(),
  messageAr: text('message_ar'),

  // Rich Content
  actionUrl: text('action_url'),
  actionText: varchar('action_text', { length: 100 }),
  imageUrl: text('image_url'),
  icon: varchar('icon', { length: 50 }),
  color: varchar('color', { length: 20 }),

  // Related Entity
  relatedEntityType: varchar('related_entity_type', { length: 50 }),
  relatedEntityId: varchar('related_entity_id', { length: 100 }),

  // Context
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'set null' }),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'set null' }),
  subjectId: integer('subject_id').references(() => subjects.id, { onDelete: 'set null' }),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'set null' }),

  // Delivery
  channels: jsonb('channels').default('["in_app"]'), // ["in_app", "email", "sms", "push"]

  // In-App Status
  isRead: boolean('is_read').default(false).notNull(),
  readAt: timestamp('read_at'),

  isArchived: boolean('is_archived').default(false).notNull(),
  archivedAt: timestamp('archived_at'),

  isHighPriority: boolean('is_high_priority').default(false).notNull(),

  // Delivery Tracking
  deliveredAt: timestamp('delivered_at'),
  deliveredChannels: jsonb('delivered_channels').default('[]'),
  failedChannels: jsonb('failed_channels').default('[]'),

  // Expiry
  expiresAt: timestamp('expires_at'),

  metadata: jsonb('metadata').default('{}'),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  // Primary lookups
  notifOrgIdx: index('notif_org_idx').on(table.organizationId),
  notifRecipientIdx: index('notif_recipient_idx').on(table.recipientId),
  notifSenderIdx: index('notif_sender_idx').on(table.senderId),
  notifTypeIdx: index('notif_type_idx').on(table.type),
  notifPriorityIdx: index('notif_priority_idx').on(table.priority),

  // Status lookups
  notifReadIdx: index('notif_read_idx').on(table.recipientId, table.isRead),
  notifUnreadIdx: index('notif_unread_idx').on(table.recipientId).where(sql`${table.isRead} = false`),
  notifArchivedIdx: index('notif_archived_idx').on(table.recipientId, table.isArchived),

  // Entity lookups
  notifEntityIdx: index('notif_entity_idx').on(table.relatedEntityType, table.relatedEntityId),
  notifStudentIdx: index('notif_student_idx').on(table.studentId),
  notifClassIdx: index('notif_class_idx').on(table.classId),

  // Time-based
  notifCreatedIdx: index('notif_created_idx').on(table.recipientId, table.createdAt.desc()),
  notifDeliveredIdx: index('notif_delivered_idx').on(table.deliveredAt),
}));

// Notification Preferences (Per User)
export const notificationPreferences = pgTable('notification_preferences', {
  id: serial('id').primaryKey(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),

  // Notification Type Settings
  notificationType: notificationTypeEnum('notification_type').notNull(),

  // Channel Preferences
  inAppEnabled: boolean('in_app_enabled').default(true).notNull(),
  emailEnabled: boolean('email_enabled').default(true).notNull(),
  smsEnabled: boolean('sms_enabled').default(false).notNull(),
  pushEnabled: boolean('push_enabled').default(true).notNull(),
  whatsappEnabled: boolean('whatsapp_enabled').default(false).notNull(),

  // Quiet Hours
  quietHoursEnabled: boolean('quiet_hours_enabled').default(false).notNull(),
  quietHoursStart: time('quiet_hours_start'),
  quietHoursEnd: time('quiet_hours_end'),

  // Frequency
  digestMode: varchar('digest_mode', { length: 50 }).default('immediate'), // immediate, daily, weekly

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  prefUnique: unique('notif_pref_unique').on(table.userId, table.notificationType),
  prefUserIdx: index('pref_user_idx').on(table.userId),
}));

// Notification Templates
export const notificationTemplates = pgTable('notification_templates', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  name: varchar('name', { length: 255 }).notNull(),
  code: varchar('code', { length: 100 }).notNull(),
  description: text('description'),

  type: notificationTypeEnum('type').notNull(),

  // Content
  titleTemplate: varchar('title_template', { length: 255 }).notNull(),
  titleTemplateAr: varchar('title_template_ar', { length: 255 }),
  messageTemplate: text('message_template').notNull(),
  messageTemplateAr: text('message_template_ar'),

  // Variables
  variables: jsonb('variables').default('[]'), // ["student_name", "grade", "subject"]

  // Channels
  defaultChannels: jsonb('default_channels').default('["in_app"]'),

  isActive: boolean('is_active').default(true).notNull(),
  isSystem: boolean('is_system').default(false).notNull(),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  templateOrgIdx: index('template_org_idx').on(table.organizationId),
  templateCodeIdx: unique('template_code_idx').on(table.organizationId, table.code),
  templateTypeIdx: index('template_type_idx').on(table.type),
}));


// ============================================================
// PART 10: 10 DOCUMENTS AUDIT
// ============================================================

// ==========================================
// SECTION 13: DOCUMENTS & FILE MANAGEMENT
// ==========================================

export const documents = pgTable('documents', {
  id: uuid('id').defaultRandom().primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  // Ownership
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  uploadedById: uuid('uploaded_by_id').references(() => users.id, { onDelete: 'set null' }),

  // File Details
  name: varchar('name', { length: 255 }).notNull(),
  title: varchar('title', { length: 255 }),
  description: text('description'),

  type: documentTypeEnum('type').default('file').notNull(),
  visibility: documentVisibilityEnum('visibility').default('private').notNull(),

  size: integer('size'),
  mimeType: varchar('mime_type', { length: 100 }),
  extension: varchar('extension', { length: 20 }),

  // Storage
  storageProvider: varchar('storage_provider', { length: 50 }).default('s3'), // s3, azure, gcs, local
  bucket: varchar('bucket', { length: 100 }),
  key: text('key').notNull(),
  url: text('url'),
  thumbnailUrl: text('thumbnail_url'),

  // Hierarchy
  parentFolderId: uuid('parent_folder_id').references((): AnyPgColumn => documents.id, { onDelete: 'cascade' }),
  path: text('path'), // Materialized path: /folder1/folder2/
  depth: integer('depth').default(0),

  // Sharing
  sharedWith: jsonb('shared_with').default('[]'), // [{userId, permission}]
  shareLink: text('share_link'),
  shareLinkExpiresAt: timestamp('share_link_expires_at'),

  // Versioning
  version: integer('version').default(1),
  previousVersionId: uuid('previous_version_id'),
  isLatestVersion: boolean('is_latest_version').default(true).notNull(),

  // Processing
  processingStatus: varchar('processing_status', { length: 20 }).default('completed'),
  ocrText: text('ocr_text'),
  extractedData: jsonb('extracted_data').default('{}'),

  // Flags
  isPinned: boolean('is_pinned').default(false).notNull(),
  isStarred: boolean('is_starred').default(false).notNull(),
  isTemplate: boolean('is_template').default(false).notNull(),

  // Tags
  tags: jsonb('tags').$type<string[]>(),

  // Metadata
  metadata: jsonb('metadata').$type<Record<string, any>>().default('{}'),

  deletedAt: timestamp('deleted_at'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  docOrgIdx: index('doc_org_idx').on(table.organizationId),
  docUserIdx: index('doc_user_idx').on(table.userId),
  docParentIdx: index('doc_parent_idx').on(table.parentFolderId),
  docTypeIdx: index('doc_type_idx').on(table.type),
  docVisibilityIdx: index('doc_visibility_idx').on(table.visibility),
  docPathIdx: index('doc_path_idx').on(table.path),
  docPinnedIdx: index('doc_pinned_idx').on(table.userId, table.isPinned).where(sql`${table.isPinned} = true`),
  docCreatedIdx: index('doc_created_idx').on(table.userId, table.createdAt.desc()),
  docLatestIdx: index('doc_latest_idx').on(table.isLatestVersion),
}));

// Short Links
export const shortLinks = pgTable('short_links', {
  id: uuid('id').defaultRandom().primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  code: varchar('code', { length: 255 }).unique().notNull(),
  url: text('url').notNull(),

  // Tracking
  title: varchar('title', { length: 255 }),
  description: text('description'),

  // Access Control
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  maxClicks: integer('max_clicks'),
  currentClicks: integer('current_clicks').default(0),

  // Security
  password: text('password'),
  isActive: boolean('is_active').default(true).notNull(),

  // Expiry
  expiresAt: timestamp('expires_at'),

  // Analytics
  lastAccessedAt: timestamp('last_accessed_at'),
  lastAccessedIp: varchar('last_accessed_ip', { length: 45 }),
  accessLog: jsonb('access_log').default('[]'), // [{ip, time, userAgent}]

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  linkOrgIdx: index('link_org_idx').on(table.organizationId),
  linkCodeIdx: index('link_code_idx').on(table.code),
  linkUserIdx: index('link_user_idx').on(table.userId),
  linkActiveIdx: index('link_active_idx').on(table.isActive),
}));

// ==========================================
// SECTION 14: CERTIFICATES & TEMPLATES
// ==========================================

export const certificateTemplates = pgTable('certificate_templates', {
  id: uuid('id').defaultRandom().primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  name: varchar('name', { length: 255 }).notNull(),
  nameAr: varchar('name_ar', { length: 255 }),
  description: text('description'),

  category: templateCategoryEnum('category').default('certificate').notNull(),

  // Design
  content: jsonb('content').notNull().default('{}'),
  css: text('css'),
  fonts: jsonb('fonts').default('[]'),

  // Preview
  thumbnailUrl: text('thumbnail_url'),
  previewUrl: text('preview_url'),

  // Configuration
  pageSize: varchar('page_size', { length: 20 }).default('A4'),
  orientation: varchar('orientation', { length: 20 }).default('portrait'),
  margin: jsonb('margin').default('{"top": 20, "right": 20, "bottom": 20, "left": 20}'),

  // Variables
  variables: jsonb('variables').default('[]'), // [{name, type, label, required}]

  // Status
  isDefault: boolean('is_default').default(false).notNull(),
  isArchived: boolean('is_archived').default(false).notNull(),
  isActive: boolean('is_active').default(true).notNull(),

  version: integer('version').default(1),
  tags: jsonb('tags').$type<string[]>(),

  createdById: uuid('created_by_id').references(() => users.id, { onDelete: 'set null' }),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  certTempOrgIdx: index('cert_temp_org_idx').on(table.organizationId),
  certTempCategoryIdx: index('cert_temp_category_idx').on(table.category),
  certTempActiveIdx: index('cert_temp_active_idx').on(table.isActive),
}));

// Generated Certificates/Reports
export const generatedDocuments = pgTable('generated_documents', {
  id: uuid('id').defaultRandom().primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  templateId: uuid('template_id').references(() => certificateTemplates.id, { onDelete: 'set null' }),

  // Document Info
  name: varchar('name', { length: 255 }).notNull(),
  type: reportTypeEnum('type').default('academic').notNull(),
  format: reportFormatEnum('format').default('pdf').notNull(),

  // For
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'set null' }),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'set null' }),

  // Content
  data: jsonb('data').notNull(), // The actual data used to generate
  fileUrl: text('file_url'),
  fileSize: integer('file_size'),
  pageCount: integer('page_count'),

  // Status
  status: varchar('status', { length: 50 }).default('generated').notNull(), // generated, printed, emailed, downloaded, revoked

  // Security
  verificationCode: varchar('verification_code', { length: 100 }),
  isVerified: boolean('is_verified').default(false),

  // Delivery
  printedAt: timestamp('printed_at'),
  printedById: uuid('printed_by_id').references(() => users.id, { onDelete: 'set null' }),
  emailedAt: timestamp('emailed_at'),
  emailedTo: jsonb('emailed_to').default('[]'),
  downloadedAt: timestamp('downloaded_at'),
  downloadCount: integer('download_count').default(0),

  // Signature
  signedById: uuid('signed_by_id').references(() => users.id, { onDelete: 'set null' }),
  signedAt: timestamp('signed_at'),
  digitalSignature: text('digital_signature'),

  notes: text('notes'),
  metadata: jsonb('metadata').default('{}'),

  createdById: uuid('created_by_id').references(() => users.id, { onDelete: 'set null' }),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  genDocOrgIdx: index('gen_doc_org_idx').on(table.organizationId),
  genDocTemplateIdx: index('gen_doc_template_idx').on(table.templateId),
  genDocStudentIdx: index('gen_doc_student_idx').on(table.studentId),
  genDocClassIdx: index('gen_doc_class_idx').on(table.classId),
  genDocYearIdx: index('gen_doc_year_idx').on(table.academicYearId),
  genDocStatusIdx: index('gen_doc_status_idx').on(table.status),
  genDocTypeIdx: index('gen_doc_type_idx').on(table.type),
}));

// ==========================================
// SECTION 15: AUDIT LOGS & SYSTEM MONITORING
// ==========================================

export const auditLogs = pgTable('audit_logs', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }),

  // Action Details
  action: varchar('action', { length: 50 }).notNull(), // create, update, delete, login, logout, export, print
  entityType: varchar('entity_type', { length: 50 }).notNull(), // students, grades, fees, etc.
  entityId: varchar('entity_id', { length: 100 }).notNull(),

  // Data
  oldData: jsonb('old_data'),
  newData: jsonb('new_data'),
  changes: jsonb('changes').default('[]'), // Diff of changes

  // Context
  description: text('description'),
  reason: text('reason'),

  // User
  userId: uuid('user_id').references(() => users.id, { onDelete: 'set null' }),
  userRole: varchar('user_role', { length: 50 }),
  impersonatedById: uuid('impersonated_by_id').references(() => users.id, { onDelete: 'set null' }),

  // Request Context
  ipAddress: varchar('ip_address', { length: 45 }),
  userAgent: text('user_agent'),
  requestId: varchar('request_id', { length: 100 }),
  sessionId: varchar('session_id', { length: 255 }),

  // Location
  location: jsonb('location').default('{}'),
  device: jsonb('device').default('{}'),

  // Result
  status: varchar('status', { length: 20 }).default('success').notNull(), // success, failed, denied
  errorMessage: text('error_message'),

  // Performance
  executionTimeMs: integer('execution_time_ms'),

  timestamp: timestamp('timestamp').defaultNow().notNull(),
}, (table) => ({
  // Primary lookups
  auditOrgIdx: index('audit_org_idx').on(table.organizationId),
  auditUserIdx: index('audit_user_idx').on(table.userId),
  auditActionIdx: index('audit_action_idx').on(table.action),
  auditEntityIdx: index('audit_entity_idx').on(table.entityType, table.entityId),
  auditStatusIdx: index('audit_status_idx').on(table.status),

  // Time-based (for cleanup)
  auditTimestampIdx: index('audit_timestamp_idx').on(table.timestamp),
  auditDateIdx: index('audit_date_idx').on(sql`DATE(${table.timestamp})`),

  // Composite
  auditUserActionIdx: index('audit_user_action_idx').on(table.userId, table.action),
  auditEntityActionIdx: index('audit_entity_action_idx').on(table.entityType, table.entityId, table.action),
}));

// System Activity Log (For performance monitoring)
export const systemLogs = pgTable('system_logs', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }),

  level: varchar('level', { length: 20 }).notNull(), // debug, info, warn, error, fatal
  category: varchar('category', { length: 50 }).notNull(), // api, database, auth, payment, notification

  message: text('message').notNull(),
  details: jsonb('details').default('{}'),
  stackTrace: text('stack_trace'),

  // Context
  userId: uuid('user_id').references(() => users.id, { onDelete: 'set null' }),
  requestId: varchar('request_id', { length: 100 }),
  endpoint: varchar('endpoint', { length: 255 }),
  method: varchar('method', { length: 10 }),

  // Performance
  durationMs: integer('duration_ms'),
  memoryUsageMb: integer('memory_usage_mb'),

  timestamp: timestamp('timestamp').defaultNow().notNull(),
}, (table) => ({
  sysLogOrgIdx: index('sys_log_org_idx').on(table.organizationId),
  sysLogLevelIdx: index('sys_log_level_idx').on(table.level),
  sysLogCategoryIdx: index('sys_log_category_idx').on(table.category),
  sysLogUserIdx: index('sys_log_user_idx').on(table.userId),
  sysLogTimestampIdx: index('sys_log_timestamp_idx').on(table.timestamp),
  sysLogEndpointIdx: index('sys_log_endpoint_idx').on(table.endpoint),
}));

// Data Import/Export Logs
export const dataTransfers = pgTable('data_transfers', {
  id: serial('id').primaryKey(),
  organizationId: uuid('organization_id').references(() => organizations.id, { onDelete: 'cascade' }).notNull(),

  type: varchar('type', { length: 20 }).notNull(), // import, export, migration
  entityType: varchar('entity_type', { length: 50 }).notNull(), // students, grades, fees

  // File
  fileName: varchar('file_name', { length: 255 }),
  fileUrl: text('file_url'),
  fileSize: integer('file_size'),
  fileFormat: varchar('file_format', { length: 20 }), // csv, excel, json, xml

  // Stats
  totalRecords: integer('total_records').default(0),
  successRecords: integer('success_records').default(0),
  failedRecords: integer('failed_records').default(0),
  skippedRecords: integer('skipped_records').default(0),

  // Errors
  errorLog: jsonb('error_log').default('[]'),
  warningLog: jsonb('warning_log').default('[]'),

  // Status
  status: varchar('status', { length: 50 }).default('pending').notNull(), // pending, processing, completed, failed, cancelled
  startedAt: timestamp('started_at'),
  completedAt: timestamp('completed_at'),

  // User
  initiatedById: uuid('initiated_by_id').references(() => users.id, { onDelete: 'set null' }),

  // Configuration
  config: jsonb('config').default('{}'),
  mapping: jsonb('mapping').default('{}'),

  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  transferOrgIdx: index('transfer_org_idx').on(table.organizationId),
  transferTypeIdx: index('transfer_type_idx').on(table.type),
  transferEntityIdx: index('transfer_entity_idx').on(table.entityType),
  transferStatusIdx: index('transfer_status_idx').on(table.status),
  transferUserIdx: index('transfer_user_idx').on(table.initiatedById),
}));


// ============================================================
// PART 11: 11 RELATIONS
// ============================================================

// ==========================================
// SECTION 16: RELATIONS
// ==========================================

export const organizationsRelations = relations(organizations, ({ many }) => ({
  users: many(users, { relationName: "orgUsers" }),
  schools: many(schools, { relationName: "orgSchools" }),
  departments: many(departments, { relationName: "orgDepartments" }),
  academicYears: many(academicYears, { relationName: "orgAcademicYears" }),
  subjects: many(subjects, { relationName: "orgSubjects" }),
  classes: many(classes, { relationName: "orgClasses" }),
  roles: many(roles, { relationName: "orgRoles" }),
  staff: many(staffMembers, { relationName: "orgStaff" }),
  students: many(students, { relationName: "orgStudents" }),
  feeCategories: many(feeCategories, { relationName: "orgFeeCategories" }),
  feeTypes: many(feeTypes, { relationName: "orgFeeTypes" }),
  treasuries: many(treasuries, { relationName: "orgTreasuries" }),
  notifications: many(notifications, { relationName: "orgNotifications" }),
  documents: many(documents, { relationName: "orgDocuments" }),
  certificateTemplates: many(certificateTemplates, { relationName: "orgTemplates" }),
  gradeGroups: many(gradeGroups, { relationName: "orgGradeGroups" }),
  timeSlots: many(timeSlots, { relationName: "orgTimeSlots" }),
  periodLocks: many(periodLocks, { relationName: "orgLocks" }),
}));

export const usersRelations = relations(users, ({ one, many }) => ({
  organization: one(organizations, { fields: [users.organizationId], references: [organizations.id], relationName: "orgUsers" }),
  profile: one(userProfiles, { fields: [users.id], references: [userProfiles.userId] }),
  accounts: many(userAccounts, { relationName: "userAccounts" }),
  sessions: many(userSessions, { relationName: "userSessions" }),

  // Roles & Permissions
  userRoles: many(userRoles, { relationName: "userRoleAssignments" }),
  userPermissions: many(userPermissions, { relationName: "userDirectPermissions" }),

  // Academic
  studentProfile: one(students, { fields: [users.id], references: [students.userId], relationName: "studentUser" }),
  staffProfile: one(staffMembers, { fields: [users.id], references: [staffMembers.userId], relationName: "staffUser" }),

  // Notifications
  sentNotifications: many(notifications, { relationName: "sentNotifications" }),
  receivedNotifications: many(notifications, { relationName: "receivedNotifications" }),
  notificationPreferences: many(notificationPreferences, { relationName: "userNotifPrefs" }),

  // Documents
  documents: many(documents, { relationName: "userDocuments" }),
  shortLinks: many(shortLinks, { relationName: "userShortLinks" }),

  // Audit
  auditLogs: many(auditLogs, { relationName: "userAuditLogs" }),
  systemLogs: many(systemLogs, { relationName: "userSystemLogs" }),

  // Grading
  gradedEntries: many(gradeEntries, { relationName: "gradedByUser" }),
  gradeHistory: many(gradeEntryHistory, { relationName: "gradeHistoryByUser" }),

  // Attendance
  markedAttendance: many(attendanceRecords, { relationName: "markedAttendance" }),

  // Teaching
  taughtLessons: many(scheduledLessons, { relationName: "teacherLessons" }),
  substitutedLessons: many(scheduledLessons, { relationName: "substituteLessons" }),

  // Decisions
  promotionDecisions: many(studentPromotions, { relationName: "promotionDecisions" }),
  lockOverrides: many(periodLocks, { relationName: "lockOverrides" }),
}));

export const userProfilesRelations = relations(userProfiles, ({ one }) => ({
  user: one(users, { fields: [userProfiles.userId], references: [users.id] }),
}));

export const userAccountsRelations = relations(userAccounts, ({ one }) => ({
  user: one(users, { fields: [userAccounts.userId], references: [users.id], relationName: "userAccounts" }),
}));

export const userSessionsRelations = relations(userSessions, ({ one }) => ({
  user: one(users, { fields: [userSessions.userId], references: [users.id], relationName: "userSessions" }),
}));

export const rolesRelations = relations(roles, ({ one, many }) => ({
  organization: one(organizations, { fields: [roles.organizationId], references: [organizations.id], relationName: "orgRoles" }),
  permissions: many(rolePermissions, { relationName: "rolePermissions" }),
  userAssignments: many(userRoles, { relationName: "roleUserAssignments" }),
}));

export const permissionsRelations = relations(permissions, ({ many }) => ({
  roleAssignments: many(rolePermissions, { relationName: "permissionRoleAssignments" }),
  userAssignments: many(userPermissions, { relationName: "permissionUserAssignments" }),
}));

export const rolePermissionsRelations = relations(rolePermissions, ({ one }) => ({
  role: one(roles, { fields: [rolePermissions.roleId], references: [roles.id], relationName: "rolePermissions" }),
  permission: one(permissions, { fields: [rolePermissions.permissionId], references: [permissions.id], relationName: "permissionRoleAssignments" }),
}));

export const userRolesRelations = relations(userRoles, ({ one }) => ({
  user: one(users, { fields: [userRoles.userId], references: [users.id], relationName: "userRoleAssignments" }),
  role: one(roles, { fields: [userRoles.roleId], references: [roles.id], relationName: "roleUserAssignments" }),
}));

export const schoolsRelations = relations(schools, ({ one, many }) => ({
  organization: one(organizations, { fields: [schools.organizationId], references: [organizations.id], relationName: "orgSchools" }),
  principal: one(users, { fields: [schools.principalId], references: [users.id] }),
  academicYears: many(academicYears, { relationName: "schoolAcademicYears" }),
  classes: many(classes, { relationName: "schoolClasses" }),
  departments: many(departments, { relationName: "schoolDepartments" }),
  staff: many(staffMembers, { relationName: "schoolStaff" }),
  treasuries: many(treasuries, { relationName: "schoolTreasuries" }),
}));

export const departmentsRelations = relations(departments, ({ one, many }) => ({
  organization: one(organizations, { fields: [departments.organizationId], references: [organizations.id], relationName: "orgDepartments" }),
  school: one(schools, { fields: [departments.schoolId], references: [schools.id], relationName: "schoolDepartments" }),
  head: one(users, { fields: [departments.headId], references: [users.id] }),
  parent: one(departments, { fields: [departments.parentDepartmentId], references: [departments.id], relationName: "deptChildren" }),
  children: many(departments, { relationName: "deptChildren" }),
  subjects: many(subjects, { relationName: "deptSubjects" }),
  staff: many(staffMembers, { relationName: "deptStaff" }),
}));

export const academicYearsRelations = relations(academicYears, ({ one, many }) => ({
  organization: one(organizations, { fields: [academicYears.organizationId], references: [organizations.id], relationName: "orgAcademicYears" }),
  school: one(schools, { fields: [academicYears.schoolId], references: [schools.id], relationName: "schoolAcademicYears" }),
  terms: many(academicTerms, { relationName: "yearTerms" }),
  classes: many(classes, { relationName: "yearClasses" }),
  enrollments: many(enrollments, { relationName: "yearEnrollments" }),
  gradeGroups: many(gradeGroups, { relationName: "yearGradeGroups" }),
  assessments: many(assessmentItems, { relationName: "yearAssessments" }),
  promotionsFrom: many(studentPromotions, { relationName: "promotionsFromYear" }),
  promotionsTo: many(studentPromotions, { relationName: "promotionsToYear" }),
  timeSlots: many(timeSlots, { relationName: "yearTimeSlots" }),
  lessons: many(scheduledLessons, { relationName: "yearLessons" }),
  studentFees: many(studentFees, { relationName: "yearStudentFees" }),
  periodLocks: many(periodLocks, { relationName: "yearLocks" }),
}));

export const academicTermsRelations = relations(academicTerms, ({ one, many }) => ({
  year: one(academicYears, { fields: [academicTerms.academicYearId], references: [academicYears.id], relationName: "yearTerms" }),
  assessments: many(assessmentItems, { relationName: "termAssessments" }),
  lessons: many(scheduledLessons, { relationName: "termLessons" }),
  notifications: many(notifications, { relationName: "termNotifications" }),
  studentFees: many(studentFees, { relationName: "termStudentFees" }),
  periodLocks: many(periodLocks, { relationName: "termLocks" }),
}));

export const subjectsRelations = relations(subjects, ({ one, many }) => ({
  organization: one(organizations, { fields: [subjects.organizationId], references: [organizations.id], relationName: "orgSubjects" }),
  department: one(departments, { fields: [subjects.departmentId], references: [departments.id], relationName: "deptSubjects" }),
  classSubjects: many(classSubjects, { relationName: "subjectClassSubjects" }),
  assessments: many(assessmentItems, { relationName: "subjectAssessments" }),
  lessons: many(scheduledLessons, { relationName: "subjectLessons" }),
  teacherSubjects: many(teacherSubjects, { relationName: "subjectTeacherSubjects" }),
}));

export const classesRelations = relations(classes, ({ one, many }) => ({
  organization: one(organizations, { fields: [classes.organizationId], references: [organizations.id], relationName: "orgClasses" }),
  school: one(schools, { fields: [classes.schoolId], references: [schools.id], relationName: "schoolClasses" }),
  academicYear: one(academicYears, { fields: [classes.academicYearId], references: [academicYears.id], relationName: "yearClasses" }),
  parent: one(classes, { fields: [classes.parentClassId], references: [classes.id], relationName: "classChildren" }),
  children: many(classes, { relationName: "classChildren" }),
  homeroomTeacher: one(users, { fields: [classes.homeroomTeacherId], references: [users.id] }),

  enrollments: many(enrollments, { relationName: "classEnrollments" }),
  students: many(students, { relationName: "classStudents" }),
  classSubjects: many(classSubjects, { relationName: "classClassSubjects" }),
  assessments: many(assessmentItems, { relationName: "classAssessments" }),
  lessons: many(scheduledLessons, { relationName: "classLessons" }),
  notifications: many(notifications, { relationName: "classNotifications" }),
  promotionsFrom: many(studentPromotions, { relationName: "promotionsFromClass" }),
  promotionsTo: many(studentPromotions, { relationName: "promotionsToClass" }),
}));

export const classSubjectsRelations = relations(classSubjects, ({ one, many }) => ({
  class: one(classes, { fields: [classSubjects.classId], references: [classes.id], relationName: "classClassSubjects" }),
  subject: one(subjects, { fields: [classSubjects.subjectId], references: [subjects.id], relationName: "subjectClassSubjects" }),
  academicYear: one(academicYears, { fields: [classSubjects.academicYearId], references: [academicYears.id] }),
  primaryTeacher: one(users, { fields: [classSubjects.primaryTeacherId], references: [users.id] }),
  secondaryTeacher: one(users, { fields: [classSubjects.secondaryTeacherId], references: [users.id] }),
  gradeGroups: many(classSubjectGradeGroups, { relationName: "classSubjectGradeGroups" }),
  assessments: many(assessmentItems, { relationName: "classSubjectAssessments" }),
  studentAverages: many(studentSubjectAverages, { relationName: "classSubjectAverages" }),
}));

export const studentsRelations = relations(students, ({ one, many }) => ({
  user: one(users, { fields: [students.userId], references: [users.id], relationName: "studentUser" }),
  organization: one(organizations, { fields: [students.organizationId], references: [organizations.id], relationName: "orgStudents" }),
  currentClass: one(classes, { fields: [students.currentClassId], references: [classes.id], relationName: "classStudents" }),
  currentYear: one(academicYears, { fields: [students.currentAcademicYearId], references: [academicYears.id] }),

  enrollments: many(enrollments, { relationName: "studentEnrollments" }),
  grades: many(gradeEntries, { relationName: "studentGrades" }),
  gradeHistory: many(gradeEntryHistory, { relationName: "studentGradeHistory" }),
  attendanceRecords: many(attendanceRecords, { relationName: "studentAttendance" }),
  attendanceSummaries: many(attendanceSummaries, { relationName: "studentAttendanceSummaries" }),
  behaviorRecords: many(behaviorRecords, { relationName: "studentBehaviors" }),
  guardianLinks: many(studentGuardians, { relationName: "studentGuardians" }),
  documents: many(studentDocuments, { relationName: "studentDocuments" }),
  promotions: many(studentPromotions, { relationName: "studentPromotions" }),
  studentFees: many(studentFees, { relationName: "studentFees" }),
  feePayments: many(feePayments, { relationName: "studentFeePayments" }),
  discounts: many(studentDiscounts, { relationName: "studentDiscounts" }),
  subjectAverages: many(studentSubjectAverages, { relationName: "studentSubjectAverages" }),
  generatedDocuments: many(generatedDocuments, { relationName: "studentDocuments" }),
}));

export const enrollmentsRelations = relations(enrollments, ({ one, many }) => ({
  student: one(students, { fields: [enrollments.studentId], references: [students.userId], relationName: "studentEnrollments" }),
  class: one(classes, { fields: [enrollments.classId], references: [classes.id], relationName: "classEnrollments" }),
  academicYear: one(academicYears, { fields: [enrollments.academicYearId], references: [academicYears.id], relationName: "yearEnrollments" }),

  grades: many(gradeEntries, { relationName: "enrollmentGrades" }),
  attendanceRecords: many(attendanceRecords, { relationName: "enrollmentAttendance" }),
  attendanceSummaries: many(attendanceSummaries, { relationName: "enrollmentAttendanceSummaries" }),
  studentFees: many(studentFees, { relationName: "enrollmentFees" }),
  promotions: many(studentPromotions, { relationName: "enrollmentPromotions" }),
}));

export const studentGuardiansRelations = relations(studentGuardians, ({ one }) => ({
  student: one(students, { fields: [studentGuardians.studentId], references: [students.userId], relationName: "studentGuardians" }),
  guardian: one(users, { fields: [studentGuardians.guardianId], references: [users.id], relationName: "guardianStudents" }),
}));

export const staffMembersRelations = relations(staffMembers, ({ one, many }) => ({
  user: one(users, { fields: [staffMembers.userId], references: [users.id], relationName: "staffUser" }),
  organization: one(organizations, { fields: [staffMembers.organizationId], references: [organizations.id], relationName: "orgStaff" }),
  school: one(schools, { fields: [staffMembers.schoolId], references: [schools.id], relationName: "schoolStaff" }),
  department: one(departments, { fields: [staffMembers.departmentId], references: [departments.id], relationName: "deptStaff" }),

  teacherSubjects: many(teacherSubjects, { relationName: "staffTeacherSubjects" }),
  teacherClasses: many(teacherClasses, { relationName: "staffTeacherClasses" }),
  leaves: many(staffLeaves, { relationName: "staffLeaves" }),
  leaveBalances: many(leaveBalances, { relationName: "staffLeaveBalances" }),
  attendance: many(staffAttendance, { relationName: "staffAttendance" }),
}));

export const teacherSubjectsRelations = relations(teacherSubjects, ({ one }) => ({
  staff: one(staffMembers, { fields: [teacherSubjects.staffId], references: [staffMembers.id], relationName: "staffTeacherSubjects" }),
  subject: one(subjects, { fields: [teacherSubjects.subjectId], references: [subjects.id], relationName: "subjectTeacherSubjects" }),
  academicYear: one(academicYears, { fields: [teacherSubjects.academicYearId], references: [academicYears.id] }),
}));

export const teacherClassesRelations = relations(teacherClasses, ({ one }) => ({
  staff: one(staffMembers, { fields: [teacherClasses.staffId], references: [staffMembers.id], relationName: "staffTeacherClasses" }),
  class: one(classes, { fields: [teacherClasses.classId], references: [classes.id], relationName: "classTeacherClasses" }),
  subject: one(subjects, { fields: [teacherClasses.subjectId], references: [subjects.id] }),
  academicYear: one(academicYears, { fields: [teacherClasses.academicYearId], references: [academicYears.id] }),
}));

export const gradeGroupsRelations = relations(gradeGroups, ({ one, many }) => ({
  organization: one(organizations, { fields: [gradeGroups.organizationId], references: [organizations.id], relationName: "orgGradeGroups" }),
  academicYear: one(academicYears, { fields: [gradeGroups.academicYearId], references: [academicYears.id], relationName: "yearGradeGroups" }),
  classSubjectMappings: many(classSubjectGradeGroups, { relationName: "gradeGroupMappings" }),
  assessments: many(assessmentItems, { relationName: "gradeGroupAssessments" }),
}));

export const classSubjectGradeGroupsRelations = relations(classSubjectGradeGroups, ({ one }) => ({
  classSubject: one(classSubjects, { fields: [classSubjectGradeGroups.classSubjectId], references: [classSubjects.id], relationName: "classSubjectGradeGroups" }),
  gradeGroup: one(gradeGroups, { fields: [classSubjectGradeGroups.gradeGroupId], references: [gradeGroups.id], relationName: "gradeGroupMappings" }),
}));

export const assessmentItemsRelations = relations(assessmentItems, ({ one, many }) => ({
  organization: one(organizations, { fields: [assessmentItems.organizationId], references: [organizations.id], relationName: "orgAssessments" }),
  academicYear: one(academicYears, { fields: [assessmentItems.academicYearId], references: [academicYears.id], relationName: "yearAssessments" }),
  academicTerm: one(academicTerms, { fields: [assessmentItems.academicTermId], references: [academicTerms.id], relationName: "termAssessments" }),
  classSubject: one(classSubjects, { fields: [assessmentItems.classSubjectId], references: [classSubjects.id], relationName: "classSubjectAssessments" }),
  gradeGroup: one(gradeGroups, { fields: [assessmentItems.gradeGroupId], references: [gradeGroups.id], relationName: "gradeGroupAssessments" }),

  grades: many(gradeEntries, { relationName: "assessmentGrades" }),
  publishedBy: one(users, { fields: [assessmentItems.publishedById], references: [users.id] }),
  lockedBy: one(users, { fields: [assessmentItems.lockedById], references: [users.id] }),
}));

export const gradeEntriesRelations = relations(gradeEntries, ({ one, many }) => ({
  academicYear: one(academicYears, { fields: [gradeEntries.academicYearId], references: [academicYears.id] }),
  student: one(students, { fields: [gradeEntries.studentId], references: [students.userId], relationName: "studentGrades" }),
  assessmentItem: one(assessmentItems, { fields: [gradeEntries.assessmentItemId], references: [assessmentItems.id], relationName: "assessmentGrades" }),
  enrollment: one(enrollments, { fields: [gradeEntries.enrollmentId], references: [enrollments.id], relationName: "enrollmentGrades" }),
  grader: one(users, { fields: [gradeEntries.gradedById], references: [users.id], relationName: "gradedByUser" }),
  modifier: one(users, { fields: [gradeEntries.modifiedById], references: [users.id] }),
  history: many(gradeEntryHistory, { relationName: "gradeHistory" }),
}));

export const gradeEntryHistoryRelations = relations(gradeEntryHistory, ({ one }) => ({
  gradeEntry: one(gradeEntries, { fields: [gradeEntryHistory.gradeEntryId], references: [gradeEntries.id], relationName: "gradeHistory" }),
  changedBy: one(users, { fields: [gradeEntryHistory.changedById], references: [users.id], relationName: "gradeHistoryByUser" }),
}));

export const studentSubjectAveragesRelations = relations(studentSubjectAverages, ({ one }) => ({
  student: one(students, { fields: [studentSubjectAverages.studentId], references: [students.userId], relationName: "studentSubjectAverages" }),
  classSubject: one(classSubjects, { fields: [studentSubjectAverages.classSubjectId], references: [classSubjects.id], relationName: "classSubjectAverages" }),
  academicYear: one(academicYears, { fields: [studentSubjectAverages.academicYearId], references: [academicYears.id] }),
  academicTerm: one(academicTerms, { fields: [studentSubjectAverages.academicTermId], references: [academicTerms.id] }),
}));

export const timeSlotsRelations = relations(timeSlots, ({ one, many }) => ({
  organization: one(organizations, { fields: [timeSlots.organizationId], references: [organizations.id], relationName: "orgTimeSlots" }),
  academicYear: one(academicYears, { fields: [timeSlots.academicYearId], references: [academicYears.id], relationName: "yearTimeSlots" }),
  lessons: many(scheduledLessons, { relationName: "slotLessons" }),
}));

export const scheduledLessonsRelations = relations(scheduledLessons, ({ one, many }) => ({
  organization: one(organizations, { fields: [scheduledLessons.organizationId], references: [organizations.id], relationName: "orgLessons" }),
  academicYear: one(academicYears, { fields: [scheduledLessons.academicYearId], references: [academicYears.id], relationName: "yearLessons" }),
  academicTerm: one(academicTerms, { fields: [scheduledLessons.academicTermId], references: [academicTerms.id], relationName: "termLessons" }),
  class: one(classes, { fields: [scheduledLessons.classId], references: [classes.id], relationName: "classLessons" }),
  classSubject: one(classSubjects, { fields: [scheduledLessons.classSubjectId], references: [classSubjects.id] }),
  subject: one(subjects, { fields: [scheduledLessons.subjectId], references: [subjects.id], relationName: "subjectLessons" }),
  teacher: one(users, { fields: [scheduledLessons.teacherId], references: [users.id], relationName: "teacherLessons" }),
  substituteTeacher: one(users, { fields: [scheduledLessons.substituteTeacherId], references: [users.id], relationName: "substituteLessons" }),
  timeSlot: one(timeSlots, { fields: [scheduledLessons.timeSlotId], references: [timeSlots.id], relationName: "slotLessons" }),
  attendanceRecords: many(attendanceRecords, { relationName: "lessonAttendance" }),
}));

export const attendanceRecordsRelations = relations(attendanceRecords, ({ one }) => ({
  organization: one(organizations, { fields: [attendanceRecords.organizationId], references: [organizations.id] }),
  academicYear: one(academicYears, { fields: [attendanceRecords.academicYearId], references: [academicYears.id] }),
  student: one(students, { fields: [attendanceRecords.studentId], references: [students.userId], relationName: "studentAttendance" }),
  scheduledLesson: one(scheduledLessons, { fields: [attendanceRecords.scheduledLessonId], references: [scheduledLessons.id], relationName: "lessonAttendance" }),
  enrollment: one(enrollments, { fields: [attendanceRecords.enrollmentId], references: [enrollments.id], relationName: "enrollmentAttendance" }),
  markedBy: one(users, { fields: [attendanceRecords.markedById], references: [users.id], relationName: "markedAttendance" }),
  verifiedBy: one(users, { fields: [attendanceRecords.verifiedById], references: [users.id] }),
}));

export const attendanceSummariesRelations = relations(attendanceSummaries, ({ one }) => ({
  student: one(students, { fields: [attendanceSummaries.studentId], references: [students.userId], relationName: "studentAttendanceSummaries" }),
  enrollment: one(enrollments, { fields: [attendanceSummaries.enrollmentId], references: [enrollments.id], relationName: "enrollmentAttendanceSummaries" }),
  academicYear: one(academicYears, { fields: [attendanceSummaries.academicYearId], references: [academicYears.id] }),
  academicTerm: one(academicTerms, { fields: [attendanceSummaries.academicTermId], references: [academicTerms.id] }),
}));

export const behaviorRecordsRelations = relations(behaviorRecords, ({ one }) => ({
  organization: one(organizations, { fields: [behaviorRecords.organizationId], references: [organizations.id] }),
  academicYear: one(academicYears, { fields: [behaviorRecords.academicYearId], references: [academicYears.id] }),
  student: one(students, { fields: [behaviorRecords.studentId], references: [students.userId], relationName: "studentBehaviors" }),
  enrollment: one(enrollments, { fields: [behaviorRecords.enrollmentId], references: [enrollments.id] }),
  reportedBy: one(users, { fields: [behaviorRecords.reportedById], references: [users.id] }),
  reviewedBy: one(users, { fields: [behaviorRecords.reviewedById], references: [users.id] }),
  resolvedBy: one(users, { fields: [behaviorRecords.resolvedById], references: [users.id] }),
}));

export const feeCategoriesRelations = relations(feeCategories, ({ one, many }) => ({
  organization: one(organizations, { fields: [feeCategories.organizationId], references: [organizations.id], relationName: "orgFeeCategories" }),
  feeTypes: many(feeTypes, { relationName: "categoryFeeTypes" }),
}));

export const feeTypesRelations = relations(feeTypes, ({ one, many }) => ({
  organization: one(organizations, { fields: [feeTypes.organizationId], references: [organizations.id], relationName: "orgFeeTypes" }),
  category: one(feeCategories, { fields: [feeTypes.categoryId], references: [feeCategories.id], relationName: "categoryFeeTypes" }),
  studentFees: many(studentFees, { relationName: "feeTypeStudentFees" }),
}));

export const studentFeesRelations = relations(studentFees, ({ one, many }) => ({
  organization: one(organizations, { fields: [studentFees.organizationId], references: [organizations.id] }),
  academicYear: one(academicYears, { fields: [studentFees.academicYearId], references: [academicYears.id], relationName: "yearStudentFees" }),
  academicTerm: one(academicTerms, { fields: [studentFees.academicTermId], references: [academicTerms.id], relationName: "termStudentFees" }),
  student: one(students, { fields: [studentFees.studentId], references: [students.userId], relationName: "studentFees" }),
  enrollment: one(enrollments, { fields: [studentFees.enrollmentId], references: [enrollments.id], relationName: "enrollmentFees" }),
  feeType: one(feeTypes, { fields: [studentFees.feeTypeId], references: [feeTypes.id], relationName: "feeTypeStudentFees" }),
  payments: many(feePayments, { relationName: "feePayments" }),
  discounts: many(studentDiscounts, { relationName: "feeDiscounts" }),
}));

export const feePaymentsRelations = relations(feePayments, ({ one }) => ({
  organization: one(organizations, { fields: [feePayments.organizationId], references: [organizations.id] }),
  studentFee: one(studentFees, { fields: [feePayments.studentFeeId], references: [studentFees.id], relationName: "feePayments" }),
  student: one(students, { fields: [feePayments.studentId], references: [students.userId], relationName: "studentFeePayments" }),
  treasury: one(treasuries, { fields: [feePayments.treasuryId], references: [treasuries.id] }),
  verifiedBy: one(users, { fields: [feePayments.verifiedById], references: [users.id] }),
  createdBy: one(users, { fields: [feePayments.createdById], references: [users.id] }),
}));

export const feeDiscountsRelations = relations(feeDiscounts, ({ one, many }) => ({
  organization: one(organizations, { fields: [feeDiscounts.organizationId], references: [organizations.id] }),
  studentDiscounts: many(studentDiscounts, { relationName: "discountApplications" }),
}));

export const studentDiscountsRelations = relations(studentDiscounts, ({ one }) => ({
  student: one(students, { fields: [studentDiscounts.studentId], references: [students.userId], relationName: "studentDiscounts" }),
  discount: one(feeDiscounts, { fields: [studentDiscounts.discountId], references: [feeDiscounts.id], relationName: "discountApplications" }),
  academicYear: one(academicYears, { fields: [studentDiscounts.academicYearId], references: [academicYears.id] }),
  appliedToFee: one(studentFees, { fields: [studentDiscounts.appliedToFeeId], references: [studentFees.id], relationName: "feeDiscounts" }),
}));

export const treasuriesRelations = relations(treasuries, ({ one, many }) => ({
  organization: one(organizations, { fields: [treasuries.organizationId], references: [organizations.id], relationName: "orgTreasuries" }),
  school: one(schools, { fields: [treasuries.schoolId], references: [schools.id], relationName: "schoolTreasuries" }),
  transactions: many(treasuryTransactions, { relationName: "treasuryTransactions" }),
  feePayments: many(feePayments, { relationName: "treasuryPayments" }),
}));

export const treasuryTransactionsRelations = relations(treasuryTransactions, ({ one }) => ({
  organization: one(organizations, { fields: [treasuryTransactions.organizationId], references: [organizations.id] }),
  treasury: one(treasuries, { fields: [treasuryTransactions.treasuryId], references: [treasuries.id], relationName: "treasuryTransactions" }),
  createdBy: one(users, { fields: [treasuryTransactions.createdById], references: [users.id] }),
  approvedBy: one(users, { fields: [treasuryTransactions.approvedById], references: [users.id] }),
}));

export const periodLocksRelations = relations(periodLocks, ({ one, many }) => ({
  organization: one(organizations, { fields: [periodLocks.organizationId], references: [organizations.id], relationName: "orgLocks" }),
  academicYear: one(academicYears, { fields: [periodLocks.academicYearId], references: [academicYears.id], relationName: "yearLocks" }),
  academicTerm: one(academicTerms, { fields: [periodLocks.academicTermId], references: [academicTerms.id], relationName: "termLocks" }),
  overriddenBy: one(users, { fields: [periodLocks.overriddenById], references: [users.id], relationName: "lockOverrides" }),
  createdBy: one(users, { fields: [periodLocks.createdById], references: [users.id] }),
  exceptions: many(lockExceptions, { relationName: "lockExceptions" }),
}));

export const lockExceptionsRelations = relations(lockExceptions, ({ one }) => ({
  periodLock: one(periodLocks, { fields: [lockExceptions.periodLockId], references: [periodLocks.id], relationName: "lockExceptions" }),
  grantedBy: one(users, { fields: [lockExceptions.grantedById], references: [users.id] }),
}));

export const studentPromotionsRelations = relations(studentPromotions, ({ one }) => ({
  organization: one(organizations, { fields: [studentPromotions.organizationId], references: [organizations.id] }),
  student: one(students, { fields: [studentPromotions.studentId], references: [students.userId], relationName: "studentPromotions" }),
  enrollment: one(enrollments, { fields: [studentPromotions.enrollmentId], references: [enrollments.id], relationName: "enrollmentPromotions" }),
  fromClass: one(classes, { fields: [studentPromotions.fromClassId], references: [classes.id], relationName: "promotionsFromClass" }),
  toClass: one(classes, { fields: [studentPromotions.toClassId], references: [classes.id], relationName: "promotionsToClass" }),
  fromYear: one(academicYears, { fields: [studentPromotions.fromAcademicYearId], references: [academicYears.id], relationName: "promotionsFromYear" }),
  toYear: one(academicYears, { fields: [studentPromotions.toAcademicYearId], references: [academicYears.id], relationName: "promotionsToYear" }),
  decisionBy: one(users, { fields: [studentPromotions.decisionById], references: [users.id], relationName: "promotionDecisions" }),
}));

export const notificationsRelations = relations(notifications, ({ one }) => ({
  organization: one(organizations, { fields: [notifications.organizationId], references: [organizations.id], relationName: "orgNotifications" }),
  sender: one(users, { fields: [notifications.senderId], references: [users.id], relationName: "sentNotifications" }),
  recipient: one(users, { fields: [notifications.recipientId], references: [users.id], relationName: "receivedNotifications" }),
  academicYear: one(academicYears, { fields: [notifications.academicYearId], references: [academicYears.id], relationName: "termNotifications" }),
  academicTerm: one(academicTerms, { fields: [notifications.academicTermId], references: [academicTerms.id], relationName: "termNotifications" }),
  class: one(classes, { fields: [notifications.classId], references: [classes.id], relationName: "classNotifications" }),
  subject: one(subjects, { fields: [notifications.subjectId], references: [subjects.id] }),
  student: one(students, { fields: [notifications.studentId], references: [students.userId] }),
}));

export const notificationPreferencesRelations = relations(notificationPreferences, ({ one }) => ({
  user: one(users, { fields: [notificationPreferences.userId], references: [users.id], relationName: "userNotifPrefs" }),
}));

export const notificationTemplatesRelations = relations(notificationTemplates, ({ one }) => ({
  organization: one(organizations, { fields: [notificationTemplates.organizationId], references: [organizations.id] }),
}));

export const documentsRelations = relations(documents, ({ one, many }) => ({
  organization: one(organizations, { fields: [documents.organizationId], references: [organizations.id], relationName: "orgDocuments" }),
  user: one(users, { fields: [documents.userId], references: [users.id], relationName: "userDocuments" }),
  uploadedBy: one(users, { fields: [documents.uploadedById], references: [users.id] }),
  parentFolder: one(documents, { fields: [documents.parentFolderId], references: [documents.id], relationName: "folderChildren" }),
  children: many(documents, { relationName: "folderChildren" }),
}));

export const shortLinksRelations = relations(shortLinks, ({ one }) => ({
  organization: one(organizations, { fields: [shortLinks.organizationId], references: [organizations.id] }),
  user: one(users, { fields: [shortLinks.userId], references: [users.id], relationName: "userShortLinks" }),
}));

export const certificateTemplatesRelations = relations(certificateTemplates, ({ one, many }) => ({
  organization: one(organizations, { fields: [certificateTemplates.organizationId], references: [organizations.id], relationName: "orgTemplates" }),
  createdBy: one(users, { fields: [certificateTemplates.createdById], references: [users.id] }),
  generatedDocuments: many(generatedDocuments, { relationName: "templateDocuments" }),
}));

export const generatedDocumentsRelations = relations(generatedDocuments, ({ one }) => ({
  organization: one(organizations, { fields: [generatedDocuments.organizationId], references: [organizations.id] }),
  template: one(certificateTemplates, { fields: [generatedDocuments.templateId], references: [certificateTemplates.id], relationName: "templateDocuments" }),
  student: one(students, { fields: [generatedDocuments.studentId], references: [students.userId], relationName: "studentDocuments" }),
  class: one(classes, { fields: [generatedDocuments.classId], references: [classes.id] }),
  academicYear: one(academicYears, { fields: [generatedDocuments.academicYearId], references: [academicYears.id] }),
  academicTerm: one(academicTerms, { fields: [generatedDocuments.academicTermId], references: [academicTerms.id] }),
  createdBy: one(users, { fields: [generatedDocuments.createdById], references: [users.id] }),
  signedBy: one(users, { fields: [generatedDocuments.signedById], references: [users.id] }),
  printedBy: one(users, { fields: [generatedDocuments.printedById], references: [users.id] }),
}));

export const auditLogsRelations = relations(auditLogs, ({ one }) => ({
  organization: one(organizations, { fields: [auditLogs.organizationId], references: [organizations.id] }),
  user: one(users, { fields: [auditLogs.userId], references: [users.id], relationName: "userAuditLogs" }),
  impersonatedBy: one(users, { fields: [auditLogs.impersonatedById], references: [users.id] }),
}));

export const systemLogsRelations = relations(systemLogs, ({ one }) => ({
  organization: one(organizations, { fields: [systemLogs.organizationId], references: [organizations.id] }),
  user: one(users, { fields: [systemLogs.userId], references: [users.id], relationName: "userSystemLogs" }),
}));

export const dataTransfersRelations = relations(dataTransfers, ({ one }) => ({
  organization: one(organizations, { fields: [dataTransfers.organizationId], references: [organizations.id] }),
  initiatedBy: one(users, { fields: [dataTransfers.initiatedById], references: [users.id] }),
}));

// Staff Leave Relations
export const staffLeavesRelations = relations(staffLeaves, ({ one }) => ({
  staff: one(staffMembers, { fields: [staffLeaves.staffId], references: [staffMembers.id], relationName: "staffLeaves" }),
  requestedBy: one(users, { fields: [staffLeaves.requestedById], references: [users.id] }),
  reviewedBy: one(users, { fields: [staffLeaves.reviewedById], references: [users.id] }),
  approvedBy: one(users, { fields: [staffLeaves.approvedById], references: [users.id] }),
}));

export const leaveBalancesRelations = relations(leaveBalances, ({ one }) => ({
  staff: one(staffMembers, { fields: [leaveBalances.staffId], references: [staffMembers.id], relationName: "staffLeaveBalances" }),
  academicYear: one(academicYears, { fields: [leaveBalances.academicYearId], references: [academicYears.id] }),
}));

export const staffAttendanceRelations = relations(staffAttendance, ({ one }) => ({
  staff: one(staffMembers, { fields: [staffAttendance.staffId], references: [staffMembers.id], relationName: "staffAttendance" }),
  markedBy: one(users, { fields: [staffAttendance.markedById], references: [users.id] }),
}));

// Student Documents Relations
export const studentDocumentsRelations = relations(studentDocuments, ({ one }) => ({
  student: one(students, { fields: [studentDocuments.studentId], references: [students.userId], relationName: "studentDocuments" }),
  verifiedBy: one(users, { fields: [studentDocuments.verifiedById], references: [users.id] }),
}));
