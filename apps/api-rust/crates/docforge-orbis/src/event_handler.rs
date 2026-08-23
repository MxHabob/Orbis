use sea_orm::DatabaseConnection;
use shared_kernel::event_bus::EventEnvelope;
use tracing::{info, error};

use crate::projection::ProjectionStore;

pub async fn handle_domain_event(db: &DatabaseConnection, event: &EventEnvelope) {
    if event.event_type == "PeriodScoreCalculated" && event.entity_type == "period_score" {
        info!("docforge-orbis processing PeriodScoreCalculated for {}", event.entity_id);
        
        let org_id = &event.org_id;
        let entity_id = &event.entity_id;
        
        // The payload contains the projection data we need for the report card
        let data = event.payload.clone();
        
        // We register this projection under 'student_report_card'
        match ProjectionStore::upsert(db, org_id, "student_report_card", entity_id, data).await {
            Ok(_) => info!("Successfully updated projection for student_report_card {}", entity_id),
            Err(e) => error!("Failed to upsert projection: {}", e),
        }
    }
}
