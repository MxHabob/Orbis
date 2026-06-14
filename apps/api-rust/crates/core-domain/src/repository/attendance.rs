use crate::entities::attendance;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder, QuerySelect};

impl super::Repository {
    pub async fn create_attendance(&self, a: attendance::Model) -> Result<attendance::Model, DbErr> {
        let active_model = attendance::ActiveModel {
            id: Set(a.id),
            org_id: Set(a.org_id),
            cycle_id: Set(a.cycle_id),
            enrollment_id: Set(a.enrollment_id),
            class_id: Set(a.class_id),
            date: Set(a.date),
            status: Set(a.status),
            check_in_time: Set(a.check_in_time),
            check_out_time: Set(a.check_out_time),
            period: Set(a.period),
            subject_name: Set(a.subject_name),
            recorded_by: Set(a.recorded_by),
            notes: Set(a.notes),
            excuse_type: Set(a.excuse_type),
            excuse_document: Set(a.excuse_document),
            is_excused: Set(a.is_excused),
            created_at: Set(a.created_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_attendance(
        &self,
        org_id: &str,
        params: crate::params::AttendanceParams,
    ) -> Result<Vec<attendance::Model>, DbErr> {
        let mut query = attendance::Entity::find()
            .filter(attendance::Column::OrgId.eq(org_id));

        if let Some(ref enrollment_id) = params.enrollment_id {
            if !enrollment_id.trim().is_empty() {
                query = query.filter(attendance::Column::EnrollmentId.eq(enrollment_id));
            }
        }

        if let Some(ref class_id) = params.class_id {
            if !class_id.trim().is_empty() {
                query = query.filter(attendance::Column::ClassId.eq(class_id));
            }
        }

        if let Some(ref cycle_id) = params.cycle_id {
            if !cycle_id.trim().is_empty() {
                query = query.filter(attendance::Column::CycleId.eq(cycle_id));
            }
        }

        if let Some(ref date) = params.date {
            if !date.trim().is_empty() {
                query = query.filter(attendance::Column::Date.eq(date));
            }
        }

        if let Some(ref date_from) = params.date_from {
            if !date_from.trim().is_empty() {
                query = query.filter(attendance::Column::Date.gte(date_from));
            }
        }

        if let Some(ref date_to) = params.date_to {
            if !date_to.trim().is_empty() {
                query = query.filter(attendance::Column::Date.lte(date_to));
            }
        }

        if let Some(ref status) = params.status {
            if !status.trim().is_empty() {
                query = query.filter(attendance::Column::Status.eq(status));
            }
        }

        query = query.order_by_desc(attendance::Column::Date);

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }
        if let Some(offset) = params.offset {
            query = query.offset(offset);
        }

        query.all(&self.db).await
    }

    pub async fn get_attendance_by_enrollment(&self, enrollment_id: &str) -> Result<Vec<attendance::Model>, DbErr> {
        attendance::Entity::find()
            .filter(attendance::Column::EnrollmentId.eq(enrollment_id))
            .order_by_desc(attendance::Column::Date)
            .all(&self.db)
            .await
    }

    pub async fn get_attendance_by_class_date(&self, class_id: &str, date: &str) -> Result<Vec<attendance::Model>, DbErr> {
        attendance::Entity::find()
            .filter(attendance::Column::ClassId.eq(class_id))
            .filter(attendance::Column::Date.eq(date))
            .all(&self.db)
            .await
    }

    pub async fn get_attendance_count_by_status(&self, enrollment_id: &str, status: &str) -> Result<u64, DbErr> {
        attendance::Entity::find()
            .filter(attendance::Column::EnrollmentId.eq(enrollment_id))
            .filter(attendance::Column::Status.eq(status))
            .count(&self.db)
            .await
    }

    pub async fn get_attendance_total(&self, enrollment_id: &str) -> Result<u64, DbErr> {
        attendance::Entity::find()
            .filter(attendance::Column::EnrollmentId.eq(enrollment_id))
            .count(&self.db)
            .await
    }
}

