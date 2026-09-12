use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. Organizations
        let _ = db.execute_unprepared("ALTER TABLE organizations ADD COLUMN is_hijri_enabled BOOLEAN NOT NULL DEFAULT 1;").await;
        let _ = db.execute_unprepared("ALTER TABLE organizations ADD COLUMN base_currency_code TEXT;").await;
        let _ = db.execute_unprepared("UPDATE organizations SET is_hijri_enabled = hijri_enabled WHERE hijri_enabled IS NOT NULL;").await;
        let _ = db.execute_unprepared("UPDATE organizations SET base_currency_code = base_currency_id WHERE base_currency_id IS NOT NULL AND base_currency_code IS NULL;").await;

        // 2. Users
        let _ = db.execute_unprepared("ALTER TABLE users ADD COLUMN is_email_verified BOOLEAN NOT NULL DEFAULT 0;").await;
        let _ = db.execute_unprepared("ALTER TABLE users ADD COLUMN is_phone_verified BOOLEAN NOT NULL DEFAULT 0;").await;
        let _ = db.execute_unprepared("ALTER TABLE users ADD COLUMN is_mfa_enabled BOOLEAN NOT NULL DEFAULT 0;").await;
        let _ = db.execute_unprepared("ALTER TABLE users ADD COLUMN failed_login_attempts INTEGER NOT NULL DEFAULT 0;").await;
        let _ = db.execute_unprepared("UPDATE users SET is_email_verified = email_verified WHERE email_verified IS NOT NULL;").await;
        let _ = db.execute_unprepared("UPDATE users SET is_phone_verified = phone_verified WHERE phone_verified IS NOT NULL;").await;
        let _ = db.execute_unprepared("UPDATE users SET is_mfa_enabled = mfa_enabled WHERE mfa_enabled IS NOT NULL;").await;
        let _ = db.execute_unprepared("UPDATE users SET failed_login_attempts = failed_logins WHERE failed_logins IS NOT NULL;").await;

        // 3. User Sessions
        let _ = db.execute_unprepared("ALTER TABLE user_sessions ADD COLUMN is_mfa_verified BOOLEAN NOT NULL DEFAULT 0;").await;
        let _ = db.execute_unprepared("UPDATE user_sessions SET is_mfa_verified = mfa_verified WHERE mfa_verified IS NOT NULL;").await;

        // 4. Persons
        let _ = db.execute_unprepared("ALTER TABLE persons ADD COLUMN middle_name TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE persons ADD COLUMN first_name_ar TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE persons ADD COLUMN middle_name_ar TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE persons ADD COLUMN last_name_ar TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE persons ADD COLUMN display_name TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE persons ADD COLUMN birth_date TEXT;").await;
        let _ = db.execute_unprepared("UPDATE persons SET birth_date = date_of_birth WHERE date_of_birth IS NOT NULL AND birth_date IS NULL;").await;

        // 5. Enrollments
        let _ = db.execute_unprepared("ALTER TABLE enrollments ADD COLUMN enrolled_at DATETIME;").await;
        let _ = db.execute_unprepared("ALTER TABLE enrollments ADD COLUMN withdrawn_at DATETIME;").await;
        let _ = db.execute_unprepared("UPDATE enrollments SET enrolled_at = enrollment_date WHERE enrollment_date IS NOT NULL AND enrolled_at IS NULL;").await;
        let _ = db.execute_unprepared("UPDATE enrollments SET withdrawn_at = withdrawal_date WHERE withdrawal_date IS NOT NULL AND withdrawn_at IS NULL;").await;

        // 6. Subjects
        let _ = db.execute_unprepared("ALTER TABLE subjects ADD COLUMN theory_hours DECIMAL(5,2);").await;
        let _ = db.execute_unprepared("ALTER TABLE subjects ADD COLUMN practical_hours DECIMAL(5,2);").await;
        let _ = db.execute_unprepared("ALTER TABLE subjects ADD COLUMN is_lab_required BOOLEAN NOT NULL DEFAULT 0;").await;
        let _ = db.execute_unprepared("UPDATE subjects SET is_lab_required = requires_lab WHERE requires_lab IS NOT NULL;").await;

        // 7. Academic Periods
        let _ = db.execute_unprepared("ALTER TABLE academic_periods ADD COLUMN enrollment_starts_at DATETIME;").await;
        let _ = db.execute_unprepared("ALTER TABLE academic_periods ADD COLUMN enrollment_ends_at DATETIME;").await;
        let _ = db.execute_unprepared("ALTER TABLE academic_periods ADD COLUMN grade_entry_starts_at DATETIME;").await;
        let _ = db.execute_unprepared("ALTER TABLE academic_periods ADD COLUMN grade_entry_ends_at DATETIME;").await;
        let _ = db.execute_unprepared("ALTER TABLE academic_periods ADD COLUMN exam_starts_at DATETIME;").await;
        let _ = db.execute_unprepared("ALTER TABLE academic_periods ADD COLUMN exam_ends_at DATETIME;").await;
        let _ = db.execute_unprepared("UPDATE academic_periods SET enrollment_starts_at = enrollment_start WHERE enrollment_start IS NOT NULL AND enrollment_starts_at IS NULL;").await;
        let _ = db.execute_unprepared("UPDATE academic_periods SET enrollment_ends_at = enrollment_end WHERE enrollment_end IS NOT NULL AND enrollment_ends_at IS NULL;").await;
        let _ = db.execute_unprepared("UPDATE academic_periods SET grade_entry_starts_at = grade_entry_start WHERE grade_entry_start IS NOT NULL AND grade_entry_starts_at IS NULL;").await;
        let _ = db.execute_unprepared("UPDATE academic_periods SET grade_entry_ends_at = grade_entry_end WHERE grade_entry_end IS NOT NULL AND grade_entry_ends_at IS NULL;").await;
        let _ = db.execute_unprepared("UPDATE academic_periods SET exam_starts_at = exam_start WHERE exam_start IS NOT NULL AND exam_starts_at IS NULL;").await;
        let _ = db.execute_unprepared("UPDATE academic_periods SET exam_ends_at = exam_end WHERE exam_end IS NOT NULL AND exam_ends_at IS NULL;").await;

        // 8. Grades
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN org_id BLOB;").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN period_id BLOB;").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN enrollment_id BLOB;").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN component_id BLOB;").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN max_score DECIMAL(5,2);").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN weight DECIMAL(5,2);").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN letter_grade TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN is_final BOOLEAN;").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN is_exempt BOOLEAN;").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN change_reason TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE grades ADD COLUMN recorded_by BLOB;").await;

        // 9. Attendances
        let _ = db.execute_unprepared("ALTER TABLE attendances ADD COLUMN attendance_date DATE;").await;
        let _ = db.execute_unprepared("UPDATE attendances SET attendance_date = date WHERE date IS NOT NULL AND attendance_date IS NULL;").await;

        // 10. Accounts
        let _ = db.execute_unprepared("ALTER TABLE accounts ADD COLUMN currency_code TEXT;").await;
        let _ = db.execute_unprepared("UPDATE accounts SET currency_code = currency WHERE currency IS NOT NULL AND currency_code IS NULL;").await;

        // 11. Journal Entries
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN reference_number TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN currency_code TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN description_ar TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN source_type TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN source_id BLOB;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN status TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN is_reversed BOOLEAN;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN reversed_by BLOB;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN reversal_reason TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entries ADD COLUMN created_by BLOB;").await;
        let _ = db.execute_unprepared("UPDATE journal_entries SET reference_number = transaction_ref WHERE transaction_ref IS NOT NULL AND reference_number IS NULL;").await;
        let _ = db.execute_unprepared("UPDATE journal_entries SET currency_code = currency_id WHERE currency_id IS NOT NULL AND currency_code IS NULL;").await;

        // 12. Journal Entry Lines
        let _ = db.execute_unprepared("ALTER TABLE journal_entry_lines ADD COLUMN debit_amount DECIMAL(16,4) NOT NULL DEFAULT 0;").await;
        let _ = db.execute_unprepared("ALTER TABLE journal_entry_lines ADD COLUMN credit_amount DECIMAL(16,4) NOT NULL DEFAULT 0;").await;
        let _ = db.execute_unprepared("UPDATE journal_entry_lines SET debit_amount = debit WHERE debit IS NOT NULL;").await;
        let _ = db.execute_unprepared("UPDATE journal_entry_lines SET credit_amount = credit WHERE credit IS NOT NULL;").await;

        // 13. Fee Templates
        let _ = db.execute_unprepared("ALTER TABLE fee_templates ADD COLUMN code TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE fee_templates ADD COLUMN name_ar TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE fee_templates ADD COLUMN category TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE fee_templates ADD COLUMN currency_code TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE fee_templates ADD COLUMN applicable_levels JSON;").await;
        let _ = db.execute_unprepared("ALTER TABLE fee_templates ADD COLUMN applicable_study_modes JSON;").await;
        let _ = db.execute_unprepared("ALTER TABLE fee_templates ADD COLUMN frequency TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE fee_templates ADD COLUMN is_active BOOLEAN;").await;

        // 14. Student Fees
        let _ = db.execute_unprepared("ALTER TABLE student_fees ADD COLUMN period_id BLOB;").await;
        let _ = db.execute_unprepared("ALTER TABLE student_fees ADD COLUMN paid_amount DECIMAL(12,2);").await;
        let _ = db.execute_unprepared("ALTER TABLE student_fees ADD COLUMN balance_due DECIMAL(12,2);").await;
        let _ = db.execute_unprepared("ALTER TABLE student_fees ADD COLUMN discount_details JSON;").await;
        let _ = db.execute_unprepared("ALTER TABLE student_fees ADD COLUMN installment_number INTEGER;").await;
        let _ = db.execute_unprepared("ALTER TABLE student_fees ADD COLUMN notes TEXT;").await;

        // 15. Payments
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN enrollment_id BLOB;").await;
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN student_id BLOB;").await;
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN currency_code TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN exchange_rate DECIMAL(12,6);").await;
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN base_amount DECIMAL(12,2);").await;
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN receipt_number TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN payer_name TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN payer_phone TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN notes TEXT;").await;
        let _ = db.execute_unprepared("ALTER TABLE payments ADD COLUMN created_by BLOB;").await;

        // 16. Discount Templates
        let _ = db.execute_unprepared("ALTER TABLE discount_templates ADD COLUMN is_approval_required BOOLEAN NOT NULL DEFAULT 0;").await;
        let _ = db.execute_unprepared("UPDATE discount_templates SET is_approval_required = requires_approval WHERE requires_approval IS NOT NULL;").await;

        // 17. Autopilot Tasks
        let _ = db.execute_unprepared("CREATE TABLE IF NOT EXISTS autopilot_tasks (
            id BLOB PRIMARY KEY NOT NULL,
            org_id BLOB NOT NULL,
            task_type TEXT NOT NULL,
            status TEXT NOT NULL,
            parameters JSON,
            scheduled_at DATETIME NOT NULL,
            executed_at DATETIME,
            error_log TEXT,
            FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
        );").await;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
