import {
  uuid,
  varchar,
  text,
  boolean,
  timestamp,
  date,
  integer,
  smallint,
  numeric,
  jsonb,
  index,
  unique,
  serial,
} from "drizzle-orm/pg-core";
import { sql } from "drizzle-orm";
import { financeSchema } from "./types";
import { moneyType, percentType } from "./types";

// ============================================================
// F1: LEDGER ACCOUNTS (Chart of Accounts)
// ============================================================

export const ledgerAccounts = financeSchema.table(
  "ledger_accounts",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    // معيار محاسبي هرمي
    code: varchar("code", { length: 50 }).notNull(),
    name: varchar("name", { length: 100 }).notNull(),
    nameAr: varchar("name_ar", { length: 100 }),

    // التصنيف
    type: varchar("type", { length: 30 }).notNull(),
    // asset | liability | equity | revenue | expense | contra

    subType: varchar("sub_type", { length: 30 }),
    // current_asset | fixed_asset | current_liability | long_term_liability | ...

    // هرمية غير محدودة العمق
    parentId: uuid("parent_id"),
    path: varchar("path", { length: 500 }).notNull(),
    depth: smallint("depth").default(0).notNull(),

    // العملة والأرصدة
    currency: varchar("currency", { length: 3 }).default("SAR").notNull(),
    openingBalance: moneyType("opening_balance").default(0),
    currentBalance: moneyType("current_balance").default(0),

    // أعلام التحكم
    isBankAccount: boolean("is_bank_account").default(false).notNull(),
    isCashAccount: boolean("is_cash_account").default(false).notNull(),
    isControlAccount: boolean("is_control_account").default(false).notNull(),

    // ربط خارجي (بنوك، بوابات دفع)
    externalRef: jsonb("external_ref").$type<{
      bankIban?: string;
      bankSwift?: string;
      paymentGatewayId?: string;
      erpCode?: string;
    }>().default("{}"),

    isActive: boolean("is_active").default(true).notNull(),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgCodeIdx: unique("ledger_org_code_idx").on(t.orgId, t.code),
    orgTypeIdx: index("ledger_org_type_idx").on(t.orgId, t.type, t.isActive),
    pathIdx: index("ledger_path_idx").on(t.path),
    parentIdx: index("ledger_parent_idx").on(t.parentId),
    bankIdx: index("ledger_bank_idx")
      .on(t.orgId, t.isBankAccount)
      .where(sql`${t.isBankAccount} = true`),
    cashIdx: index("ledger_cash_idx")
      .on(t.orgId, t.isCashAccount)
      .where(sql`${t.isCashAccount} = true`),
  })
);

// ============================================================
// F2: JOURNAL ENTRIES (Immutable Ledger)
// ============================================================

export const journalEntries = financeSchema.table(
  "journal_entries",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    // معرف فريد عالمي للمعاملة
    transactionRef: varchar("transaction_ref", { length: 100 }).notNull(),

    // التاريخ المحاسبي
    accountingDate: date("accounting_date").notNull(),
    fiscalPeriodId: uuid("fiscal_period_id").notNull(),

    // المبلغ (موجب = مدين، سالب = دائن)
    amount: moneyType("amount").notNull(),

    // الحسابات
    debitAccountId: uuid("debit_account_id").notNull(),
    creditAccountId: uuid("credit_account_id").notNull(),

    // العملة والتحويل
    currency: varchar("currency", { length: 3 }).default("SAR").notNull(),
    exchangeRate: numeric("exchange_rate", { precision: 15, scale: 8 }).default("1"),
    baseCurrencyAmount: moneyType("base_currency_amount"),

    // المصدر (أي نظام أتى منه)
    sourceType: varchar("source_type", { length: 30 }).notNull(),
    // fee_payment | salary | purchase | transfer | adjustment | grant | donation | ...

    sourceId: uuid("source_id"),
    sourceModule: varchar("source_module", { length: 30 }),
    // academics | finance | hr | procurement | inventory | research | ...

    // الوصف
    description: text("description").notNull(),
    descriptionAr: text("description_ar"),
    metadata: jsonb("metadata").$type<Record<string, unknown>>().default("{}"),

    // المراجعة والعكس
    isReversed: boolean("is_reversed").default(false).notNull(),
    reversedBy: uuid("reversed_by"),
    reversalReason: text("reversal_reason"),

    // التسوية
    isReconciled: boolean("is_reconciled").default(false).notNull(),
    reconciledAt: timestamp("reconciled_at"),
    reconciliationRef: varchar("reconciliation_ref", { length: 100 }),

    // المرحلة
    status: varchar("status", { length: 20 }).default("posted").notNull(),
    // draft | posted | pending_approval | approved | rejected | reversed

    postedBy: uuid("posted_by"),
    postedAt: timestamp("posted_at"),
    approvedBy: uuid("approved_by"),
    approvedAt: timestamp("approved_at"),

    // Audit
    createdBy: uuid("created_by"),
    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    refIdx: unique("journal_ref_idx").on(t.orgId, t.transactionRef),
    dateIdx: index("journal_date_idx").on(t.orgId, t.accountingDate),
    sourceIdx: index("journal_source_idx").on(t.sourceType, t.sourceId),
    statusIdx: index("journal_status_idx").on(t.orgId, t.status),
    debitIdx: index("journal_debit_idx").on(t.debitAccountId, t.accountingDate),
    creditIdx: index("journal_credit_idx").on(t.creditAccountId, t.accountingDate),
    unreconciledIdx: index("journal_unreconciled_idx")
      .on(t.orgId, t.isReconciled)
      .where(sql`${t.isReconciled} = false`),
    fiscalIdx: index("journal_fiscal_idx").on(t.fiscalPeriodId, t.status),
  })
);

// ============================================================
// F3: FEE TEMPLATES (Dynamic Pricing Engine)
// ============================================================

export const feeTemplates = financeSchema.table(
  "fee_templates",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    name: varchar("name", { length: 100 }).notNull(),
    nameAr: varchar("name_ar", { length: 100 }),
    code: varchar("code", { length: 30 }).unique(),

    // التصنيف الديناميكي (أي نص حر!)
    category: varchar("category", { length: 50 }).notNull(),
    // tuition | registration | bus | uniform | housing | lab | 
    // library | cafeteria | parking | insurance | visa | 
    // research_grant | equipment | conference | publication | ...

    // ربط بدفتر الأستاذ
    revenueAccountId: uuid("revenue_account_id").notNull(),
    receivableAccountId: uuid("receivable_account_id").notNull(),

    // آلية التسعير (6 أنواع)
    pricingType: varchar("pricing_type", { length: 20 }).notNull(),
    // fixed | per_unit | tiered | formula | percentage | custom_script

    pricingConfig: jsonb("pricing_config").$type<{
      // fixed: { amount: 50000 }
      // per_unit: { unitPrice: 1000, unitField: "credit_hours" }
      // tiered: { tiers: [{ max: 10, price: 1000 }, { max: 20, price: 900 }] }
      // formula: { expression: "baseFee + (creditHours * rate) - discount" }
      // percentage: { baseField: "tuitionAmount", percent: 10 }
      // custom_script: { scriptId: "uuid", params: Record<string, unknown> }
    }>().default("{}"),

    // شروط التطبيق الديناميكية
    applicabilityRules: jsonb("applicability_rules").$type<{
      targetEntity: "enrollment" | "student" | "class" | "program" | "department" | "research_project";
      conditions: Array<{
        field: string;
        operator: "eq" | "ne" | "gt" | "lt" | "in" | "not_in" | "between";
        value: unknown;
      }>;
      effectiveDateRange?: { from: string; to: string };
    }>().default("{}"),

    // التكرار
    frequency: varchar("frequency", { length: 20 }).default("one_time").notNull(),
    // one_time | daily | weekly | monthly | termly | quarterly | yearly | 
    // per_semester | per_credit_hour | per_session | custom

    frequencyConfig: jsonb("frequency_config").$type<{
      billingDay?: number;
      prorationEnabled?: boolean;
      advanceBillingDays?: number;
    }>().default("{}"),

    // الرسوم المتأخرة
    lateFeeRules: jsonb("late_fee_rules").$type<{
      type: "fixed" | "percentage" | "tiered" | "daily_compound";
      amount?: number;
      percentage?: number;
      gracePeriodDays: number;
      maxLateFee?: number;
      dailyRate?: number;
    }>().default("{}"),

    // الخصومات المسموحة
    allowedDiscounts: jsonb("allowed_discounts").$type<string[]>().default("[]"),

    // التحكم
    isMandatory: boolean("is_mandatory").default(false).notNull(),
    isActive: boolean("is_active").default(true).notNull(),
    sortOrder: smallint("sort_order").default(0),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgCodeIdx: unique("fee_tpl_org_code_idx").on(t.orgId, t.code),
    orgActiveIdx: index("fee_tpl_org_active_idx").on(t.orgId, t.isActive),
    categoryIdx: index("fee_tpl_category_idx").on(t.orgId, t.category),
    revenueIdx: index("fee_tpl_revenue_idx").on(t.revenueAccountId),
  })
);

// ============================================================
// F4: DISCOUNT TEMPLATES (Dynamic Eligibility Engine)
// ============================================================

export const discountTemplates = financeSchema.table(
  "discount_templates",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    name: varchar("name", { length: 100 }).notNull(),
    code: varchar("code", { length: 30 }).unique(),

    // النوع
    type: varchar("type", { length: 30 }).notNull(),
    // percentage | fixed_amount | sibling | staff | academic_merit | 
    // need_based | early_payment | loyalty | volume | referral | 
    // sibling_cumulative | military | disability | custom_script

    // آلية الحساب
    calculationMethod: varchar("calculation_method", { length: 20 }).notNull(),
    // flat | progressive | formula | conditional

    value: integer("value").notNull(), // نسبة أو مبلغ ثابت

    // شروط التأهل الديناميكية
    eligibilityRules: jsonb("eligibility_rules").$type<{
      academic?: {
        minGpa?: number;
        maxGpa?: number;
        minAttendanceRate?: number;
        minGrade?: string;
        program?: string[];
        level?: string[];
      };
      financial?: {
        familyIncomeMax?: number;
        familyIncomeMin?: number;
        previousPaymentsOnTime?: boolean;
        totalPaidAmountMin?: number;
      };
      family?: {
        siblingCountMin?: number;
        siblingCountMax?: number;
        siblingEnrolled?: boolean;
        parentIsStaff?: boolean;
        parentIsAlumni?: boolean;
      };
      temporal?: {
        earlyPaymentDays?: number;
        registrationBefore?: string;
        validFrom?: string;
        validUntil?: string;
      };
      customExpression?: string; // JS/Python expression
    }>().default("{}"),

    // الحدود
    limits: jsonb("limits").$type<{
      maxDiscountAmount?: number;
      maxRecipients?: number;
      maxPerStudent?: number;
      maxPerFamily?: number;
      applicableFeeTypes?: string[];
    }>().default("{}"),

    // الموافقة
    requiresApproval: boolean("requires_approval").default(false).notNull(),
    approverRoles: jsonb("approver_roles").$type<string[]>().default("[]"),

    isAutoApply: boolean("is_auto_apply").default(false).notNull(),
    isStackable: boolean("is_stackable").default(false).notNull(),
    priority: smallint("priority").default(0),

    isActive: boolean("is_active").default(true).notNull(),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgCodeIdx: unique("disc_tpl_org_code_idx").on(t.orgId, t.code),
    orgTypeIdx: index("disc_tpl_org_type_idx").on(t.orgId, t.type, t.isActive),
    autoApplyIdx: index("disc_tpl_auto_apply_idx")
      .on(t.orgId, t.isAutoApply)
      .where(sql`${t.isAutoApply} = true AND ${t.isActive} = true`),
  })
);

// ============================================================
// F5: PAYMENT PLANS (Flexible Installments)
// ============================================================

export const paymentPlans = financeSchema.table(
  "payment_plans",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    name: varchar("name", { length: 100 }).notNull(),
    nameAr: varchar("name_ar", { length: 100 }),

    // النوع
    type: varchar("type", { length: 30 }).notNull(),
    // standard | installment | deferred | income_based | 
    // scholarship_full | scholarship_partial | work_study | 
    // corporate_sponsored | government_sponsored | custom

    // جدولة الأقساط
    schedule: jsonb("schedule").$type<{
      installments: Array<{
        number: number;
        percent: number;
        dueDateOffsetDays: number;
        minAmount?: number;
        maxAmount?: number;
      }>;
      firstPaymentDate: "immediate" | "registration" | "custom";
      customFirstDate?: string;
    }>().notNull(),

    // شروط التأهل
    eligibilityRules: jsonb("eligibility_rules").$type<{
      minTotalAmount?: number;
      maxTotalAmount?: number;
      allowedFeeTypes?: string[];
      creditScoreMin?: number;
      guarantorRequired?: boolean;
    }>().default("{}"),

    // رسوم إضافية على الخطة
    planFees: jsonb("plan_fees").$type<{
      setupFee?: number;
      processingFeePercent?: number;
      lateFeeMultiplier?: number;
    }>().default("{}"),

    isActive: boolean("is_active").default(true).notNull(),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgTypeIdx: index("plan_org_type_idx").on(t.orgId, t.type, t.isActive),
  })
);

// ============================================================
// F6: STUDENT FEES (Generated Instances)
// ============================================================

export const studentFees = financeSchema.table(
  "student_fees",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    enrollmentId: uuid("enrollment_id").notNull(),
    feeTemplateId: uuid("fee_template_id").notNull(),

    // المبالغ
    originalAmount: moneyType("original_amount").notNull(),
    discountAmount: moneyType("discount_amount").default(0).notNull(),
    lateFeeAmount: moneyType("late_fee_amount").default(0).notNull(),

    // المحسوبة
    totalAmount: moneyType("total_amount").notNull(),
    paidAmount: moneyType("paid_amount").default(0).notNull(),
    balance: moneyType("balance").notNull(),

    // خصومات مفصلة (تتبع كل خصم)
    appliedDiscounts: jsonb("applied_discounts").$type<Array<{
      discountTemplateId: string;
      amount: number;
      reason: string;
    }>>().default("[]"),

    // خطة الدفع المطبقة
    paymentPlanId: uuid("payment_plan_id"),

    status: varchar("status", { length: 20 }).default("pending").notNull(),
    // pending | paid | partial | overdue | waived | cancelled

    dueDate: date("due_date").notNull(),
    isOverdue: boolean("is_overdue").default(false).notNull(),
    overdueSince: date("overdue_since"),
    overdueDays: smallint("overdue_days").default(0),

    // أقساط
    isInstallment: boolean("is_installment").default(false).notNull(),
    parentFeeId: uuid("parent_fee_id"),
    installmentNumber: smallint("installment_number"),
    totalInstallments: smallint("total_installments"),

    // إشعارات
    reminderSent: boolean("reminder_sent").default(false),
    reminderSentAt: timestamp("reminder_sent_at"),
    finalNoticeSent: boolean("final_notice_sent").default(false),
    finalNoticeSentAt: timestamp("final_notice_sent_at"),

    // قفل
    isLocked: boolean("is_locked").default(false).notNull(),
    lockedAt: timestamp("locked_at"),
    lockedBy: uuid("locked_by"),

    notes: text("notes"),
    meta: jsonb("meta").default("{}").$type<Record<string, unknown>>(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    // Unique: one fee per template per enrollment (per installment)
    enrollFeeIdx: unique("sf_enroll_tpl_install_idx").on(
      t.enrollmentId,
      t.feeTemplateId,
      t.installmentNumber
    ),
    enrollIdx: index("sf_enroll_idx").on(t.enrollmentId),
    templateIdx: index("sf_template_idx").on(t.feeTemplateId),

    // Financial queries
    dueIdx: index("sf_due_idx")
      .on(t.cycleId, t.dueDate, t.status)
      .where(sql`${t.status} = 'pending'`),
    overdueIdx: index("sf_overdue_idx")
      .on(t.cycleId, t.isOverdue)
      .where(sql`${t.isOverdue} = true`),
    lockedIdx: index("sf_locked_idx")
      .on(t.cycleId, t.isLocked)
      .where(sql`${t.isLocked} = true`),

    // Reminder queue
    reminderIdx: index("sf_reminder_idx")
      .on(t.cycleId, t.reminderSent, t.dueDate)
      .where(
        sql`${t.reminderSent} = false AND ${t.status} = 'pending' AND ${t.dueDate} <= NOW() + INTERVAL '3 days'`
      ),
  })
);

// ============================================================
// F7: STUDENT DISCOUNT APPLICATIONS
// ============================================================

export const studentDiscounts = financeSchema.table(
  "student_discounts",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    enrollmentId: uuid("enrollment_id").notNull(),
    discountTemplateId: uuid("discount_template_id").notNull(),

    appliedAmount: moneyType("applied_amount").notNull(),
    appliedToFeeId: uuid("applied_to_fee_id"),

    status: varchar("status", { length: 20 }).default("pending").notNull(),
    // pending | approved | rejected | revoked | expired

    reason: text("reason"),
    evidenceUrls: jsonb("evidence_urls").$type<string[]>().default("[]"),

    appliedBy: uuid("applied_by"),
    appliedAt: timestamp("applied_at").defaultNow(),

    approvedBy: uuid("approved_by"),
    approvedAt: timestamp("approved_at"),
    approvalNotes: text("approval_notes"),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    unique: unique("stud_disc_unique_idx").on(t.enrollmentId, t.discountTemplateId),
    statusIdx: index("stud_disc_status_idx").on(t.status),
    pendingIdx: index("stud_disc_pending_idx")
      .on(t.status)
      .where(sql`${t.status} = 'pending'`),
  })
);

// ============================================================
// F8: PAYMENTS
// ============================================================

export const payments = financeSchema.table(
  "payments",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    feeId: uuid("fee_id").notNull(),
    enrollmentId: uuid("enrollment_id").notNull(),

    amount: moneyType("amount").notNull(),
    currency: varchar("currency", { length: 3 }).default("SAR").notNull(),
    exchangeRate: numeric("exchange_rate", { precision: 15, scale: 8 }).default("1"),

    // الطريقة (أي نص حر للمرونة)
    method: varchar("method", { length: 30 }).notNull(),
    // cash | card | bank_transfer | cheque | online | mobile_money | 
    // installment | scholarship | crypto | barter | ...

    // المراجع
    reference: varchar("reference", { length: 100 }),
    transactionId: varchar("transaction_id", { length: 100 }),
    receiptNumber: varchar("receipt_number", { length: 50 }),

    // الربط بالقيود المحاسبية
    journalEntryId: uuid("journal_entry_id"),

    // الخزينة/البنك
    treasuryId: uuid("treasury_id"),

    // معلومات الدافع
    payerName: varchar("payer_name", { length: 100 }),
    payerPhone: varchar("payer_phone", { length: 15 }),
    payerRelation: varchar("payer_relation", { length: 30 }),
    payerId: uuid("payer_id"), // ربط بـ users للدافع المعروف

    // التحقق
    isVerified: boolean("is_verified").default(false).notNull(),
    verifiedBy: uuid("verified_by"),
    verifiedAt: timestamp("verified_at"),

    // الاسترداد
    isRefund: boolean("is_refund").default(false).notNull(),
    refundAmount: moneyType("refund_amount"),
    refundReason: varchar("refund_reason", { length: 200 }),
    refundApprovedBy: uuid("refund_approved_by"),

    // المرفقات
    receiptUrl: varchar("receipt_url", { length: 500 }),
    proofUrl: varchar("proof_url", { length: 500 }),

    // التسوية
    reconciled: boolean("reconciled").default(false),
    reconciledAt: timestamp("reconciled_at"),
    reconciledBy: uuid("reconciled_by"),

    notes: text("notes"),
    meta: jsonb("meta").default("{}").$type<Record<string, unknown>>(),

    paidAt: timestamp("paid_at").notNull(),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    createdBy: uuid("created_by"),
  },
  (t) => ({
    feeIdx: index("pay_fee_idx").on(t.feeId),
    enrollIdx: index("pay_enroll_idx").on(t.enrollmentId),
    journalIdx: index("pay_journal_idx").on(t.journalEntryId),
    verifyIdx: index("pay_verify_idx")
      .on(t.orgId, t.isVerified)
      .where(sql`${t.isVerified} = false`),
    receiptIdx: unique("pay_receipt_idx")
      .on(t.orgId, t.receiptNumber)
      .where(sql`${t.receiptNumber} IS NOT NULL`),
    transactionIdx: index("pay_transaction_idx")
      .on(t.transactionId)
      .where(sql`${t.transactionId} IS NOT NULL`),
    dateIdx: index("pay_date_idx").on(t.orgId, t.paidAt),
    refundIdx: index("pay_refund_idx")
      .on(t.orgId, t.isRefund)
      .where(sql`${t.isRefund} = true`),
  })
);

// ============================================================
// F9: TREASURIES (Bank Accounts & Cash)
// ============================================================

export const treasuries = financeSchema.table(
  "treasuries",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    unitId: uuid("unit_id"),

    name: varchar("name", { length: 100 }).notNull(),
    nameAr: varchar("name_ar", { length: 100 }),
    code: varchar("code", { length: 20 }),

    // النوع (أي نص حر)
    type: varchar("type", { length: 30 }).notNull(),
    // cash | bank | wallet | online_gateway | crypto_wallet | escrow | ...

    // ربط بدفتر الأستاذ
    ledgerAccountId: uuid("ledger_account_id").notNull(),

    // تفاصيل البنك
    bankName: varchar("bank_name", { length: 100 }),
    bankBranch: varchar("bank_branch", { length: 100 }),
    accountNumber: varchar("account_number", { length: 50 }),
    accountName: varchar("account_name", { length: 100 }),
    iban: varchar("iban", { length: 50 }),
    swiftCode: varchar("swift_code", { length: 20 }),

    // الأرصدة
    openingBalance: moneyType("opening_balance").default(0),
    currentBalance: moneyType("current_balance").default(0),
    currency: varchar("currency", { length: 3 }).default("SAR").notNull(),

    // الصلاحيات
    authorizedUsers: jsonb("authorized_users").$type<string[]>().default("[]"),

    isDefault: boolean("is_default").default(false).notNull(),
    isActive: boolean("is_active").default(true).notNull(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgTypeIdx: index("treasury_org_type_idx").on(t.orgId, t.type, t.isActive),
    orgCodeIdx: unique("treasury_org_code_idx").on(t.orgId, t.code),
    ledgerIdx: index("treasury_ledger_idx").on(t.ledgerAccountId),
    defaultIdx: index("treasury_default_idx")
      .on(t.orgId, t.isDefault)
      .where(sql`${t.isDefault} = true`),
  })
);

// ============================================================
// F10: TREASURY TRANSACTIONS (Journal Lines)
// ============================================================

export const treasuryTransactions = financeSchema.table(
  "treasury_transactions",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    treasuryId: uuid("treasury_id").notNull(),

    // ربط بالقيود المحاسبية
    journalEntryId: uuid("journal_entry_id").notNull(),

    type: varchar("type", { length: 20 }).notNull(),
    // income | expense | transfer | adjustment

    category: varchar("category", { length: 30 }).notNull(),
    // fee | salary | utility | transfer | refund | other

    amount: moneyType("amount").notNull(),
    currency: varchar("currency", { length: 3 }).default("SAR").notNull(),
    exchangeRate: numeric("exchange_rate", { precision: 15, scale: 8 }).default("1"),

    // المراجع
    referenceType: varchar("reference_type", { length: 30 }),
    referenceId: uuid("reference_id"),

    description: text("description").notNull(),
    notes: text("notes"),

    attachments: jsonb("attachments").$type<string[]>().default("[]"),

    // Audit
    createdBy: uuid("created_by"),
    approvedBy: uuid("approved_by"),
    approvedAt: timestamp("approved_at"),

    transactionDate: date("transaction_date").notNull(),
    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    treasuryIdx: index("trans_treasury_idx").on(t.treasuryId, t.transactionDate),
    journalIdx: index("trans_journal_idx").on(t.journalEntryId),
    typeIdx: index("trans_type_idx").on(t.orgId, t.type, t.category),
    dateIdx: index("trans_date_idx").on(t.orgId, t.transactionDate),
    refIdx: index("trans_ref_idx").on(t.referenceType, t.referenceId),
    approvedIdx: index("trans_approved_idx")
      .on(t.orgId, t.approvedBy)
      .where(sql`${t.approvedBy} IS NULL`),
  })
);

// ============================================================
// F11: FINANCIAL EVENTS (Event Store for Event Sourcing)
// ============================================================

export const financialEvents = financeSchema.table(
  "financial_events",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    // نوع الحدث
    eventType: varchar("event_type", { length: 50 }).notNull(),
    // fee.generated | fee.due | fee.overdue | payment.received | 
    // payment.failed | discount.applied | discount.expired |
    // refund.requested | refund.processed | account.reconciled |
    // budget.exceeded | forecast.updated | ...

    // الكيان المصدر
    entityType: varchar("entity_type", { length: 30 }).notNull(),
    entityId: uuid("entity_id").notNull(),

    // البيانات (immutable)
    payload: jsonb("payload").notNull(),

    // التسلسل للـ Event Sourcing
    sequenceNumber: serial("sequence_number"),
    aggregateId: uuid("aggregate_id"),

    // المعالجة
    status: varchar("status", { length: 20 }).default("pending").notNull(),
    // pending | processing | completed | failed | retrying

    processedAt: timestamp("processed_at"),
    processedBy: varchar("processed_by", { length: 50 }),
    errorMessage: text("error_message"),
    retryCount: smallint("retry_count").default(0),

    // ربط بالقيود
    journalEntryId: uuid("journal_entry_id"),

    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    entityIdx: index("fin_event_entity_idx").on(t.entityType, t.entityId, t.createdAt),
    typeIdx: index("fin_event_type_idx").on(t.orgId, t.eventType),
    statusIdx: index("fin_event_status_idx")
      .on(t.orgId, t.status)
      .where(sql`${t.status} = 'pending'`),
    sequenceIdx: index("fin_event_sequence_idx").on(t.aggregateId, t.sequenceNumber),
  })
);

// ============================================================
// F12: FINANCIAL REPORTS (Dynamic Reporting)
// ============================================================

export const financialReports = financeSchema.table(
  "financial_reports",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    name: varchar("name", { length: 100 }).notNull(),
    type: varchar("type", { length: 30 }).notNull(),
    // balance_sheet | income_statement | cash_flow | 
    // aging_report | collection_forecast | budget_variance |
    // fee_collection | discount_summary | custom

    // التكوين الديناميكي
    config: jsonb("config").$type<{
      dateRange: { from: string; to: string };
      groupBy?: string[];
      filters?: Record<string, unknown>;
      columns?: Array<{ field: string; label: string; format: string }>;
      chartType?: "table" | "bar" | "line" | "pie" | "pivot";
    }>().notNull(),

    // الجدولة
    isScheduled: boolean("is_scheduled").default(false),
    schedule: jsonb("schedule").$type<{
      frequency: "daily" | "weekly" | "monthly" | "quarterly";
      recipients: string[];
      format: "pdf" | "excel" | "csv";
    }>(),

    lastRunAt: timestamp("last_run_at"),
    lastResult: jsonb("last_result"),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgTypeIdx: index("fin_report_org_type_idx").on(t.orgId, t.type),
    scheduledIdx: index("fin_report_scheduled_idx")
      .on(t.orgId, t.isScheduled)
      .where(sql`${t.isScheduled} = true`),
  })
);
