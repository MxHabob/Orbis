-- ── PRODUCTION SCHOOL ACCOUNTING COA & TREASURIES SEED ──
-- This script registers the complete, clean accounting structure (Chart of Accounts)
-- and core treasury boxes (safes & bank accounts) for a school organization.
-- Contains ZERO dummy/mock transaction data.
--
-- Target Organization ID: 'default-org-id'

-- 1. CLEANUP PREVIOUS ENTRIES FOR THE TARGET ORG
DELETE FROM treasuries WHERE org_id = 'default-org-id';
DELETE FROM ledger_accounts WHERE org_id = 'default-org-id';

-- 2. CHART OF ACCOUNTS (LEDGER ACCOUNTS)
-- Grouped by Account Types: asset, liability, equity, revenue, expense

-- 2.1 ASSET ACCOUNTS (11000 - 19999)
INSERT INTO ledger_accounts (id, org_id, code, name, name_ar, type, sub_type, parent_id, path, depth, currency, opening_balance, current_balance, is_bank_account, is_cash_account, is_control_account, external_ref, is_active, created_at, updated_at) VALUES 
('ledger-1101-safe-main', 'default-org-id', '1101', 'School Main Safe Cashbox', 'حساب صندوق النقدية الرئيسي للمدرسة', 'asset', 'cash', NULL, '1101', 1, 'YER', 0.0, 0.0, 0, 1, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-1102-safe-petty', 'default-org-id', '1102', 'Petty Cash Safe Box', 'حساب صندوق العهدة والنثرية', 'asset', 'cash', NULL, '1102', 1, 'YER', 0.0, 0.0, 0, 1, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-1201-bank-kuraimi', 'default-org-id', '1201', 'Kuraimi Current Corporate Bank', 'حساب بنك الكريمي الجاري للمدرسة', 'asset', 'bank', NULL, '1201', 1, 'YER', 0.0, 0.0, 1, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-1202-bank-tadhamon', 'default-org-id', '1202', 'Tadhamon School Operating Account', 'حساب بنك التضامن التشغيلي للمدرسة', 'asset', 'bank', NULL, '1202', 1, 'YER', 0.0, 0.0, 1, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-1203-bank-usd', 'default-org-id', '1203', 'Kuraimi Corporate USD Account', 'حساب بنك الكريمي بالدولار الأمريكي', 'asset', 'bank', NULL, '1203', 1, 'USD', 0.0, 0.0, 1, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-1204-bank-sar', 'default-org-id', '1204', 'Tadhamon Corporate SAR Account', 'حساب بنك التضامن بالريال السعودي', 'asset', 'bank', NULL, '1204', 1, 'SAR', 0.0, 0.0, 1, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-1301-ar-tuition', 'default-org-id', '1301', 'Student Tuition Receivables', 'حساب ذمم الرسوم الدراسية المستحقة للطلاب', 'asset', 'receivable', NULL, '1301', 1, 'YER', 0.0, 0.0, 0, 0, 1, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-1302-ar-bus', 'default-org-id', '1302', 'Student Bus service Receivables', 'حساب ذمم النقل المدرسي للطلاب', 'asset', 'receivable', NULL, '1302', 1, 'YER', 0.0, 0.0, 0, 0, 1, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00');

-- 2.2 LIABILITY ACCOUNTS (21000 - 29999)
INSERT INTO ledger_accounts (id, org_id, code, name, name_ar, type, sub_type, parent_id, path, depth, currency, opening_balance, current_balance, is_bank_account, is_cash_account, is_control_account, external_ref, is_active, created_at, updated_at) VALUES 
('ledger-2101-liab-deferred', 'default-org-id', '2101', 'Deferred Tuition Revenue', 'إيرادات الرسوم الدراسية المؤجلة / غير المكتسبة', 'liability', 'deferred_revenue', NULL, '2101', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-2201-liab-ap', 'default-org-id', '2201', 'Accounts Payable (Suppliers)', 'حساب ذمم الدائنين والموردين', 'liability', 'payable', NULL, '2201', 1, 'YER', 0.0, 0.0, 0, 0, 1, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-2301-liab-wages', 'default-org-id', '2301', 'Accrued Salaries & Wages', 'مصروف الرواتب والأجور المستحقة', 'liability', 'accrued_expense', NULL, '2301', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00');

-- 2.3 EQUITY ACCOUNTS (31000 - 39999)
INSERT INTO ledger_accounts (id, org_id, code, name, name_ar, type, sub_type, parent_id, path, depth, currency, opening_balance, current_balance, is_bank_account, is_cash_account, is_control_account, external_ref, is_active, created_at, updated_at) VALUES 
('ledger-3101-eq-capital', 'default-org-id', '3101', 'Shareholder Paid-in Capital', 'رأس مال المدرسة المدفوع والتأسيسي', 'equity', 'capital', NULL, '3101', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-3201-eq-retained', 'default-org-id', '3201', 'Retained Earnings', 'حساب الأرباح والخسائر المبقاة والمدورة', 'equity', 'retained_earnings', NULL, '3201', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00');

-- 2.4 REVENUE ACCOUNTS (41000 - 49999)
INSERT INTO ledger_accounts (id, org_id, code, name, name_ar, type, sub_type, parent_id, path, depth, currency, opening_balance, current_balance, is_bank_account, is_cash_account, is_control_account, external_ref, is_active, created_at, updated_at) VALUES 
('ledger-4101-rev-kg', 'default-org-id', '4101', 'Kindergarten Tuition Income', 'إيرادات الرسوم الدراسية - رياض الأطفال', 'revenue', 'income', NULL, '4101', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-4102-rev-primary', 'default-org-id', '4102', 'Primary School Tuition Income', 'إيرادات الرسوم الدراسية - المرحلة الابتدائية', 'revenue', 'income', NULL, '4102', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-4103-rev-middle', 'default-org-id', '4103', 'Middle School Tuition Income', 'إيرادات الرسوم الدراسية - المرحلة المتوسطة', 'revenue', 'income', NULL, '4103', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-4104-rev-high', 'default-org-id', '4104', 'High School Tuition Income', 'إيرادات الرسوم الدراسية - المرحلة الثانوية', 'revenue', 'income', NULL, '4104', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-4201-rev-bus', 'default-org-id', '4201', 'School Bus Services Income', 'إيرادات خدمة النقل والمواصلات المدرسية', 'revenue', 'income', NULL, '4201', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-4202-rev-uniform', 'default-org-id', '4202', 'Books and Uniform Sales Revenue', 'إيرادات بيع الكتب والزي المدرسي', 'revenue', 'income', NULL, '4202', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-4301-rev-activities', 'default-org-id', '4301', 'Activities, Laboratory & IT Fees', 'إيرادات الأنشطة الطلابية والمعامل وتقنية المعلومات', 'revenue', 'income', NULL, '4301', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00');

-- 2.5 EXPENSE ACCOUNTS (51000 - 59999)
INSERT INTO ledger_accounts (id, org_id, code, name, name_ar, type, sub_type, parent_id, path, depth, currency, opening_balance, current_balance, is_bank_account, is_cash_account, is_control_account, external_ref, is_active, created_at, updated_at) VALUES 
('ledger-5101-exp-salaries-aca', 'default-org-id', '5101', 'Academic Salaries Expense', 'مصروف رواتب الكادر التعليمي والتدريس', 'expense', 'cost_of_sales', NULL, '5101', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-5102-exp-salaries-adm', 'default-org-id', '5102', 'Administrative & Service wages', 'مصروف رواتب الإداريين والحراس والصيانة', 'expense', 'cost_of_sales', NULL, '5102', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-5201-exp-rent', 'default-org-id', '5201', 'Premises Rent Expense', 'مصروف إيجار مبنى المدرسة والمنشآت', 'expense', 'overhead', NULL, '5201', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-5202-exp-utilities', 'default-org-id', '5202', 'Utilities: Electricity & Water', 'مصروف كهرباء ومياه مبنى المدرسة', 'expense', 'overhead', NULL, '5202', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-5203-exp-erp', 'default-org-id', '5203', 'Software Licenses & ERP system', 'مصروف تراخيص البرمجيات والأنظمة والإنترنت', 'expense', 'overhead', NULL, '5203', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-5301-exp-maintenance', 'default-org-id', '5301', 'Facility Maintenance & Supplies', 'مصروف صيانة المبنى والتكييف والمستلزمات', 'expense', 'overhead', NULL, '5301', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('ledger-5401-exp-marketing', 'default-org-id', '5401', 'Marketing, Admission & PR campaigns', 'مصروف الحملات الإعلانية والتسويق والتسجيل', 'expense', 'marketing', NULL, '5401', 1, 'YER', 0.0, 0.0, 0, 0, 0, NULL, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00');

-- 3. CORE TREASURY BOXES
-- Establish cashboxes and banks mapped 1:1 to their corresponding ledger asset accounts.
INSERT INTO treasuries (id, org_id, unit_id, name, name_ar, code, "type", ledger_account_id, bank_name, bank_branch, account_number, account_name, iban, swift_code, opening_balance, current_balance, currency, authorized_users, is_default, is_active, created_at, updated_at) VALUES 
('treasury-1-safe-main', 'default-org-id', NULL, 'School Main Safe Cashbox', 'الصندوق الرئيسي للمدرسة', 'SAFE-MAIN', 'cash', 'ledger-1101-safe-main', NULL, NULL, NULL, NULL, NULL, NULL, 0.0, 0.0, 'YER', '["admin"]', 1, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('treasury-2-safe-petty', 'default-org-id', NULL, 'Petty Cash safe box', 'صندوق العهدة والمصروفات النثرية', 'SAFE-PETTY', 'cash', 'ledger-1102-safe-petty', NULL, NULL, NULL, NULL, NULL, NULL, 0.0, 0.0, 'YER', '["accountant"]', 0, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('treasury-3-bank-kuraimi', 'default-org-id', NULL, 'Kuraimi Current Corporate Account', 'حساب شركة المدرسة - بنك الكريمي', 'BANK-MAIN', 'bank', 'ledger-1201-bank-kuraimi', 'Kuraimi Bank', 'Sanaa Branch', '123456789', 'School Corporate Account', 'YE123456789', 'KRMIBYE', 0.0, 0.0, 'YER', '["admin", "owner"]', 0, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('treasury-4-bank-tadhamon', 'default-org-id', NULL, 'Tadhamon School Operating Account', 'حساب بنك التضامن التشغيلي للمدرسة', 'BANK-OPER', 'bank', 'ledger-1202-bank-tadhamon', 'Tadhamon Bank', 'Sanaa Branch', '987654321', 'School Operating Ledger', 'YE987654321', 'TIBKBYE', 0.0, 0.0, 'YER', '["accountant"]', 0, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('treasury-5-bank-usd', 'default-org-id', NULL, 'Kuraimi USD Corporate Account', 'حساب بنك الكريمي بالدولار الأمريكي', 'BANK-USD', 'bank', 'ledger-1203-bank-usd', 'Kuraimi Bank', 'Sanaa Branch', '1122334455', 'School USD Account', 'YE1122334455', 'KRMIBYE', 0.0, 0.0, 'USD', '["admin", "owner"]', 0, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00'),
('treasury-6-bank-sar', 'default-org-id', NULL, 'Tadhamon SAR Operating Account', 'حساب بنك التضامن بالريال السعودي', 'BANK-SAR', 'bank', 'ledger-1204-bank-sar', 'Tadhamon Bank', 'Sanaa Branch', '5544332211', 'School SAR Ledger', 'YE5544332211', 'TIBKBYE', 0.0, 0.0, 'SAR', '["accountant"]', 0, 1, '2026-06-21 12:00:00', '2026-06-21 12:00:00');
