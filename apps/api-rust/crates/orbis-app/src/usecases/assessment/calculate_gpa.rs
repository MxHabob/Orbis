use orbis_domain::entities::gpa_record::CumulativeGpaRecord;
use orbis_domain::entities::student::AcademicStanding;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::{GpaRecordRepository, GradeRepository, EnrollmentRepository};
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

pub struct CalculateGpaUseCase {
    pub gpa_repo: Arc<dyn GpaRecordRepository>,
    pub grade_repo: Arc<dyn GradeRepository>,
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CalculateGpaUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
    ) -> Result<CumulativeGpaRecord, DomainError> {
        let grades = self.grade_repo.find_by_student(student_id).await?;
        let past_records = self.gpa_repo.find_by_student(student_id).await?;

        // Calculate term GPA
        let mut term_attempted = Decimal::ZERO;
        let mut term_earned = Decimal::ZERO;
        let mut term_points = Decimal::ZERO;

        for g in &grades {
            let credit = Decimal::from(3); // Standard course credit default
            term_attempted += credit;
            if g.score >= Decimal::from(60) {
                term_earned += credit;
            }
            // Point conversion: 4.0 scale based on score percentage
            let course_gpa_point = (g.score / Decimal::from(100)) * Decimal::from(4);
            term_points += course_gpa_point * credit;
        }

        let term_gpa = if term_attempted > Decimal::ZERO {
            (term_points / term_attempted).round_dp(2)
        } else {
            Decimal::ZERO
        };

        // Aggregate with past records for Cumulative GPA
        let mut cum_attempted = term_attempted;
        let mut cum_earned = term_earned;
        let mut cum_points = term_points;

        for rec in &past_records {
            if rec.period_id != period_id {
                cum_attempted += rec.term_attempted_credits;
                cum_earned += rec.term_earned_credits;
                cum_points += rec.term_points;
            }
        }

        let cumulative_gpa = if cum_attempted > Decimal::ZERO {
            (cum_points / cum_attempted).round_dp(2)
        } else {
            Decimal::ZERO
        };

        // Determine standing based on cumulative GPA
        let standing = if cumulative_gpa >= Decimal::from_str_exact("3.5").unwrap_or(Decimal::from(3)) {
            AcademicStanding::HonorList
        } else if cumulative_gpa >= Decimal::from(2) {
            AcademicStanding::GoodStanding
        } else if cumulative_gpa >= Decimal::from_str_exact("1.75").unwrap_or(Decimal::from(1)) {
            AcademicStanding::AcademicWarning1
        } else if cumulative_gpa >= Decimal::from_str_exact("1.5").unwrap_or(Decimal::from(1)) {
            AcademicStanding::AcademicWarning2
        } else if cumulative_gpa >= Decimal::from(1) {
            AcademicStanding::Probation
        } else {
            AcademicStanding::Dismissed
        };

        let now = Utc::now();
        let record = CumulativeGpaRecord {
            id: Uuid::new_v4(),
            org_id,
            student_id,
            period_id,
            term_attempted_credits: term_attempted,
            term_earned_credits: term_earned,
            term_points,
            term_gpa,
            cumulative_attempted_credits: cum_attempted,
            cumulative_earned_credits: cum_earned,
            cumulative_points: cum_points,
            cumulative_gpa,
            standing,
            calculated_at: now,
        };

        self.gpa_repo.save(&record).await?;

        let event = DomainEvent::GpaCalculated {
            student_id,
            period_id,
            gpa: term_gpa,
            cgpa: cumulative_gpa,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(record)
    }
}
