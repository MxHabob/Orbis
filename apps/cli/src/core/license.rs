use anyhow::{Context, Result, anyhow};
use orbis_app::ports::license::LicenseRuntime;
use orbis_domain::entities::license::{LicenseSnapshot, LicenseStateDto, TrialOrganizationProfile};
use orbis_infra::adapters::license_runtime::MulverseLicenseRuntime;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::config::{CliState, DeployTarget, native_license_dir};
use crate::core::docker;

const SNAPSHOT_FILE: &str = "license_snapshot.json";
const INSTALLATION_ID_FILE: &str = ".installation-id";

/// Prepares a local staging directory for Docker (pulling existing license files from the volume),
/// or returns the host `ORBIS_LICENSE_DATA_DIR` for Native deployments.
fn resolve_working_license_dir(state: &CliState) -> Result<(PathBuf, bool)> {
    if state.target == DeployTarget::Native {
        let dir = native_license_dir();
        fs::create_dir_all(&dir)?;
        return Ok((dir, false));
    }

    let staging = CliState::state_dir().join("license_staging");
    if staging.exists() {
        let _ = fs::remove_dir_all(&staging);
    }
    fs::create_dir_all(&staging)?;

    let _ = docker::copy_file_from_volume(
        docker::LICENSE_VOLUME,
        "/var/lib/orbis/license",
        SNAPSHOT_FILE,
        &staging.join(SNAPSHOT_FILE),
        &state.api_image,
    );
    let _ = docker::copy_file_from_volume(
        docker::LICENSE_VOLUME,
        "/var/lib/orbis/license",
        INSTALLATION_ID_FILE,
        &staging.join(INSTALLATION_ID_FILE),
        &state.api_image,
    );

    Ok((staging, true))
}

fn push_staging_to_docker(state: &CliState, dir: &Path) -> Result<()> {
    let snapshot_path = dir.join(SNAPSHOT_FILE);
    if snapshot_path.exists() {
        docker::write_file_to_volume(
            docker::LICENSE_VOLUME,
            "/var/lib/orbis/license",
            &snapshot_path,
            SNAPSHOT_FILE,
            &state.api_image,
        )?;
    }

    let id_path = dir.join(INSTALLATION_ID_FILE);
    if id_path.exists() {
        docker::write_file_to_volume(
            docker::LICENSE_VOLUME,
            "/var/lib/orbis/license",
            &id_path,
            INSTALLATION_ID_FILE,
            &state.api_image,
        )?;
    }

    if docker::container_running(&state.container_name) {
        docker::restart_api(&state.container_name)?;
    }
    Ok(())
}

fn build_runtime(dir: &Path) -> Result<MulverseLicenseRuntime> {
    let mulverse_url = std::env::var("MULVERSE_API_URL").ok();
    MulverseLicenseRuntime::new(dir, env!("CARGO_PKG_VERSION"), mulverse_url.as_deref())
        .map_err(|e| anyhow!("Failed to create license runtime: {e}"))
}

pub async fn get_status(state: &CliState) -> Result<LicenseStateDto> {
    let (dir, is_staging) = resolve_working_license_dir(state)?;
    let runtime = build_runtime(&dir)?;
    runtime.initialize().await.map_err(|e| anyhow!("{e}"))?;
    let dto = runtime.current_state_dto();
    if is_staging {
        let _ = fs::remove_dir_all(&dir);
    }
    Ok(dto)
}

/// Imports an existing `license_snapshot.json` (e.g. from Windows Desktop) and syncs its
/// `installation_id` so the target container/host verifies it cleanly without mismatch.
pub async fn import_snapshot_file(state: &CliState, source_file: &Path) -> Result<LicenseStateDto> {
    let raw = fs::read_to_string(source_file)
        .with_context(|| format!("Cannot read license file {}", source_file.display()))?;
    let snapshot: LicenseSnapshot = serde_json::from_str(&raw)
        .with_context(|| format!("Invalid license_snapshot.json in {}", source_file.display()))?;

    let (dir, is_staging) = resolve_working_license_dir(state)?;
    fs::write(dir.join(SNAPSHOT_FILE), &raw)?;
    if !snapshot.installation_id.trim().is_empty() {
        fs::write(dir.join(INSTALLATION_ID_FILE), snapshot.installation_id.trim())?;
    }

    let runtime = build_runtime(&dir)?;
    runtime.initialize().await.map_err(|e| anyhow!("{e}"))?;
    let dto = runtime.current_state_dto();

    if is_staging {
        push_staging_to_docker(state, &dir)?;
        let _ = fs::remove_dir_all(&dir);
    }

    Ok(dto)
}

fn default_trial_profile(org_id: uuid::Uuid, name: String) -> TrialOrganizationProfile {
    TrialOrganizationProfile {
        id: org_id,
        name,
        name_ar: None,
        slug: "orbis".to_string(),
        institution_type: "school".to_string(),
        organization_size: None,
        educational_track: None,
        country_code: None,
        country_name: None,
        region_name: None,
        city: None,
    }
}

pub async fn activate_key(
    state: &CliState,
    product_key: &str,
    org_id: uuid::Uuid,
    org_name: Option<String>,
) -> Result<LicenseStateDto> {
    let (dir, is_staging) = resolve_working_license_dir(state)?;
    let runtime = build_runtime(&dir)?;
    runtime.initialize().await.map_err(|e| anyhow!("{e}"))?;

    match runtime.activate_product_key(product_key, org_id).await {
        Ok(_) => {}
        Err(orbis_domain::errors::DomainError::LicenseKeyRequiresClaim) => {
            let profile = default_trial_profile(
                org_id,
                org_name.unwrap_or_else(|| "Orbis Organization".to_string()),
            );
            runtime
                .redeem_product_key(product_key, profile)
                .await
                .map_err(|e| anyhow!("{e}"))?;
        }
        Err(e) => return Err(anyhow!("{e}")),
    }

    let dto = runtime.current_state_dto();
    if is_staging {
        push_staging_to_docker(state, &dir)?;
        let _ = fs::remove_dir_all(&dir);
    }
    Ok(dto)
}

pub async fn start_trial(state: &CliState, org_id: uuid::Uuid, org_name: &str) -> Result<LicenseStateDto> {
    let (dir, is_staging) = resolve_working_license_dir(state)?;
    let runtime = build_runtime(&dir)?;
    runtime.initialize().await.map_err(|e| anyhow!("{e}"))?;

    let profile = default_trial_profile(org_id, org_name.to_string());
    runtime.start_trial(profile).await.map_err(|e| anyhow!("{e}"))?;
    let dto = runtime.current_state_dto();

    if is_staging {
        push_staging_to_docker(state, &dir)?;
        let _ = fs::remove_dir_all(&dir);
    }
    Ok(dto)
}

pub async fn refresh(state: &CliState, org_id: Option<uuid::Uuid>) -> Result<LicenseStateDto> {
    let (dir, is_staging) = resolve_working_license_dir(state)?;
    let runtime = build_runtime(&dir)?;
    runtime.initialize().await.map_err(|e| anyhow!("{e}"))?;
    runtime.refresh(org_id).await.map_err(|e| anyhow!("{e}"))?;
    let dto = runtime.current_state_dto();

    if is_staging {
        push_staging_to_docker(state, &dir)?;
        let _ = fs::remove_dir_all(&dir);
    }
    Ok(dto)
}

pub async fn deactivate(state: &CliState) -> Result<LicenseStateDto> {
    let (dir, is_staging) = resolve_working_license_dir(state)?;
    let runtime = build_runtime(&dir)?;
    runtime.initialize().await.map_err(|e| anyhow!("{e}"))?;
    runtime.deactivate().await.map_err(|e| anyhow!("{e}"))?;
    let dto = runtime.current_state_dto();

    if is_staging {
        push_staging_to_docker(state, &dir)?;
        let _ = fs::remove_dir_all(&dir);
    }
    Ok(dto)
}
