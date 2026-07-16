/** Finance domain types. */

export interface Payment {
  id: string;
  org_id: string;
  fee_id: string;
  enrollment_id: string;
  amount: number;
  currency: string;
  exchange_rate?: number;
  method: string;
  reference?: string;
  transaction_id?: string;
  receipt_number?: string;
  journal_entry_id?: string;
  treasury_id?: string;
  payer_name?: string;
  payer_phone?: string;
  payer_relation?: string;
  payer_id?: string;
  is_verified: boolean;
  verified_by?: string;
  verified_at?: string;
  is_refund: boolean;
  refund_amount?: number;
  refund_reason?: string;
  refund_approved_by?: string;
  receipt_url?: string;
  proof_url?: string;
  reconciled?: boolean;
  reconciled_at?: string;
  reconciled_by?: string;
  notes?: string;
  meta?: Record<string, any>;
  paid_at: string;
  created_at: string;
  created_by?: string;
}

export interface StudentFee {
  id: string;
  org_id: string;
  cycle_id: string;
  enrollment_id: string;
  fee_template_id: string;
  original_amount: number;
  discount_amount: number;
  late_fee_amount: number;
  total_amount: number;
  paid_amount: number;
  balance: number;
  applied_discounts?: Record<string, any>;
  payment_plan_id?: string;
  status: string;
  due_date: string;
  is_overdue: boolean;
  overdue_since?: string;
  overdue_days?: number;
  is_installment: boolean;
  parent_fee_id?: string;
  installment_number?: number;
  total_installments?: number;
  reminder_sent?: boolean;
  reminder_sent_at?: string;
  final_notice_sent?: boolean;
  final_notice_sent_at?: string;
  is_locked: boolean;
  locked_at?: string;
  locked_by?: string;
  notes?: string;
  meta?: Record<string, any>;
  created_at: string;
  updated_at: string;
}

export interface FeeTemplate {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string;
  code?: string;
  category: string;
  revenue_account_id: string;
  receivable_account_id: string;
  pricing_type: string;
  pricing_config?: Record<string, any>;
  applicability_rules?: Record<string, any>;
  frequency: string;
  frequency_config?: Record<string, any>;
  late_fee_rules?: Record<string, any>;
  allowed_discounts?: Record<string, any>;
  is_mandatory: boolean;
  is_active: boolean;
  sort_order?: number;
  created_at: string;
  updated_at: string;
}

export interface Treasury {
  id: string;
  org_id: string;
  unit_id?: string;
  name: string;
  name_ar?: string;
  code?: string;
  type: string;
  ledger_account_id: string;
  bank_name?: string;
  bank_branch?: string;
  account_number?: string;
  account_name?: string;
  iban?: string;
  swift_code?: string;
  opening_balance?: number;
  current_balance?: number;
  currency: string;
  authorized_users?: Record<string, any>;
  is_default: boolean;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface DiscountTemplate {
  id: string;
  org_id: string;
  name: string;
  code?: string;
  type: string;
  calculation_method: string;
  value: number;
  eligibility_rules?: Record<string, any>;
  limits?: Record<string, any>;
  requires_approval: boolean;
  approver_roles?: Record<string, any>;
  is_auto_apply: boolean;
  is_stackable: boolean;
  priority?: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface Account {
  id: string;
  org_id: string;
  code: string;
  name: string;
  name_ar?: string | null;
  type: string;
  sub_type?: string | null;
  parent_id?: string | null;
  path: string;
  depth: number;
  currency: string;
  balances: Record<string, number>;
  entity_type?: string | null;
  entity_id?: string | null;
  is_control_account: number;
  is_system_account: number;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface JournalEntry {
  id: string;
  org_id: string;
  transaction_ref: string;
  accounting_date: string;
  fiscal_period_id: string;
  amount: number;
  debit_account_id: string;
  credit_account_id: string;
  currency: string;
  exchange_rate?: number;
  base_currency_amount?: number;
  source_type: string;
  source_id?: string;
  source_module?: string;
  description: string;
  description_ar?: string;
  metadata?: Record<string, any>;
  is_reversed: boolean;
  reversed_by?: string;
  reversal_reason?: string;
  is_reconciled: boolean;
  reconciled_at?: string;
  reconciliation_ref?: string;
  status: string;
  posted_by?: string;
  posted_at?: string;
  approved_by?: string;
  approved_at?: string;
  created_by?: string;
  created_at: string;
}

export interface Currency {
  code: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  symbol: string;
  exchange_rate: number;
  is_base: number;
  is_active: number;
}

export interface FiscalYear {
  id: string;
  org_id: string;
  name: string;
  start_date: string;
  end_date: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface FiscalPeriod {
  id: string;
  org_id: string;
  fiscal_period_id: string;
  name: string;
  start_date: string;
  end_date: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface FinancialReport {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string;
  type: string;
  start_date: string;
  end_date: string;
  summary?: Record<string, any>;
  details?: Record<string, any>;
  status: string;
  generated_by?: string;
  generated_at?: string;
  created_at: string;
}
