use sea_orm_migration::prelude::*;

use crate::models::{
    academic_calendar, academic_period, account, assessment_scheme, attendance, autopilot_task,
    behavior_record, currency, custom_field_definition, discount_template, document,
    document_requirement, employee, enrollment, exam_proctor, exam_schedule, exam_session,
    exchange_rate, fee_template, folder, gpa_record, grade, group, group_permission,
    integration::{integration_config, integration_secret},
    journal_entry, journal_entry_line, mfa_factor, mfa_recovery_code, org_unit, organization,
    override_request, payment, period_type, person, prerequisite, progression_detail,
    progression_job,
    report::{
        report_audit_log, report_definition, report_job, report_template, report_template_binding,
        report_template_version,
    },
    role, role_permission, room, scheduled_lesson, seating_number, secret_number, section,
    section_subject, sequence_counter, student, student_fee, student_guardian, subject, time_slot,
    user, user_device, user_group_membership, user_identity, user_role_assignment, user_session,
    workflow::{workflow_action, workflow_execution, workflow_rule},
};

/// Unified schema migration for Orbis ERP.
/// Creates all tables in a single idempotent migration that supports
/// schools, universities, institutes, and training centers.
#[derive(DeriveMigrationName)]
pub struct Migration;

async fn create_table_from_entity<E>(manager: &SchemaManager<'_>, entity: E) -> Result<(), DbErr>
where
    E: sea_orm::EntityTrait,
{
    let builder = manager.get_database_backend();
    let schema = sea_orm::Schema::new(builder);
    let mut stmt = schema.create_table_from_entity(entity);
    stmt.if_not_exists();
    manager.create_table(stmt).await
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, schema: &SchemaManager) -> Result<(), DbErr> {
        // ========================================
        // 1. ORGANIZATION & IAM
        // ========================================
        create_table_from_entity(schema, organization::Entity).await?;
        create_table_from_entity(schema, org_unit::Entity).await?;
        create_table_from_entity(schema, person::Entity).await?;
        create_table_from_entity(schema, user::Entity).await?;
        create_table_from_entity(schema, user_session::Entity).await?;
        create_table_from_entity(schema, role::Entity).await?;
        create_table_from_entity(schema, role_permission::Entity).await?;
        create_table_from_entity(schema, user_role_assignment::Entity).await?;
        create_table_from_entity(schema, user_identity::Entity).await?;
        create_table_from_entity(schema, mfa_factor::Entity).await?;
        create_table_from_entity(schema, mfa_recovery_code::Entity).await?;
        create_table_from_entity(schema, user_device::Entity).await?;

        // ========================================
        // 2. EMPLOYEE & GROUPS
        // ========================================
        create_table_from_entity(schema, employee::Entity).await?;
        create_table_from_entity(schema, group::Entity).await?;
        create_table_from_entity(schema, user_group_membership::Entity).await?;
        create_table_from_entity(schema, group_permission::Entity).await?;

        // ========================================
        // 3. ACADEMIC STRUCTURE (UNIFIED)
        // ========================================
        create_table_from_entity(schema, academic_calendar::Entity).await?;
        create_table_from_entity(schema, period_type::Entity).await?;
        create_table_from_entity(schema, academic_period::Entity).await?;
        create_table_from_entity(schema, subject::Entity).await?;
        create_table_from_entity(schema, prerequisite::Entity).await?;
        create_table_from_entity(schema, section::Entity).await?;
        create_table_from_entity(schema, assessment_scheme::Entity).await?;
        create_table_from_entity(schema, section_subject::Entity).await?;

        // ========================================
        // 4. STUDENT LIFECYCLE
        // ========================================
        create_table_from_entity(schema, student::Entity).await?;
        create_table_from_entity(schema, enrollment::Entity).await?;
        create_table_from_entity(schema, student_guardian::Entity).await?;
        create_table_from_entity(schema, progression_job::Entity).await?;
        create_table_from_entity(schema, progression_detail::Entity).await?;
        create_table_from_entity(schema, override_request::Entity).await?;

        // ========================================
        // 5. ASSESSMENT & GRADING
        // ========================================
        create_table_from_entity(schema, grade::Entity).await?;
        create_table_from_entity(schema, gpa_record::Entity).await?;
        create_table_from_entity(schema, attendance::Entity).await?;
        create_table_from_entity(schema, behavior_record::Entity).await?;

        // ========================================
        // 6. EXAMS
        // ========================================
        create_table_from_entity(schema, exam_session::Entity).await?;
        create_table_from_entity(schema, exam_schedule::Entity).await?;
        create_table_from_entity(schema, exam_proctor::Entity).await?;
        create_table_from_entity(schema, seating_number::Entity).await?;
        create_table_from_entity(schema, secret_number::Entity).await?;

        // ========================================
        // 7. FINANCE
        // ========================================
        create_table_from_entity(schema, currency::Entity).await?;
        create_table_from_entity(schema, exchange_rate::Entity).await?;
        create_table_from_entity(schema, account::Entity).await?;
        create_table_from_entity(schema, journal_entry::Entity).await?;
        create_table_from_entity(schema, journal_entry_line::Entity).await?;
        create_table_from_entity(schema, fee_template::Entity).await?;
        create_table_from_entity(schema, discount_template::Entity).await?;
        create_table_from_entity(schema, student_fee::Entity).await?;
        create_table_from_entity(schema, payment::Entity).await?;

        // ========================================
        // 8. DOCUMENTS & STORAGE
        // ========================================
        create_table_from_entity(schema, document::Entity).await?;
        create_table_from_entity(schema, folder::Entity).await?;
        create_table_from_entity(schema, document_requirement::Entity).await?;
        create_table_from_entity(schema, custom_field_definition::Entity).await?;

        // ========================================
        // 9. INTEGRATION & AUTOMATION
        // ========================================
        create_table_from_entity(schema, integration_config::Entity).await?;
        create_table_from_entity(schema, integration_secret::Entity).await?;
        create_table_from_entity(schema, workflow_rule::Entity).await?;
        create_table_from_entity(schema, workflow_action::Entity).await?;
        create_table_from_entity(schema, workflow_execution::Entity).await?;
        create_table_from_entity(schema, autopilot_task::Entity).await?;

        // ========================================
        // 10. REPORTING
        // ========================================
        create_table_from_entity(schema, report_definition::Entity).await?;
        create_table_from_entity(schema, report_template::Entity).await?;
        create_table_from_entity(schema, report_template_version::Entity).await?;
        create_table_from_entity(schema, report_template_binding::Entity).await?;
        create_table_from_entity(schema, report_job::Entity).await?;
        create_table_from_entity(schema, report_audit_log::Entity).await?;

        // ========================================
        // 11. SCHEDULING & ROOMS
        // ========================================
        create_table_from_entity(schema, room::Entity).await?;
        create_table_from_entity(schema, time_slot::Entity).await?;
        create_table_from_entity(schema, scheduled_lesson::Entity).await?;

        // ========================================
        // 12. INFRASTRUCTURE
        // ========================================
        create_table_from_entity(schema, sequence_counter::Entity).await?;

        // ========================================
        // 13. INDEXES (performance-critical)
        // ========================================
        let db = schema.get_connection();

        // Enrollment indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_enrollments_student_period ON enrollments (student_id, period_id)",
        ).await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_enrollments_unique ON enrollments (org_id, student_id, period_id, section_id, attempt_number)",
        ).await?;

        // Section subject indexes
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_section_subjects_unique ON section_subjects (org_id, section_id, subject_id, period_id)",
        ).await?;

        // Grade indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_grades_section_subject_period ON grades (section_subject_id, period_id)",
        ).await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_grades_student_period ON grades (student_id, period_id)",
        ).await?;

        // Attendance indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_attendances_student_date ON attendances (student_id, attendance_date)",
        ).await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_attendances_unique ON attendances (student_id, section_subject_id, attendance_date)",
        ).await?;

        // Section indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_sections_org ON sections (org_id, section_type)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_sections_code ON sections (org_id, code)",
        )
        .await?;

        // Academic period indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_periods_calendar ON academic_periods (calendar_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_periods_org ON academic_periods (org_id)",
        )
        .await?;

        // Subject indexes
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_subjects_org_code ON subjects (org_id, code)",
        )
        .await?;

        // Prerequisite indexes
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_prerequisites_unique ON prerequisites (org_id, subject_id, prerequisite_subject_id)",
        ).await?;

        // Student indexes
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_students_org ON students (org_id)")
            .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_students_org_number ON students (org_id, student_number)",
        ).await?;

        // Payment indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payments_student ON payments (student_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payments_org_date ON payments (org_id, paid_at)",
        )
        .await?;

        // Student fee indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_student_fees_student_period ON student_fees (student_id, period_id)",
        ).await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_student_fees_org_status ON student_fees (org_id, status)",
        ).await?;

        // Employee indexes
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_employees_org_number ON employees (org_id, employee_number)",
        ).await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_employees_org_person ON employees (org_id, person_id)",
        ).await?;

        // Guardian indexes
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_guardians_student_guardian ON student_guardians (student_id, guardian_id)",
        ).await?;

        // Document indexes
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_documents_org ON documents (org_id)")
            .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_documents_entity ON documents (entity_type, entity_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_documents_folder ON documents (folder_id)",
        )
        .await?;

        // User indexes
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_users_org ON users (org_id)")
            .await?;
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_users_person ON users (person_id)")
            .await?;

        // Report indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_report_jobs_org ON report_jobs (org_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_report_definitions_org ON report_definitions (org_id, domain, category)",
        ).await?;

        // Scheduled lesson indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_scheduled_lessons_section ON scheduled_lessons (section_subject_id)",
        ).await?;

        // Sequence counter
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_sequence_counters_unique ON sequence_counters (org_id, sequence_type, academic_calendar)",
        ).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Full teardown - in reverse order of creation
        let db = _manager.get_connection();

        let tables = vec![
            "sequence_counters",
            "scheduled_lessons",
            "time_slots",
            "rooms",
            "report_audit_log",
            "report_jobs",
            "report_template_bindings",
            "report_template_versions",
            "report_templates",
            "report_definitions",
            "autopilot_tasks",
            "workflow_executions",
            "workflow_actions",
            "workflow_rules",
            "integration_secrets",
            "integration_configs",
            "custom_field_definitions",
            "document_requirements",
            "folders",
            "documents",
            "payments",
            "student_fees",
            "discount_templates",
            "fee_templates",
            "journal_entry_lines",
            "journal_entries",
            "accounts",
            "exchange_rates",
            "currencies",
            "secret_numbers",
            "seating_numbers",
            "exam_proctors",
            "exam_schedules",
            "exam_sessions",
            "behavior_records",
            "attendances",
            "cumulative_gpa_records",
            "grades",
            "academic_override_requests",
            "progression_details",
            "progression_jobs",
            "student_guardians",
            "enrollments",
            "students",
            "section_subjects",
            "sections",
            "assessment_schemes",
            "prerequisites",
            "subjects",
            "academic_periods",
            "period_types",
            "academic_calendars",
            "group_permissions",
            "user_group_memberships",
            "groups",
            "employees",
            "user_devices",
            "mfa_recovery_codes",
            "mfa_factors",
            "user_identities",
            "user_role_assignments",
            "role_permissions",
            "roles",
            "user_sessions",
            "users",
            "persons",
            "org_units",
            "organizations",
        ];

        for table in tables {
            db.execute_unprepared(&format!("DROP TABLE IF EXISTS {}", table))
                .await?;
        }

        Ok(())
    }
}
