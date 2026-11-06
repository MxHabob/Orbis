//! System Tauri IPC commands — system status, health, readiness, entitlements,
//! Application metadata, GitHub releases, and signed update management.

use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

// ═══════════════════════════════════════════════════════════════
// DTOs
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub build_profile: String,
    pub target_os: String,
    pub target_arch: String,
}

/// Structured release-note payload for the desktop app.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReleaseNoteItem {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReleaseNoteSection {
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub items: Vec<ReleaseNoteItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReleaseNotesRelease {
    pub version: String,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub sections: Vec<ReleaseNoteSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReleaseNotesDocument {
    #[serde(default)]
    pub schema_version: u32,
    #[serde(default)]
    pub releases: Vec<ReleaseNotesRelease>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReleaseNotesState {
    #[serde(default)]
    pub schema_version: u32,
    #[serde(default)]
    pub last_seen_version: Option<String>,
    #[serde(default)]
    pub initialized: bool,
}

/// Published release data used by the changelog and onboarding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseChangelogItem {
    pub version: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub published_at: Option<String>,
    pub body: Option<String>,
    pub sections: Vec<ReleaseNoteSection>,
    pub is_current: bool,
    pub is_latest: bool,
    pub html_url: String,
    pub images: Vec<String>,
    pub resources: Vec<ReleaseResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseResource {
    pub name: String,
    pub url: String,
    pub content_type: String,
    pub size: u64,
}

/// Structured update check result with GitHub Releases comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResult {
    pub current_version: String,
    pub latest_version: String,
    pub has_update: bool,
    pub release_notes: Option<String>,
    pub release_date: Option<String>,
    pub release_url: Option<String>,
    pub channel: String,
    pub status: String,
    pub error_message: Option<String>,
}

pub fn is_staging_build(identifier: &str) -> bool {
    identifier.ends_with(".staging")
}

fn semver(version: &str) -> Option<(u64, u64, u64)> {
    let version = version.strip_prefix('v').unwrap_or(version);
    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

fn is_allowed_section_type(value: &str) -> bool {
    matches!(
        value,
        "feature" | "improvement" | "fix" | "security" | "breaking" | "technical"
    )
}

fn normalize_release_notes(doc: &mut ReleaseNotesDocument) {
    let mut releases = std::mem::take(&mut doc.releases);
    releases.sort_by(|left, right| {
        let left_version = semver(&left.version).unwrap_or((0, 0, 0));
        let right_version = semver(&right.version).unwrap_or((0, 0, 0));
        right_version.cmp(&left_version)
    });

    let mut seen = std::collections::BTreeSet::new();
    releases.retain(|release| {
        if !semver(&release.version).is_some() || !seen.insert(release.version.clone()) {
            return false;
        }
        true
    });

    doc.releases = releases;
}

fn release_markdown(release: &ReleaseNotesRelease) -> String {
    if release.sections.is_empty() {
        return release.title.clone();
    }

    let mut markdown = String::new();
    for section in &release.sections {
        let type_label = section.r#type.as_str();
        if !is_allowed_section_type(type_label) {
            continue;
        }
        markdown.push_str(&format!("## {}\n\n", section.title));
        for item in &section.items {
            markdown.push_str(&format!("- **{}**: {}\n", item.title, item.description));
        }
        markdown.push('\n');
    }
    markdown.trim().to_string()
}

fn release_notes_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let resource_dir = app
        .path()
        .resolve("release-notes.json", tauri::path::BaseDirectory::Resource)
        .map_err(|error| error.to_string())?;
    Ok(resource_dir)
}

fn release_state_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    Ok(data_dir.join("release-notes-state.json"))
}

fn read_release_notes_state(app: &AppHandle) -> Result<ReleaseNotesState, String> {
    let path = release_state_path(app)?;
    if !path.exists() {
        return Ok(ReleaseNotesState {
            schema_version: 1,
            last_seen_version: None,
            initialized: false,
        });
    }

    let value = std::fs::read_to_string(&path)
        .map_err(|error| format!("Failed to read release notes state: {error}"))?;
    let state: ReleaseNotesState = serde_json::from_str(&value).unwrap_or(ReleaseNotesState {
        schema_version: 1,
        last_seen_version: None,
        initialized: false,
    });
    Ok(state)
}

fn write_release_notes_state(app: &AppHandle, state: &ReleaseNotesState) -> Result<(), String> {
    let path = release_state_path(app)?;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    std::fs::create_dir_all(parent)
        .map_err(|error| format!("Failed to create release notes state directory: {error}"))?;
    let json = serde_json::to_string_pretty(state)
        .map_err(|error| format!("Failed to serialize release notes state: {error}"))?;
    std::fs::write(path, json)
        .map_err(|error| format!("Failed to write release notes state: {error}"))?;
    Ok(())
}

fn load_release_notes_document(app: &AppHandle) -> Result<Vec<ReleaseChangelogItem>, String> {
    let source_path = release_notes_path(app)?;
    let json = std::fs::read_to_string(source_path)
        .map_err(|error| format!("Failed to read release notes catalog: {error}"))?;
    let mut document: ReleaseNotesDocument = serde_json::from_str(&json)
        .map_err(|error| format!("Failed to parse release notes catalog: {error}"))?;

    if document.schema_version != 1 {
        document.schema_version = 1;
    }
    normalize_release_notes(&mut document);

    let installed = semver(&app.package_info().version.to_string()).unwrap_or((0, 0, 0));
    let mut result = Vec::new();

    for (index, release) in document.releases.into_iter().enumerate() {
        if semver(&release.version)
            .map(|version| version > installed)
            .unwrap_or(false)
        {
            continue;
        }

        let version = release.version.clone();
        let title = release.title.clone();
        let subtitle = release.subtitle.clone();
        let published_at = release.release_date.clone();
        let sections = release.sections.clone();
        let body = release_markdown(&release);

        result.push(ReleaseChangelogItem {
            version: version.clone(),
            title,
            subtitle,
            published_at,
            body: Some(body),
            sections,
            is_current: version == app.package_info().version.to_string(),
            is_latest: index == 0,
            html_url: String::new(),
            images: Vec::new(),
            resources: Vec::new(),
        });
    }
    Ok(result)
}

// ═══════════════════════════════════════════════════════════════
// System IPC Handlers
// ═══════════════════════════════════════════════════════════════

/// Get the application identity and target platform for diagnostics and UI.
#[tauri::command]
pub async fn system_get_app_info(app: AppHandle) -> Result<AppInfo, String> {
    let package_info = app.package_info();
    Ok(AppInfo {
        name: package_info.name.clone(),
        version: package_info.version.to_string(),
        build_profile: if cfg!(debug_assertions) {
            "debug".to_string()
        } else {
            "release".to_string()
        },
        target_os: std::env::consts::OS.to_string(),
        target_arch: std::env::consts::ARCH.to_string(),
    })
}

/// Fetch the local release notes catalog and filter it to the installed app version.
#[tauri::command]
pub async fn system_get_changelog(app: AppHandle) -> Result<Vec<ReleaseChangelogItem>, String> {
    load_release_notes_document(&app)
}

#[tauri::command]
pub async fn system_get_release_notes_state(app: AppHandle) -> Result<ReleaseNotesState, String> {
    read_release_notes_state(&app)
}

#[tauri::command]
pub async fn system_acknowledge_release_notes(
    app: AppHandle,
    version: String,
) -> Result<(), String> {
    let mut state = read_release_notes_state(&app)?;
    state.schema_version = 1;
    state.last_seen_version = Some(version);
    state.initialized = true;
    write_release_notes_state(&app, &state)
}

/// Check for a signed update using the configured GitHub Releases endpoint.
#[tauri::command]
pub async fn system_check_for_updates(app: AppHandle) -> Result<UpdateCheckResult, String> {
    use tauri_plugin_updater::UpdaterExt;

    let current_version = app.package_info().version.to_string();
    if is_staging_build(&app.config().identifier) {
        return Ok(UpdateCheckResult {
            current_version: current_version.clone(),
            latest_version: current_version,
            has_update: false,
            release_notes: None,
            release_date: None,
            release_url: None,
            channel: "staging".to_string(),
            status: "unavailable".to_string(),
            error_message: Some("Updates are not configured for staging builds.".to_string()),
        });
    }

    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater.check().await.map_err(|e| e.to_string())?;

    Ok(match update {
        Some(update) => UpdateCheckResult {
            current_version,
            latest_version: update.version.to_string(),
            has_update: true,
            release_notes: update.body.clone(),
            release_date: update.date.map(|date| date.to_string()),
            release_url: None,
            channel: "stable".to_string(),
            status: "update_available".to_string(),
            error_message: None,
        },
        None => UpdateCheckResult {
            current_version: current_version.clone(),
            latest_version: current_version,
            has_update: false,
            release_notes: None,
            release_date: None,
            release_url: None,
            channel: "stable".to_string(),
            status: "up_to_date".to_string(),
            error_message: None,
        },
    })
}

#[tauri::command]
pub async fn system_install_update(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_updater::UpdaterExt;

    if is_staging_build(&app.config().identifier) {
        return Err("Updates are not configured for staging builds.".to_string());
    }

    let update = app
        .updater()
        .map_err(|error| format!("Failed to initialize updater: {error}"))?
        .check()
        .await
        .map_err(|error| format!("Failed to check for updates: {error}"))?
        .ok_or_else(|| "No update is currently available.".to_string())?;

    update
        .download_and_install(
            |_chunk_length, _content_length| {},
            || tracing::info!("Update download finished"),
        )
        .await
        .map_err(|error| format!("Failed to download or install update: {error}"))?;

    app.restart();
}

/// System health check.
#[tauri::command]
pub async fn system_health(
    state: State<'_, AppState>,
) -> Result<orbis_app::services::HealthResponse, String> {
    Ok(state.system_service.health().await)
}

/// Check if any organization exists in the system.
#[tauri::command]
pub async fn system_has_organization(state: State<'_, AppState>) -> Result<bool, String> {
    state
        .system_service
        .has_organization()
        .await
        .map_err(|e| e.to_string())
}

/// Get organization readiness status.
#[tauri::command]
pub async fn system_get_readiness(
    state: State<'_, AppState>,
) -> Result<orbis_app::services::OrganizationReadiness, String> {
    state
        .system_service
        .get_readiness()
        .await
        .map_err(|e| e.to_string())
}

/// Get system entitlement summary.
#[tauri::command]
pub async fn system_get_entitlement(
    state: State<'_, AppState>,
) -> Result<orbis_domain::entities::entitlement::EntitlementResult, String> {
    state
        .entitlement_provider
        .check_entitlement("orbis_desktop", "default")
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::is_staging_build;

    #[test]
    fn staging_identifier_disables_stable_updater() {
        assert!(is_staging_build("com.mulverse.orbis.staging"));
        assert!(!is_staging_build("com.mulverse.orbis"));
        assert!(!is_staging_build("com.mulverse.orbis.dev"));
    }
}
