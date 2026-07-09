use sea_orm::DatabaseConnection;

pub mod user;
pub mod user_mgmt;
pub mod class;
pub mod behavior;
pub mod notification;
pub mod organization;
pub mod academic_cycle;
pub mod org_unit;
pub mod enrollment;
pub mod grade;
pub mod attendance;
pub mod finance;
pub mod schedule;
pub mod subject;
pub mod assessment;
pub mod guardian;
pub mod document;
pub mod currency;
pub mod permission;
pub mod solve_job;

// ── Examination Management System ──
pub mod exam;

// ── Academic Year Management & Promotion (Registry DB) ──
// These repositories use standalone structs (AcademicYearRepo, PromotionRepo)
// that operate on the registry DatabaseConnection, not the year Repository.
pub mod academic_year;
pub mod promotion;


#[derive(Clone)]
pub struct Repository {
    pub db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
