use sea_orm::Schema;
use sea_orm_migration::prelude::*;

use crate::models::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        let table_statements = vec![
            // ── 1. Organization & Core IAM ──────────────────────────────
            schema.create_table_from_entity(organization::Entity),
            schema.create_table_from_entity(org_unit::Entity),
            schema.create_table_from_entity(person::Entity),
            schema.create_table_from_entity(user::Entity),
            schema.create_table_from_entity(user_session::Entity),
            schema.create_table_from_entity(role::Entity),
            schema.create_table_from_entity(role_permission::Entity),
            schema.create_table_from_entity(user_role_assignment::Entity),
            schema.create_table_from_entity(user_identity::Entity),
            schema.create_table_from_entity(mfa_factor::Entity),
            schema.create_table_from_entity(mfa_recovery_code::Entity),
            schema.create_table_from_entity(user_device::Entity),

            // ── 2. Academic Calendar & Structure ────────────────────────
            schema.create_table_from_entity(period_template::Entity),
            schema.create_table_from_entity(academic_year::Entity),
            schema.create_table_from_entity(academic_period::Entity),
            schema.create_table_from_entity(room::Entity),
            schema.create_table_from_entity(time_slot::Entity),
            schema.create_table_from_entity(subject::Entity),
            schema.create_table_from_entity(course_prerequisite::Entity),
            schema.create_table_from_entity(class::Entity),
            schema.create_table_from_entity(class_subject::Entity),
            schema.create_table_from_entity(scheduled_lesson::Entity),

            // ── 3. Student Lifecycle & Guardians ────────────────────────
            schema.create_table_from_entity(student::Entity),
            schema.create_table_from_entity(enrollment::Entity),
            schema.create_table_from_entity(student_guardian::Entity),
            schema.create_table_from_entity(promotion_job::Entity),
            schema.create_table_from_entity(promotion_detail::Entity),
            schema.create_table_from_entity(override_request::Entity),

            // ── 4. Assessment, Grading & Exams ──────────────────────────
            schema.create_table_from_entity(assessment_scheme::Entity),
            schema.create_table_from_entity(grade::Entity),
            schema.create_table_from_entity(gpa_record::Entity),
            schema.create_table_from_entity(attendance::Entity),
            schema.create_table_from_entity(behavior_record::Entity),
            schema.create_table_from_entity(exam_session::Entity),
            schema.create_table_from_entity(exam_schedule::Entity),
            schema.create_table_from_entity(seating_number::Entity),
            schema.create_table_from_entity(secret_number::Entity),
            schema.create_table_from_entity(exam_proctor::Entity),

            // ── 5. Finance & Accounting ─────────────────────────────────
            schema.create_table_from_entity(currency::Entity),
            schema.create_table_from_entity(exchange_rate::Entity),
            schema.create_table_from_entity(account::Entity),
            schema.create_table_from_entity(journal_entry::Entity),
            schema.create_table_from_entity(journal_entry_line::Entity),
            schema.create_table_from_entity(fee_template::Entity),
            schema.create_table_from_entity(discount_template::Entity),
            schema.create_table_from_entity(student_fee::Entity),
            schema.create_table_from_entity(payment::Entity),

            // ── 6. Documents, Integrations & Automation ─────────────────
            schema.create_table_from_entity(document::Entity),
            schema.create_table_from_entity(integration::integration_config::Entity),
            schema.create_table_from_entity(integration::integration_secret::Entity),
            schema.create_table_from_entity(ai::ai_agent::Entity),
            schema.create_table_from_entity(ai::ai_insight::Entity),
            schema.create_table_from_entity(workflow::workflow_rule::Entity),
            schema.create_table_from_entity(workflow::workflow_action::Entity),
            schema.create_table_from_entity(workflow::workflow_execution::Entity),
            schema.create_table_from_entity(autopilot_task::Entity),

            // ── 7. Reporting & Analytics ────────────────────────────────
            schema.create_table_from_entity(report::report_definition::Entity),
            schema.create_table_from_entity(report::report_template::Entity),
            schema.create_table_from_entity(report::report_template_version::Entity),
            schema.create_table_from_entity(report::report_template_binding::Entity),
            schema.create_table_from_entity(report::report_job::Entity),
            schema.create_table_from_entity(report::report_audit_log::Entity),
        ];

        for mut stmt in table_statements {
            manager.create_table(stmt.if_not_exists().to_owned()).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tables = vec![
            report::report_audit_log::Entity.to_string(),
            report::report_job::Entity.to_string(),
            report::report_template_binding::Entity.to_string(),
            report::report_template_version::Entity.to_string(),
            report::report_template::Entity.to_string(),
            report::report_definition::Entity.to_string(),
            autopilot_task::Entity.to_string(),
            workflow::workflow_execution::Entity.to_string(),
            workflow::workflow_action::Entity.to_string(),
            workflow::workflow_rule::Entity.to_string(),
            ai::ai_insight::Entity.to_string(),
            ai::ai_agent::Entity.to_string(),
            integration::integration_secret::Entity.to_string(),
            integration::integration_config::Entity.to_string(),
            document::Entity.to_string(),
            payment::Entity.to_string(),
            student_fee::Entity.to_string(),
            discount_template::Entity.to_string(),
            fee_template::Entity.to_string(),
            journal_entry_line::Entity.to_string(),
            journal_entry::Entity.to_string(),
            account::Entity.to_string(),
            exchange_rate::Entity.to_string(),
            currency::Entity.to_string(),
            exam_proctor::Entity.to_string(),
            secret_number::Entity.to_string(),
            seating_number::Entity.to_string(),
            exam_schedule::Entity.to_string(),
            exam_session::Entity.to_string(),
            behavior_record::Entity.to_string(),
            attendance::Entity.to_string(),
            gpa_record::Entity.to_string(),
            grade::Entity.to_string(),
            assessment_scheme::Entity.to_string(),
            override_request::Entity.to_string(),
            promotion_detail::Entity.to_string(),
            promotion_job::Entity.to_string(),
            student_guardian::Entity.to_string(),
            enrollment::Entity.to_string(),
            student::Entity.to_string(),
            scheduled_lesson::Entity.to_string(),
            class_subject::Entity.to_string(),
            class::Entity.to_string(),
            course_prerequisite::Entity.to_string(),
            subject::Entity.to_string(),
            time_slot::Entity.to_string(),
            room::Entity.to_string(),
            academic_period::Entity.to_string(),
            academic_year::Entity.to_string(),
            period_template::Entity.to_string(),
            user_device::Entity.to_string(),
            mfa_recovery_code::Entity.to_string(),
            mfa_factor::Entity.to_string(),
            user_identity::Entity.to_string(),
            user_role_assignment::Entity.to_string(),
            role_permission::Entity.to_string(),
            role::Entity.to_string(),
            user_session::Entity.to_string(),
            user::Entity.to_string(),
            person::Entity.to_string(),
            org_unit::Entity.to_string(),
            organization::Entity.to_string(),
        ];

        for table in tables {
            let _ = manager.get_connection().execute_unprepared(&format!("DROP TABLE IF EXISTS {};", table)).await;
        }

        Ok(())
    }
}
