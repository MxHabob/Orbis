/**
 * Finance command interfaces — mirrors svc-finance/ops.
 */
import type {
  Payment,
  StudentFee,
  FeeTemplate,
  Treasury,
  DiscountTemplate,
  Account,
  JournalEntry,
  Currency,
  FiscalYear,
} from "../types";

export interface FinanceCommands {
  // ── Ledger ──────────────────────────────────────────────────
  listAccounts(orgId: string): Promise<Account[]>;
  listTransactionAccounts(orgId: string): Promise<Account[]>;
  createAccount(data: {
    name: string;
    name_ar?: string;
    code: string;
    item_type: string;
    currency: string;
    parent_id?: string;
  }): Promise<Account>;
  getAccountStatement(
    accountId: string,
    startDate?: string,
    endDate?: string,
  ): Promise<JournalEntry[]>;

  // ── Treasury ────────────────────────────────────────────────
  listTreasuries(orgId: string): Promise<Treasury[]>;
  createTreasury(data: {
    name: string;
    name_ar?: string;
    item_type: string;
    ledger_account_id: string;
    currency: string;
  }): Promise<Treasury>;

  // ── Fee Templates ───────────────────────────────────────────
  listFeeTemplates(orgId: string): Promise<FeeTemplate[]>;
  listActiveFeeTemplates(orgId: string): Promise<FeeTemplate[]>;
  createFeeTemplate(data: {
    name: string;
    name_ar?: string;
    category: string;
    default_amount: number;
    revenue_account_id: string;
    receivable_account_id: string;
    frequency: string;
    is_mandatory: boolean;
  }): Promise<FeeTemplate>;
  updateFeeTemplate(
    templateId: string,
    data: { name?: string; default_amount?: number; is_active?: boolean },
  ): Promise<FeeTemplate>;
  deleteFeeTemplate(templateId: string): Promise<void>;

  // ── Student Fees ────────────────────────────────────────────
  listStudentFees(enrollmentId: string): Promise<StudentFee[]>;
  listAllStudentFees(orgId: string): Promise<StudentFee[]>;
  assignStudentFee(data: {
    enrollment_id: string;
    fee_template_id: string;
    amount: number;
    due_date: string;
    period_id: string;
    notes?: string;
  }): Promise<StudentFee>;

  // ── Payments ────────────────────────────────────────────────
  listPayments(orgId: string): Promise<Payment[]>;
  recordPayment(data: {
    fee_id?: string;
    enrollment_id?: string;
    account_id: string;
    amount: number;
    currency: string;
    exchange_rate: number;
    method: string;
    treasury_id?: string;
    payer_name?: string;
    notes?: string;
  }): Promise<Payment>;

  // ── Discounts ───────────────────────────────────────────────
  listDiscountTemplates(orgId: string): Promise<DiscountTemplate[]>;
  createDiscountTemplate(data: {
    name: string;
    item_type: string;
    calculation_method: string;
    value: number;
    requires_approval: boolean;
  }): Promise<DiscountTemplate>;

  // ── Currencies ──────────────────────────────────────────────
  listCurrencies(orgId: string): Promise<Currency[]>;
  createCurrency(orgId: string, data: {
    code: string;
    name: string;
    name_ar?: string;
    symbol: string;
    symbol_position: string;
    decimal_places: number;
    exchange_rate: number;
    is_base: boolean | number;
  }): Promise<Currency>;
  updateCurrency(
    orgId: string,
    code: string,
    data: { exchange_rate?: number; is_active?: boolean },
  ): Promise<Currency>;
  deleteCurrency(orgId: string, code: string): Promise<void>;
  seedDefaultFinancials(orgId: string): Promise<void>;

  // ── Journal ─────────────────────────────────────────────────
  listJournalEntries(orgId: string): Promise<JournalEntry[]>;

  // ── Fiscal Years ────────────────────────────────────────────
  listFiscalYears(orgId: string): Promise<FiscalYear[]>;
  createFiscalYear(orgId: string, data?: {
    name: string;
    start_date: string;
    end_date: string;
    status?: string;
  }): Promise<FiscalYear>;
}
