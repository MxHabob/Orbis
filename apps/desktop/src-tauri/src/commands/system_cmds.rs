//! System Tauri IPC commands — system status, health, readiness, entitlements,
//! Application metadata, GitHub releases, and signed update management.

use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

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

/// Published release data used by the changelog and onboarding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseChangelogItem {
    pub version: String,
    pub title: String,
    pub published_at: Option<String>,
    pub body: Option<String>,
    pub is_current: bool,
    pub is_latest: bool,
    pub html_url: String,
    pub images: Vec<String>,
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
    pub status: String, // "up_to_date" | "update_available" | "checking_failed"
    pub error_message: Option<String>,
}

// ═══════════════════════════════════════════════════════════════
// GitHub Release & Changelog Helpers
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
struct GitHubReleaseDto {
    tag_name: String,
    name: Option<String>,
    published_at: Option<String>,
    body: Option<String>,
    html_url: String,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    assets: Vec<GitHubReleaseAssetDto>,
}

#[derive(Debug, Deserialize)]
struct GitHubReleaseAssetDto {
    browser_download_url: String,
    content_type: String,
}

/// Fetch published stable releases from GitHub.
pub async fn fetch_github_changelog(
    current_version: &str,
) -> Result<Vec<ReleaseChangelogItem>, String> {
    let client = reqwest::Client::builder()
        .user_agent("Orbis")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|error| format!("Failed to create GitHub client: {error}"))?;

    let releases = client
        .get("https://api.github.com/repos/MxHabob/Orbis/releases?per_page=50")
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .map_err(|error| format!("Failed to fetch GitHub releases: {error}"))?
        .error_for_status()
        .map_err(|error| format!("GitHub releases request failed: {error}"))?
        .json::<Vec<GitHubReleaseDto>>()
        .await
        .map_err(|error| format!("Failed to parse GitHub releases: {error}"))?;

    Ok(releases
        .into_iter()
        .filter(|release| !release.prerelease)
        .enumerate()
        .map(|(index, release)| {
            let version = release.tag_name.trim_start_matches('v').to_string();
            let images = release
                .assets
                .into_iter()
                .filter(|asset| asset.content_type.starts_with("image/"))
                .filter_map(|asset| {
                    let url = reqwest::Url::parse(&asset.browser_download_url).ok()?;
                    (url.scheme() == "https" && url.host_str() == Some("github.com"))
                        .then(|| url.to_string())
                })
                .take(6)
                .collect();

            ReleaseChangelogItem {
                is_current: version == current_version,
                is_latest: index == 0,
                version,
                title: release
                    .name
                    .filter(|name| !name.is_empty())
                    .unwrap_or(release.tag_name),
                published_at: release.published_at,
                body: release.body,
                html_url: release.html_url,
                images,
            }
        })
        .collect())
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

/// Fetch real release changelog entries from GitHub Releases.
#[tauri::command]
pub async fn system_get_changelog(app: AppHandle) -> Result<Vec<ReleaseChangelogItem>, String> {
    let current_version = app.package_info().version.to_string();
    fetch_github_changelog(&current_version).await
}

/// Check for a signed update using the configured Tauri updater endpoint.
#[tauri::command]
pub async fn system_check_for_updates(app: AppHandle) -> Result<UpdateCheckResult, String> {
    use tauri_plugin_updater::UpdaterExt;

    let current_version = app.package_info().version.to_string();
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater.check().await.map_err(|e| e.to_string())?;

    Ok(match update {
        Some(update) => UpdateCheckResult {
            current_version,
            latest_version: update.version.to_string(),
            has_update: true,
            release_notes: update.body.clone(),
            release_date: update.date.map(|date| date.to_string()),
            release_url: Some("https://github.com/MxHabob/Orbis/releases/latest".to_string()),
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
