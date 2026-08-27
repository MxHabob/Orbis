use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_json::Value;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DatabaseBackend, Statement, TransactionTrait};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::config::CliConfig;
use crate::TemplateCommands;

#[derive(Debug, Deserialize)]
struct PackageManifest {
    #[serde(rename = "packageId")]
    package_id: String,
    #[serde(rename = "packageVersion")]
    package_version: String,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "displayNameAr")]
    display_name_ar: Option<String>,
    #[serde(default)]
    categories: Vec<CategoryManifest>,
    #[serde(default)]
    assets: Vec<AssetManifest>,
    #[serde(default)]
    templates: Vec<TemplateManifest>,
}

#[derive(Debug, Deserialize)]
struct AssetManifest {
    #[serde(rename = "assetId")]
    asset_id: String,
    name: String,
    #[serde(rename = "nameAr")]
    name_ar: Option<String>,
    #[serde(rename = "assetType")]
    asset_type: String,
    #[serde(rename = "mimeType")]
    mime_type: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct CategoryManifest {
    id: String,
    name: String,
    #[serde(rename = "nameAr")]
    name_ar: Option<String>,
    #[serde(default)]
    sort_order: i32,
}

#[derive(Debug, Clone, Deserialize)]
struct TemplateManifest {
    #[serde(rename = "templateId")]
    template_id: String,
    code: String,
    category: String,
    name: String,
    #[serde(rename = "nameAr")]
    name_ar: Option<String>,
    #[serde(rename = "documentType")]
    document_type: String,
    #[serde(rename = "entityType")]
    entity_type: String,
    #[serde(rename = "documentPath")]
    document_path: String,
    #[serde(rename = "fixturePath")]
    fixture_path: Option<String>,
    locale: Option<String>,
    #[serde(default)]
    default: bool,
    #[serde(default)]
    locked: bool,
}

struct LoadedPackage {
    root: PathBuf,
    manifest: PackageManifest,
    documents: Vec<(TemplateManifest, String)>,
}

pub async fn execute(command: &TemplateCommands) -> Result<()> {
    match command {
        TemplateCommands::Validate { package } => {
            let loaded = load_package(package)?;
            validate_package(&loaded)?;
            println!("Valid template package: {} {}", loaded.manifest.display_name, loaded.manifest.package_version);
        }
        TemplateCommands::Create { package } => install(package, false).await?,
        TemplateCommands::List => list_installed().await?,
        TemplateCommands::Install { package } => install(package, false).await?,
        TemplateCommands::Update { package } => install(package, true).await?,
    }
    Ok(())
}

fn load_package(path: &Path) -> Result<LoadedPackage> {
    let root = if path.is_dir() { path.to_path_buf() } else {
        return Err(anyhow!("Template package must be a directory: {}", path.display()));
    };
    let manifest_path = root.join("manifest.json");
    let manifest_text = fs::read_to_string(&manifest_path)
        .with_context(|| format!("Unable to read {}", manifest_path.display()))?;
    let manifest: PackageManifest = serde_json::from_str(&manifest_text)
        .with_context(|| format!("Invalid package manifest: {}", manifest_path.display()))?;

    if manifest.package_id.trim().is_empty() || manifest.package_version.trim().is_empty() {
        return Err(anyhow!("Manifest requires packageId and packageVersion"));
    }
    if manifest.templates.is_empty() {
        return Err(anyhow!("Manifest must contain at least one template"));
    }

    let mut documents = Vec::with_capacity(manifest.templates.len());
    for template in &manifest.templates {
        let path = root.join(&template.document_path);
        let document = fs::read_to_string(&path)
            .with_context(|| format!("Unable to read document {}", path.display()))?;
        documents.push((template.clone(), document));
    }

    Ok(LoadedPackage { root, manifest, documents })
}

fn validate_package(package: &LoadedPackage) -> Result<()> {
    let mut category_ids = HashSet::new();
    for category in &package.manifest.categories {
        if !category_ids.insert(&category.id) {
            return Err(anyhow!("Duplicate category id: {}", category.id));
        }
    }

    for asset in &package.manifest.assets {
        if asset.asset_id.trim().is_empty() || asset.mime_type.trim().is_empty() {
            return Err(anyhow!("Asset requires assetId and mimeType"));
        }
        let path = package.root.join(&asset.path);
        if !path.is_file() {
            return Err(anyhow!("Unable to read asset {}", path.display()));
        }
    }

    let mut template_ids = HashSet::new();
    let mut codes = HashSet::new();
    for (template, document_json) in &package.documents {
        if !template_ids.insert(&template.template_id) {
            return Err(anyhow!("Duplicate template id: {}", template.template_id));
        }
        if !codes.insert(&template.code) {
            return Err(anyhow!("Duplicate template code: {}", template.code));
        }
        if !category_ids.contains(&template.category) {
            return Err(anyhow!("Template {} references unknown category {}", template.template_id, template.category));
        }
        if !template.document_path.ends_with(".json") {
            return Err(anyhow!("Template document must be JSON: {}", template.document_path));
        }

        let document_value: Value = serde_json::from_str(document_json)
            .with_context(|| format!("Invalid JSON document: {}", template.template_id))?;
        reject_unsupported_features(&document_value, &template.template_id)?;
        let document: docforge_engine::model::TemplateDocument = serde_json::from_value(document_value)
            .with_context(|| format!("Document is not compatible with the Rust engine: {}", template.template_id))?;
        if document.root.id().is_empty() {
            return Err(anyhow!("Template {} has an empty root id", template.template_id));
        }
        if document.meta.entity_type != template.entity_type || document.meta.document_type != template.document_type {
            return Err(anyhow!("Template metadata mismatch: {}", template.template_id));
        }
        if let Some(fixture_path) = &template.fixture_path {
            let path = package.root.join(fixture_path);
            let fixture = fs::read_to_string(&path)
                .with_context(|| format!("Unable to read fixture {}", path.display()))?;
            serde_json::from_str::<Value>(&fixture)
                .with_context(|| format!("Invalid fixture: {}", fixture_path))?;
        }
    }
    Ok(())
}

fn reject_unsupported_features(value: &Value, template_id: &str) -> Result<()> {
    fn walk(value: &Value, template_id: &str) -> Result<()> {
        match value {
            Value::Object(map) => {
                if map.get("type").and_then(Value::as_str) == Some("ComponentRef") {
                    return Err(anyhow!("Template {} uses unsupported ComponentRef", template_id));
                }
                if map.contains_key("pages") {
                    return Err(anyhow!("Template {} uses pages[]; Rust engine currently requires root", template_id));
                }
                if map.contains_key("styles") || map.contains_key("theme") {
                    return Err(anyhow!("Template {} uses unsupported styles/theme", template_id));
                }
                for child in map.values() { walk(child, template_id)?; }
            }
            Value::Array(items) => for item in items { walk(item, template_id)?; },
            _ => {}
        }
        Ok(())
    }
    walk(value, template_id)
}

async fn database() -> Result<DatabaseConnection> {
    let config = CliConfig::default();
    let db = Database::connect(&config.db_url).await
        .with_context(|| format!("Unable to connect to {}", config.db_url))?;
    if db.get_database_backend() != DatabaseBackend::Sqlite {
        return Err(anyhow!("Template installation currently requires SQLite because the existing DocForge migration is SQLite-specific"));
    }
    Ok(db)
}

async fn organization_id(db: &DatabaseConnection) -> Result<String> {
    let row = db.query_one(Statement::from_string(
        DatabaseBackend::Sqlite,
        "SELECT id FROM organizations WHERE is_deleted = 0 ORDER BY created_at LIMIT 1".to_string(),
    )).await?;
    row.and_then(|value| value.try_get::<String>("", "id").ok())
        .ok_or_else(|| anyhow!("No organization found. Run `orbis init` first."))
}

async fn install(path: &Path, update: bool) -> Result<()> {
    let package = load_package(path)?;
    validate_package(&package)?;
    let db = database().await?;
    docforge_orbis::schema::migrate_template_schema(&db).await
        .context("Unable to create DocForge template tables")?;
    let org_id = organization_id(&db).await?;
    let txn = db.begin().await?;
    let now = chrono::Utc::now().to_rfc3339();
    let actor = "orbis-cli";

    let storage_root = storage_root(&CliConfig::default().db_url);
    fs::create_dir_all(storage_root.join("assets"))?;
    for asset in &package.manifest.assets {
        let source = package.root.join(&asset.path);
        let target = storage_root.join("assets").join(&asset.asset_id);
        let document_target = storage_root.join(&org_id).join("shared").join("templates").join(format!("{}.svg", asset.asset_id));
        if let Some(parent) = document_target.parent() { fs::create_dir_all(parent)?; }
        fs::copy(&source, &target).with_context(|| format!("Unable to copy asset {}", source.display()))?;
        fs::copy(&source, &document_target).with_context(|| format!("Unable to copy resource {}", source.display()))?;
        let metadata = serde_json::json!({
            "packageId": package.manifest.package_id,
            "packageVersion": package.manifest.package_version,
            "displayNameAr": package.manifest.display_name_ar,
            "nameAr": asset.name_ar,
            "sourcePath": asset.path,
        }).to_string();
        let sql = format!(
            "INSERT OR IGNORE INTO template_assets (id, org_id, name, asset_type, mime_type, file_size, storage_key, storage_provider, tags, is_system, created_by, created_at, updated_at) VALUES ('{}','{}','{}','{}','{}',{},'{}','local','{}',1,'{}','{}','{}')",
            escape(&asset.asset_id), escape(&org_id), escape(&asset.name), escape(&asset.asset_type), escape(&asset.mime_type), fs::metadata(&target)?.len(), escape(&asset.asset_id), escape(&metadata), actor, now, now
        );
        txn.execute_unprepared(&sql).await?;
        let document_sql = format!(
            "INSERT OR IGNORE INTO documents (id, org_id, uploaded_by, entity_type, entity_id, name, category, visibility, size, mime_type, extension, storage_provider, path, url, created_at, updated_at) VALUES ('{}','{}','{}','template','shared','{}','image','internal',{},'{}','svg','local','{}','/storage/{}/shared/templates/{}.svg','{}','{}')",
            escape(&asset.asset_id), escape(&org_id), actor, escape(&asset.name), fs::metadata(&source)?.len(), escape(&asset.mime_type), escape(&document_target.strip_prefix(&storage_root).unwrap_or(&document_target).to_string_lossy()), escape(&org_id), escape(&asset.asset_id), now, now
        );
        txn.execute_unprepared(&document_sql).await?;
    }

    for category in &package.manifest.categories {
        let sql = format!(
            "INSERT OR IGNORE INTO template_categories (id, org_id, name, name_ar, slug, sort_order, is_system, is_active, created_at, updated_at) VALUES ('{}','{}','{}',{},{}, {},1,1,'{}','{}')",
            escape(&category.id), escape(&org_id), escape(&category.name), optional(&category.name_ar), quote(&category.id), category.sort_order, now, now
        );
        txn.execute_unprepared(&sql).await?;
    }

    for (template, document_json) in &package.documents {
        let document: docforge_engine::model::TemplateDocument = serde_json::from_str(document_json)
            .with_context(|| format!("Document is not compatible with the Rust engine: {}", template.template_id))?;
        let page_setup = &document.meta.page_setup;
        let (page_width_mm, page_height_mm) = page_setup_dimensions(page_setup);
        let page_size = page_size_name(&page_setup.size);
        let orientation = orientation_name(&page_setup.orientation);
        let margins_json = serde_json::to_string(&page_setup.margins)?;
        let metadata = serde_json::json!({
            "packageId": package.manifest.package_id,
            "packageVersion": package.manifest.package_version,
            "source": "orbis-cli",
        }).to_string();
        let existing = txn.query_one(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "SELECT id, document_json, is_system FROM templates WHERE org_id = ? AND code = ? LIMIT 1",
            vec![org_id.clone().into(), template.code.clone().into()],
        )).await?;
        if let Some(row) = existing {
            let is_system = row.try_get::<i32>("", "is_system").unwrap_or(0);
            let current = row.try_get::<String>("", "document_json").unwrap_or_default();
            if update && is_system == 1 {
                let id = row.try_get::<String>("", "id")?;
                if current == *document_json {
                    txn.execute_unprepared(&format!(
                        "UPDATE templates SET page_size='{}', page_width_mm={}, page_height_mm={}, orientation='{}', margins_json='{}', meta='{}', updated_at='{}' WHERE id='{}' AND org_id='{}'",
                        page_size, page_width_mm, page_height_mm, orientation,
                        escape(&margins_json), escape(&metadata), now, escape(&id), escape(&org_id)
                    )).await?;
                } else {
                    txn.execute_unprepared(&format!(
                        "UPDATE templates SET document_json='{}', page_size='{}', page_width_mm={}, page_height_mm={}, orientation='{}', margins_json='{}', version=version+1, meta='{}', updated_at='{}' WHERE id='{}' AND org_id='{}'",
                        escape(document_json), page_size, page_width_mm, page_height_mm, orientation,
                        escape(&margins_json), escape(&metadata), now, escape(&id), escape(&org_id)
                    )).await?;
                }
            }
            continue;
        }

        let id = format!("{}-{}", package.manifest.package_id, template.template_id);
        let sql = format!(
            "INSERT INTO templates (id, org_id, category_id, code, name, name_ar, document_type, entity_type, page_size, page_width_mm, page_height_mm, orientation, margins_json, document_json, status, version, created_by, is_system, is_default, is_locked, locale, meta, created_at, updated_at) VALUES ('{}','{}','{}','{}','{}',{},{},{},'{}',{},{},'{}','{}','{}','published',1,'{}',1,{}, {},'{}','{}','{}','{}')",
            escape(&id), escape(&org_id), escape(&template.category), escape(&template.code), escape(&template.name), optional(&template.name_ar), quote(&template.document_type), quote(&template.entity_type), page_size, page_width_mm, page_height_mm, orientation, escape(&margins_json), escape(document_json), actor, bool_int(template.default), bool_int(template.locked), escape(template.locale.as_deref().unwrap_or("ar")), escape(&metadata), now, now
        );
        txn.execute_unprepared(&sql).await?;
        for asset in &package.manifest.assets {
            txn.execute_unprepared(&format!(
                "INSERT OR IGNORE INTO template_asset_usages (id, template_id, asset_id, usage_context, created_at) VALUES ('{}','{}','{}','header-logo','{}')",
                escape(&format!("{}-{}-{}", package.manifest.package_id, template.template_id, asset.asset_id)),
                escape(&id), escape(&asset.asset_id), now
            )).await?;
        }
    }

    txn.commit().await?;
    println!("{} template package {} {} into organization {}", if update { "Updated" } else { "Installed" }, package.manifest.package_id, package.manifest.package_version, org_id);
    Ok(())
}

async fn list_installed() -> Result<()> {
    let db = database().await?;
    let org_id = organization_id(&db).await?;
    let rows = db.query_all(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "SELECT code, name, status, version FROM templates WHERE org_id = ? AND deleted_at IS NULL ORDER BY code",
        vec![org_id.into()],
    )).await?;
    for row in rows {
        println!("{} | {} | {} | v{}", row.try_get::<Option<String>>("", "code")?.unwrap_or_default(), row.try_get::<String>("", "name")?, row.try_get::<String>("", "status")?, row.try_get::<i32>("", "version")?);
    }
    Ok(())
}

fn escape(value: &str) -> String { value.replace('\'', "''") }
fn quote(value: &str) -> String { format!("'{}'", escape(value)) }
fn optional(value: &Option<String>) -> String { value.as_deref().map(quote).unwrap_or_else(|| "NULL".to_string()) }
fn bool_int(value: bool) -> i32 { if value { 1 } else { 0 } }

fn page_size_name(size: &docforge_engine::model::document::PageSize) -> &'static str {
    match size {
        docforge_engine::model::document::PageSize::A3 => "A3",
        docforge_engine::model::document::PageSize::A4 => "A4",
        docforge_engine::model::document::PageSize::A5 => "A5",
        docforge_engine::model::document::PageSize::Letter => "Letter",
        docforge_engine::model::document::PageSize::Legal => "Legal",
        docforge_engine::model::document::PageSize::IdCard => "ID_CARD",
        docforge_engine::model::document::PageSize::Badge => "BADGE",
        docforge_engine::model::document::PageSize::Custom => "Custom",
    }
}

fn orientation_name(orientation: &docforge_engine::model::document::Orientation) -> &'static str {
    match orientation {
        docforge_engine::model::document::Orientation::Portrait => "portrait",
        docforge_engine::model::document::Orientation::Landscape => "landscape",
    }
}

fn page_setup_dimensions(page_setup: &docforge_engine::model::document::PageSetup) -> (String, String) {
    let (width, height) = match &page_setup.size {
        docforge_engine::model::document::PageSize::Custom => (
            page_setup.width.unwrap_or(210.0),
            page_setup.height.unwrap_or(297.0),
        ),
        _ => page_setup.size.dimensions(),
    };
    (width.to_string(), height.to_string())
}

fn storage_root(db_url: &str) -> PathBuf {
    let raw = db_url.strip_prefix("sqlite://").unwrap_or(db_url);
    let path = raw.split('?').next().unwrap_or(raw);
    Path::new(path).parent().unwrap_or_else(|| Path::new(".")).join("storage")
}
