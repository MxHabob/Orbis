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
  listAccounts(): Promise<Account[]>;
  listTransactionAccounts(): Promise<Account[]>;
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
  listTreasuries(): Promise<Treasury[]>;
  createTreasury(data: {
    name: string;
    name_ar?: string;
    item_type: string;
    ledger_account_id: string;
    currency: string;
  }): Promise<Treasury>;

  // ── Fee Templates ───────────────────────────────────────────
  listFeeTemplates(): Promise<FeeTemplate[]>;
  listActiveFeeTemplates(): Promise<FeeTemplate[]>;
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
  listAllStudentFees(): Promise<StudentFee[]>;
  assignStudentFee(data: {
    enrollment_id: string;
    fee_template_id: string;
    amount: number;
    due_date: string;
    period_id: string;
    notes?: string;
  }): Promise<StudentFee>;

  // ── Payments ────────────────────────────────────────────────
  listPayments(): Promise<Payment[]>;
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
  listDiscountTemplates(): Promise<DiscountTemplate[]>;
  updateDiscountTemplate(id: string, data: any): Promise<DiscountTemplate>;
  createDiscountTemplate(data: {
    name: string;
    item_type: string;
    calculation_method: string;
    value: number;
    requires_approval: boolean;
  }): Promise<DiscountTemplate>;

  // ── Currencies ──────────────────────────────────────────────
  listCurrencies(): Promise<Currency[]>;
  createCurrency(data: {
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
    code: string,
    data: { exchange_rate?: number; is_active?: boolean },
  ): Promise<Currency>;
  deleteCurrency(code: string): Promise<void>;
  seedDefaultFinancials(): Promise<void>;

  // ── Journal ─────────────────────────────────────────────────
  listJournalEntries(): Promise<JournalEntry[]>;

  // ── Fiscal Years ────────────────────────────────────────────
  listFiscalYears(): Promise<FiscalYear[]>;
  createFiscalYear(data?: {
    name: string;
    start_date: string;
    end_date: string;
    status?: string;
  }): Promise<FiscalYear>;
}
