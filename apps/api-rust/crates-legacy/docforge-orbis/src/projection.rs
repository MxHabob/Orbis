use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::Value;

pub struct ProjectionStore;

impl ProjectionStore {
    /// Save a denormalized JSON projection for a specific entity
    pub async fn upsert(
        db: &DatabaseConnection,
        org_id: &str,
        entity_type: &str,
        entity_id: &str,
        data: Value,
    ) -> Result<(), String> {
        let now = chrono::Utc::now().to_rfc3339();

        let data_str = data.to_string().replace('\'', "''");
        let org_id = org_id.replace('\'', "''");
        let entity_type = entity_type.replace('\'', "''");
        let entity_id = entity_id.replace('\'', "''");

        // Check if exists
        let check_stmt = sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            format!(
                "SELECT version FROM docforge_projections WHERE org_id = '{}' AND entity_type = '{}' AND entity_id = '{}'",
                org_id, entity_type, entity_id
            ),
        );

        let opt = db.query_one(check_stmt).await.map_err(|e| e.to_string())?;

        if let Some(record) = opt {
            let version = record.try_get::<i32>("", "version").unwrap_or(1);
            let update_stmt = sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                format!(
                    "UPDATE docforge_projections SET data = '{}', version = {}, updated_at = '{}' WHERE org_id = '{}' AND entity_type = '{}' AND entity_id = '{}'",
                    data_str, version + 1, now, org_id, entity_type, entity_id
                ),
            );
            db.execute_unprepared(&update_stmt.to_string())
                .await
                .map_err(|e| e.to_string())?;
        } else {
            let id = uuid::Uuid::new_v4().to_string();
            let insert_stmt = sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO docforge_projections (id, org_id, entity_type, entity_id, data, version, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', 1, '{}', '{}')",
                    id, org_id, entity_type, entity_id, data_str, now, now
                ),
            );
            db.execute_unprepared(&insert_stmt.to_string())
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
