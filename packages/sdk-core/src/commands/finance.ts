/**
 * Finance command interfaces — mirrors svc-finance/ops.
 */
import type { Payment, StudentFee, FeeTemplate, Treasury, DiscountTemplate, LedgerAccount, JournalEntry, Currency } from "../types";

export interface FinanceCommands {
  // ── Ledger ──────────────────────────────────────────────────
  listLedgerAccounts(orgId: string): Promise<LedgerAccount[]>;
  listTransactionAccounts(): Promise<LedgerAccount[]>;
  createLedgerAccount(data: { name: string; name_ar?: string; code: string; item_type: string; currency: string; parent_id?: string }): Promise<LedgerAccount>;
  getAccountStatement(accountId: string, startDate?: string, endDate?: string): Promise<JournalEntry[]>;

  // ── Treasury ────────────────────────────────────────────────
  listTreasuries(): Promise<Treasury[]>;
  createTreasury(data: { name: string; name_ar?: string; item_type: string; ledger_account_id: string; currency: string }): Promise<Treasury>;

  // ── Fee Templates ───────────────────────────────────────────
  listFeeTemplates(): Promise<FeeTemplate[]>;
  listActiveFeeTemplates(): Promise<FeeTemplate[]>;
  createFeeTemplate(data: { name: string; name_ar?: string; category: string; default_amount: number; revenue_account_id: string; receivable_account_id: string; frequency: string; is_mandatory: boolean }): Promise<FeeTemplate>;
  updateFeeTemplate(templateId: string, data: { name?: string; default_amount?: number; is_active?: boolean }): Promise<FeeTemplate>;
  deleteFeeTemplate(templateId: string): Promise<void>;

  // ── Student Fees ────────────────────────────────────────────
  listStudentFees(enrollmentId: string): Promise<StudentFee[]>;
  assignStudentFee(data: { enrollment_id: string; fee_template_id: string; amount: number; due_date: string; cycle_id: string; notes?: string }): Promise<StudentFee>;

  // ── Payments ────────────────────────────────────────────────
  listPayments(enrollmentId?: string, feeId?: string): Promise<Payment[]>;
  getPayments(orgId: string, studentId?: string): Promise<Payment[]>;
  recordPayment(data: { fee_id?: string; enrollment_id?: string; account_id: string; amount: number; currency: string; exchange_rate: number; method: string; treasury_id?: string; payer_name?: string; notes?: string }): Promise<Payment>;

  // ── Discounts ───────────────────────────────────────────────
  listDiscountTemplates(): Promise<DiscountTemplate[]>;
  createDiscountTemplate(data: { name: string; item_type: string; calculation_method: string; value: number; requires_approval: boolean }): Promise<DiscountTemplate>;

  // ── Currencies ──────────────────────────────────────────────
  listCurrencies(): Promise<Currency[]>;
  createCurrency(data: { code: string; name: string; symbol: string; symbol_position: string; decimal_places: number; exchange_rate: number; is_base: boolean }): Promise<Currency>;

  // ── Journal ─────────────────────────────────────────────────
  listJournalEntries(): Promise<JournalEntry[]>;
}
