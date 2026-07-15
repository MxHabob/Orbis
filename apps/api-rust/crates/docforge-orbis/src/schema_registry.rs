use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::Value;

pub struct SchemaRegistry;

impl SchemaRegistry {
    /// Save a JSON Schema for a specific entity type
    pub async fn register(
        db: &DatabaseConnection,
        org_id: &str,
        provider_name: &str,
        schema_payload: Value,
    ) -> Result<(), String> {
        let now = chrono::Utc::now().to_rfc3339();
        
        let schema_str = schema_payload.to_string().replace('\'', "''");
        let org_id = org_id.replace('\'', "''");
        let provider_name = provider_name.replace('\'', "''");

        // Check if exists
        let check_stmt = sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            format!(
                "SELECT version FROM docforge_schema_registry WHERE org_id = '{}' AND provider_name = '{}'",
                org_id, provider_name
            ),
        );

        let opt = db.query_one(check_stmt).await.map_err(|e| e.to_string())?;

        if let Some(record) = opt {
            let version = record.try_get::<i32>("", "version").unwrap_or(1);
            let update_stmt = sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                format!(
                    "UPDATE docforge_schema_registry SET schema_payload = '{}', version = {}, updated_at = '{}' WHERE org_id = '{}' AND provider_name = '{}'",
                    schema_str, version + 1, now, org_id, provider_name
                ),
            );
            db.execute_unprepared(&update_stmt.to_string()).await.map_err(|e| e.to_string())?;
        } else {
            let id = uuid::Uuid::new_v4().to_string();
            let insert_stmt = sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO docforge_schema_registry (id, org_id, provider_name, schema_payload, version, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', 1, '{}', '{}')",
                    id, org_id, provider_name, schema_str, now, now
                ),
            );
            db.execute_unprepared(&insert_stmt.to_string()).await.map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Retrieve a schema for a specific provider
    pub async fn get(
        db: &DatabaseConnection,
        org_id: &str,
        provider_name: &str,
    ) -> Result<Value, String> {
        let org_id = org_id.replace('\'', "''");
        let provider_name = provider_name.replace('\'', "''");

        let stmt = sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            format!(
                "SELECT schema_payload FROM docforge_schema_registry WHERE org_id = '{}' AND provider_name = '{}'",
                org_id, provider_name
            ),
        );

        let opt = db.query_one(stmt).await.map_err(|e| e.to_string())?;
        let record = opt.ok_or_else(|| format!("Schema not found for provider: {}", provider_name))?;

        let payload_str = record.try_get::<String>("", "schema_payload").map_err(|e| e.to_string())?;
        let schema: Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
        
        Ok(schema)
    }
}
