/** Finance domain types. */

export interface Payment {
  id: string;
  org_id: string;
  fee_id?: string | null;
  enrollment_id?: string | null;
  account_id: string;
  amount: number;
  currency: string;
  exchange_rate: number;
  base_amount: number;
  method: string;
  receipt_number?: string | null;
  treasury_id?: string | null;
  payer_name?: string | null;
  payment_type: string;
  notes?: string | null;
  paid_at: string;
  created_at: string;
}

export interface StudentFee {
  id: string;
  org_id: string;
  period_id: string;
  enrollment_id: string;
  fee_template_id: string;
  original_amount: number;
  discount_amount: number;
  net_amount: number;
  status: string;
  due_date: string;
  notes?: string | null;
  created_at: string;
  updated_at: string;
}

export interface FeeTemplate {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  category: string;
  default_amount: number;
  frequency: string;
  is_mandatory: number;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface Treasury {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  item_type: string;
  current_balance?: number | null;
  currency: string;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface DiscountTemplate {
  id: string;
  org_id: string;
  name: string;
  item_type: string;
  calculation_method: string;
  value: number;
  is_active: number;
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
  exchange_rate?: number | null;
  base_currency_amount?: number | null;
  source_type: string;
  source_id?: string | null;
  source_module?: string | null;
  description: string;
  description_ar?: string | null;
  metadata?: string | null;
  is_reversed: number;
  reversed_by?: string | null;
  reversal_reason?: string | null;
  is_reconciled: number;
  reconciled_at?: string | null;
  reconciliation_ref?: string | null;
  status: string;
  posted_by?: string | null;
  posted_at?: string | null;
  approved_by?: string | null;
  approved_at?: string | null;
  created_by?: string | null;
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
