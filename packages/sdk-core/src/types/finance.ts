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
  year_id: string;
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
  item_type: string;
  currency: string;
  current_balance?: number | null;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface JournalEntry {
  id: string;
  org_id: string;
  transaction_ref: string;
  accounting_date: string;
  amount: number;
  debit_account_id: string;
  credit_account_id: string;
  currency: string;
  description: string;
  status: string;
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
