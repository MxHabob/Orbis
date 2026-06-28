// ═══════════════════════════════════════════════════════════════
// docforge-orbis — Template CRUD Service
// Manages template lifecycle: create, read, update, delete,
// versioning, publishing, and archiving.
// ═══════════════════════════════════════════════════════════════

use sea_orm::{ConnectionTrait, FromQueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

/// Template status in the lifecycle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TemplateStatus {
    Draft,
    Published,
    Archived,
}

impl std::fmt::Display for TemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Published => write!(f, "published"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

// ── DTOs ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub document_type: String,
    pub entity_type: String,
    pub category_id: Option<String>,
    pub code: Option<String>,
    pub page_size: Option<String>,
    pub page_width_mm: Option<f64>,
    pub page_height_mm: Option<f64>,
    pub orientation: Option<String>,
    pub margins_json: Option<String>,
    pub document_json: Option<String>,
    pub locale: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub category_id: Option<String>,
    pub code: Option<String>,
    pub page_size: Option<String>,
    pub page_width_mm: Option<f64>,
    pub page_height_mm: Option<f64>,
    pub orientation: Option<String>,
    pub margins_json: Option<String>,
    pub document_json: Option<String>,
    pub locale: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct TemplateRecord {
    pub id: String,
    pub org_id: String,
    pub category_id: Option<String>,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub document_type: String,
    pub entity_type: String,
    pub page_size: Option<String>,
    pub page_width_mm: Option<f64>,
    pub page_height_mm: Option<f64>,
    pub orientation: Option<String>,
    pub margins_json: Option<String>,
    pub document_json: String,
    pub status: String,
    pub version: i32,
    pub published_version: Option<i32>,
    pub published_at: Option<String>,
    pub published_by: Option<String>,
    pub created_by: String,
    pub is_system: i32,
    pub is_default: i32,
    pub is_locked: i32,
    pub tags: Option<String>,
    pub locale: Option<String>,
    pub thumbnail_url: Option<String>,
    pub meta: Option<String>,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct TemplateVersionRecord {
    pub id: String,
    pub template_id: String,
    pub version: i32,
    pub document_json: String,
    pub page_size: Option<String>,
    pub page_width_mm: Option<f64>,
    pub page_height_mm: Option<f64>,
    pub orientation: Option<String>,
    pub margins_json: Option<String>,
    pub change_summary: Option<String>,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTemplatesParams {
    pub org_id: String,
    pub status: Option<String>,
    pub document_type: Option<String>,
    pub entity_type: Option<String>,
    pub category_id: Option<String>,
    pub search: Option<String>,
}

// ── Service ──────────────────────────────────────────────────

pub struct TemplateService;

impl TemplateService {
    /// Create a new template in draft status.
    pub async fn create(
        db: &impl ConnectionTrait,
        org_id: &str,
        user_id: &str,
        req: CreateTemplateRequest,
    ) -> Result<TemplateRecord, String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let document_json = req.document_json.unwrap_or_else(|| "{}".to_string());

        db.execute_unprepared(&format!(
            "INSERT INTO templates (
                id, org_id, category_id, code, name, name_ar,
                description, description_ar,
                document_type, entity_type,
                page_size, page_width_mm, page_height_mm,
                orientation, margins_json, document_json,
                status, version, created_by,
                locale, tags,
                created_at, updated_at
            ) VALUES (
                '{}', '{}', {}, {}, '{}', {},
                {}, {},
                '{}', '{}',
                '{}', {}, {},
                '{}', '{}', '{}',
                'draft', 1, '{}',
                '{}', {},
                '{}', '{}'
            )",
            id, org_id,
            opt_sql(&req.category_id), opt_sql(&req.code),
            sql_escape(&req.name), opt_sql(&req.name_ar),
            opt_sql(&req.description), opt_sql(&req.description_ar),
            sql_escape(&req.document_type), sql_escape(&req.entity_type),
            req.page_size.as_deref().unwrap_or("A4"),
            req.page_width_mm.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string()),
            req.page_height_mm.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string()),
            req.orientation.as_deref().unwrap_or("portrait"),
            req.margins_json.as_deref().unwrap_or(r#"{"top":10,"bottom":10,"left":10,"right":10}"#),
            sql_escape(&document_json),
            user_id,
            req.locale.as_deref().unwrap_or("ar"),
            opt_sql(&req.tags),
            now, now,
        ))
        .await
        .map_err(|e| e.to_string())?;

        // Return the created record
        Self::get_by_id(db, org_id, &id).await
    }

    /// Get a template by ID, scoped to an organization.
    pub async fn get_by_id(
        db: &impl ConnectionTrait,
        org_id: &str,
        template_id: &str,
    ) -> Result<TemplateRecord, String> {
        let result = TemplateRecord::find_by_statement(
            sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                format!(
                    "SELECT * FROM templates WHERE id = '{}' AND org_id = '{}' AND deleted_at IS NULL",
                    sql_escape(template_id), sql_escape(org_id)
                ),
            ),
        )
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

        result.ok_or_else(|| format!("Template not found: {}", template_id))
    }

    /// List templates for an organization with optional filters.
    pub async fn list(
        db: &impl ConnectionTrait,
        params: ListTemplatesParams,
    ) -> Result<Vec<TemplateRecord>, String> {
        let mut sql = format!(
            "SELECT * FROM templates WHERE org_id = '{}' AND deleted_at IS NULL",
            sql_escape(&params.org_id)
        );

        if let Some(ref status) = params.status {
            sql.push_str(&format!(" AND status = '{}'", sql_escape(status)));
        }
        if let Some(ref doc_type) = params.document_type {
            sql.push_str(&format!(" AND document_type = '{}'", sql_escape(doc_type)));
        }
        if let Some(ref entity) = params.entity_type {
            sql.push_str(&format!(" AND entity_type = '{}'", sql_escape(entity)));
        }
        if let Some(ref cat) = params.category_id {
            sql.push_str(&format!(" AND category_id = '{}'", sql_escape(cat)));
        }
        if let Some(ref search) = params.search {
            sql.push_str(&format!(
                " AND (name LIKE '%{}%' OR name_ar LIKE '%{}%')",
                sql_escape(search), sql_escape(search)
            ));
        }

        sql.push_str(" ORDER BY updated_at DESC");

        TemplateRecord::find_by_statement(
            sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Sqlite, sql),
        )
        .all(db)
        .await
        .map_err(|e| e.to_string())
    }

    /// Update a template's metadata and/or document JSON.
    /// Automatically increments the version number.
    pub async fn update(
        db: &impl ConnectionTrait,
        org_id: &str,
        template_id: &str,
        user_id: &str,
        req: UpdateTemplateRequest,
    ) -> Result<TemplateRecord, String> {
        let existing = Self::get_by_id(db, org_id, template_id).await?;
        let now = Utc::now().to_rfc3339();

        // Build SET clauses
        let mut sets = Vec::new();
        if let Some(ref name) = req.name { sets.push(format!("name = '{}'", sql_escape(name))); }
        if let Some(ref name_ar) = req.name_ar { sets.push(format!("name_ar = '{}'", sql_escape(name_ar))); }
        if let Some(ref desc) = req.description { sets.push(format!("description = '{}'", sql_escape(desc))); }
        if let Some(ref desc_ar) = req.description_ar { sets.push(format!("description_ar = '{}'", sql_escape(desc_ar))); }
        if let Some(ref cat) = req.category_id { sets.push(format!("category_id = '{}'", sql_escape(cat))); }
        if let Some(ref code) = req.code { sets.push(format!("code = '{}'", sql_escape(code))); }
        if let Some(ref ps) = req.page_size { sets.push(format!("page_size = '{}'", sql_escape(ps))); }
        if let Some(w) = req.page_width_mm { sets.push(format!("page_width_mm = {}", w)); }
        if let Some(h) = req.page_height_mm { sets.push(format!("page_height_mm = {}", h)); }
        if let Some(ref o) = req.orientation { sets.push(format!("orientation = '{}'", sql_escape(o))); }
        if let Some(ref m) = req.margins_json { sets.push(format!("margins_json = '{}'", sql_escape(m))); }
        if let Some(ref locale) = req.locale { sets.push(format!("locale = '{}'", sql_escape(locale))); }
        if let Some(ref tags) = req.tags { sets.push(format!("tags = '{}'", sql_escape(tags))); }

        // If document changed, bump version and create snapshot
        let _new_version = if let Some(ref doc_json) = req.document_json {
            sets.push(format!("document_json = '{}'", sql_escape(doc_json)));
            let v = existing.version + 1;
            sets.push(format!("version = {}", v));

            // Create version snapshot
            Self::create_version_snapshot(db, template_id, v, doc_json, &existing, user_id, &now).await?;

            v
        } else {
            existing.version
        };

        sets.push(format!("updated_at = '{}'", now));

        if sets.is_empty() {
            return Self::get_by_id(db, org_id, template_id).await;
        }

        let sql = format!(
            "UPDATE templates SET {} WHERE id = '{}' AND org_id = '{}'",
            sets.join(", "),
            sql_escape(template_id),
            sql_escape(org_id),
        );

        db.execute_unprepared(&sql).await.map_err(|e| e.to_string())?;

        Self::get_by_id(db, org_id, template_id).await
    }

    /// Soft-delete a template.
    pub async fn delete(
        db: &impl ConnectionTrait,
        org_id: &str,
        template_id: &str,
    ) -> Result<(), String> {
        let now = Utc::now().to_rfc3339();
        let sql = format!(
            "UPDATE templates SET deleted_at = '{}' WHERE id = '{}' AND org_id = '{}'",
            now, sql_escape(template_id), sql_escape(org_id)
        );
        db.execute_unprepared(&sql).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Publish a template — makes the current draft the active version.
    pub async fn publish(
        db: &impl ConnectionTrait,
        org_id: &str,
        template_id: &str,
        user_id: &str,
    ) -> Result<TemplateRecord, String> {
        let existing = Self::get_by_id(db, org_id, template_id).await?;
        let now = Utc::now().to_rfc3339();

        let sql = format!(
            "UPDATE templates SET status = 'published', published_version = {}, published_at = '{}', published_by = '{}', updated_at = '{}' WHERE id = '{}' AND org_id = '{}'",
            existing.version, now, sql_escape(user_id), now,
            sql_escape(template_id), sql_escape(org_id)
        );

        db.execute_unprepared(&sql).await.map_err(|e| e.to_string())?;

        Self::get_by_id(db, org_id, template_id).await
    }

    /// Archive a template.
    pub async fn archive(
        db: &impl ConnectionTrait,
        org_id: &str,
        template_id: &str,
    ) -> Result<TemplateRecord, String> {
        let now = Utc::now().to_rfc3339();
        let sql = format!(
            "UPDATE templates SET status = 'archived', updated_at = '{}' WHERE id = '{}' AND org_id = '{}'",
            now, sql_escape(template_id), sql_escape(org_id)
        );
        db.execute_unprepared(&sql).await.map_err(|e| e.to_string())?;
        Self::get_by_id(db, org_id, template_id).await
    }

    /// Restore an archived template to draft.
    pub async fn restore(
        db: &impl ConnectionTrait,
        org_id: &str,
        template_id: &str,
    ) -> Result<TemplateRecord, String> {
        let now = Utc::now().to_rfc3339();
        let sql = format!(
            "UPDATE templates SET status = 'draft', updated_at = '{}' WHERE id = '{}' AND org_id = '{}'",
            now, sql_escape(template_id), sql_escape(org_id)
        );
        db.execute_unprepared(&sql).await.map_err(|e| e.to_string())?;
        Self::get_by_id(db, org_id, template_id).await
    }

    /// Get version history for a template.
    pub async fn get_versions(
        db: &impl ConnectionTrait,
        template_id: &str,
    ) -> Result<Vec<TemplateVersionRecord>, String> {
        let sql = format!(
            "SELECT * FROM template_versions WHERE template_id = '{}' ORDER BY version DESC",
            sql_escape(template_id)
        );

        TemplateVersionRecord::find_by_statement(
            sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Sqlite, sql),
        )
        .all(db)
        .await
        .map_err(|e| e.to_string())
    }

    /// Revert to a specific version (creates a new version with old content).
    pub async fn revert_to_version(
        db: &impl ConnectionTrait,
        org_id: &str,
        template_id: &str,
        target_version: i32,
        user_id: &str,
    ) -> Result<TemplateRecord, String> {
        // Find the target version
        let sql = format!(
            "SELECT * FROM template_versions WHERE template_id = '{}' AND version = {}",
            sql_escape(template_id), target_version
        );

        let version = TemplateVersionRecord::find_by_statement(
            sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Sqlite, sql),
        )
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Version {} not found", target_version))?;

        // Update the template with the old document, creating a new version
        Self::update(
            db, org_id, template_id, user_id,
            UpdateTemplateRequest {
                name: None, name_ar: None,
                description: None, description_ar: None,
                category_id: None, code: None,
                page_size: version.page_size,
                page_width_mm: version.page_width_mm,
                page_height_mm: version.page_height_mm,
                orientation: version.orientation,
                margins_json: version.margins_json,
                document_json: Some(version.document_json),
                locale: None, tags: None,
            },
        )
        .await
    }

    // ── Internal ─────────────────────────────────────────────

    async fn create_version_snapshot(
        db: &impl ConnectionTrait,
        template_id: &str,
        version: i32,
        document_json: &str,
        existing: &TemplateRecord,
        user_id: &str,
        now: &str,
    ) -> Result<(), String> {
        let version_id = Uuid::new_v4().to_string();
        let sql = format!(
            "INSERT INTO template_versions (id, template_id, version, document_json, page_size, page_width_mm, page_height_mm, orientation, margins_json, created_by, created_at) VALUES ('{}', '{}', {}, '{}', {}, {}, {}, {}, {}, '{}', '{}')",
            version_id,
            sql_escape(template_id),
            version,
            sql_escape(document_json),
            opt_sql(&existing.page_size),
            existing.page_width_mm.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string()),
            existing.page_height_mm.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string()),
            opt_sql(&existing.orientation),
            opt_sql(&existing.margins_json),
            sql_escape(user_id),
            now,
        );

        db.execute_unprepared(&sql).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

// ── SQL Helpers ──────────────────────────────────────────────

fn sql_escape(s: &str) -> String {
    s.replace('\'', "''")
}

fn opt_sql(opt: &Option<String>) -> String {
    match opt {
        Some(s) => format!("'{}'", sql_escape(s)),
        None => "NULL".to_string(),
    }
}
