//! Process lifecycle for the locally managed scheduling component.

use super::managed_components::ManagedComponentManager;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::time::Duration;
use tokio::process::Child;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

const COMPONENT_ID: &str = "orbis-solver";
const DEFAULT_PORT: u16 = 8081;
const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_millis(500);
const MAX_STARTUP_WAIT: Duration = Duration::from_secs(30);
const DEFAULT_IDLE_TIMEOUT_SECS: i64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarStatus {
    pub is_running: bool,
    pub is_healthy: bool,
    pub endpoint_url: String,
    pub pid: Option<u32>,
    pub active_jobs: usize,
    pub idle_seconds: i64,
}

pub struct SolverSidecarManager {
    child: Arc<Mutex<Option<Child>>>,
    active_job_ids: Arc<Mutex<HashSet<String>>>,
    endpoint_url: String,
    last_activity: Arc<AtomicI64>,
    active_jobs: Arc<AtomicUsize>,
    update_in_progress: Arc<std::sync::atomic::AtomicBool>,
    idle_timeout_secs: i64,
    http_client: reqwest::Client,
    components: ManagedComponentManager,
}

impl SolverSidecarManager {
    pub fn new(app_data_dir: &Path) -> Result<Self, String> {
        let http_client = reqwest::Client::builder()
            .timeout(HEALTH_CHECK_TIMEOUT)
            .build()
            .map_err(|error| format!("Could not create solver health client: {error}"))?;
        let components = ManagedComponentManager::new(
            app_data_dir,
            option_env!("ORBIS_SOLVER_MANIFEST_URL"),
            option_env!("ORBIS_SOLVER_PUBLIC_KEY_B64"),
            env!("CARGO_PKG_VERSION"),
        )?;

        let manager = Self {
            child: Arc::new(Mutex::new(None)),
            active_job_ids: Arc::new(Mutex::new(HashSet::new())),
            endpoint_url: format!("http://127.0.0.1:{DEFAULT_PORT}"),
            last_activity: Arc::new(AtomicI64::new(chrono::Utc::now().timestamp())),
            active_jobs: Arc::new(AtomicUsize::new(0)),
            update_in_progress: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            idle_timeout_secs: DEFAULT_IDLE_TIMEOUT_SECS,
            http_client,
            components,
        };
        manager.start_idle_watchdog();
        Ok(manager)
    }

    pub fn endpoint_url(&self) -> &str {
        &self.endpoint_url
    }

    pub fn mark_activity(&self) {
        self.last_activity
            .store(chrono::Utc::now().timestamp(), Ordering::Relaxed);
    }

    pub async fn begin_job(&self, job_id: &str) -> Result<(), String> {
        if self.update_in_progress.load(Ordering::Acquire) {
            return Err(
                "An Orbis update is being installed; try scheduling again shortly.".to_string(),
            );
        }
        let mut active_job_ids = self.active_job_ids.lock().await;
        if !active_job_ids.insert(job_id.to_string()) {
            return Err("This scheduling job is already active.".to_string());
        }
        self.active_jobs.fetch_add(1, Ordering::Relaxed);
        if self.update_in_progress.load(Ordering::Acquire) {
            active_job_ids.remove(job_id);
            self.decrement_active_job_count();
            return Err(
                "An Orbis update is being installed; try scheduling again shortly.".to_string(),
            );
        }
        self.mark_activity();
        Ok(())
    }

    pub async fn finish_job(&self, job_id: &str) {
        if self.active_job_ids.lock().await.remove(job_id) {
            self.decrement_active_job_count();
            self.mark_activity();
        }
    }

    pub async fn mark_job_activity(&self, job_id: &str) {
        if self.active_job_ids.lock().await.contains(job_id) {
            self.mark_activity();
        }
    }

    fn decrement_active_job_count(&self) {
        let mut current = self.active_jobs.load(Ordering::Relaxed);
        while current > 0 {
            match self.active_jobs.compare_exchange_weak(
                current,
                current - 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
    }

    pub fn begin_application_update(&self) -> Result<ApplicationUpdateGuard, String> {
        self.update_in_progress
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| "An Orbis update is already being installed.".to_string())?;
        let active_jobs = self.active_jobs.load(Ordering::Acquire);
        if active_jobs > 0 {
            self.update_in_progress.store(false, Ordering::Release);
            return Err(format!(
                "The update cannot restart Orbis while {active_jobs} scheduling job(s) are active."
            ));
        }
        Ok(ApplicationUpdateGuard(Arc::clone(&self.update_in_progress)))
    }

    pub async fn check_health(&self) -> bool {
        let health_url = format!("{}/health", self.endpoint_url);
        match self
            .http_client
            .get(&health_url)
            .timeout(HEALTH_CHECK_TIMEOUT)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                match response.json::<serde_json::Value>().await {
                    Ok(health) => {
                        health.get("status").and_then(serde_json::Value::as_str) == Some("healthy")
                            && health.get("service").and_then(serde_json::Value::as_str)
                                == Some("orbis-solver")
                    }
                    Err(error) => {
                        debug!(%error, "Solver health response was invalid");
                        false
                    }
                }
            }
            Ok(response) => {
                debug!(status = %response.status(), "Solver health check failed");
                false
            }
            Err(error) => {
                debug!(%error, "Solver health check failed");
                false
            }
        }
    }

    pub async fn ensure_running(&self) -> Result<(), String> {
        self.mark_activity();
        if self.check_health().await {
            return Ok(());
        }

        let mut child_guard = self.child.lock().await;
        if self.check_health().await {
            return Ok(());
        }
        if let Some(mut old_child) = child_guard.take() {
            if let Err(error) = old_child.kill().await {
                warn!(%error, "Could not terminate an unhealthy solver process");
            }
        }

        let target = solver_target();
        let (version, executable_path) = if let Some(path) = find_development_binary() {
            ("development".to_string(), path)
        } else {
            let target = target.ok_or_else(|| {
                "The scheduling engine is not available for this platform.".to_string()
            })?;
            let component = self
                .components
                .ensure_installed(COMPONENT_ID, target)
                .await?;
            (component.version, component.executable_path)
        };
        match self.start_process(&executable_path).await {
            Ok(child) => {
                *child_guard = Some(child);
                Ok(())
            }
            Err(start_error) if version != "development" => {
                let target = target.ok_or(start_error.clone())?;
                let previous = self
                    .components
                    .previous_verified(COMPONENT_ID, target, &version)
                    .await?;
                let Some(previous) = previous else {
                    return Err(start_error);
                };
                warn!(
                    failed_version = %version,
                    fallback_version = %previous.version,
                    %start_error,
                    "Scheduling engine startup failed; trying the previous verified version"
                );
                let child = self.start_process(&previous.executable_path).await?;
                *child_guard = Some(child);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    async fn start_process(&self, executable_path: &Path) -> Result<Child, String> {
        info!(path = %executable_path.display(), "Starting the Orbis scheduling engine");
        let mut command = tokio::process::Command::new(executable_path);
        command
            .env("QUARKUS_HTTP_PORT", DEFAULT_PORT.to_string())
            .env("QUARKUS_HTTP_HOST", "127.0.0.1")
            .env("QUARKUS_LOG_LEVEL", "INFO")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        #[cfg(windows)]
        command.creation_flags(0x08000000);

        let mut child = command.spawn().map_err(|error| {
            format!(
                "Could not start the scheduling engine ({}): {error}",
                executable_path.display()
            )
        })?;
        let start = std::time::Instant::now();
        while start.elapsed() < MAX_STARTUP_WAIT {
            if self.check_health().await {
                info!("Scheduling engine is ready after {:?}", start.elapsed());
                return Ok(child);
            }
            if let Some(status) = child
                .try_wait()
                .map_err(|error| format!("Could not inspect scheduling engine process: {error}"))?
            {
                return Err(format!(
                    "The scheduling engine exited during startup with status {status}."
                ));
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        if let Err(error) = child.kill().await {
            warn!(%error, "Could not terminate scheduling engine after startup timeout");
        }
        Err("The scheduling engine did not become ready in time.".to_string())
    }

    pub async fn stop(&self) {
        let mut child_guard = self.child.lock().await;
        if let Some(mut child) = child_guard.take() {
            if let Err(error) = child.kill().await {
                warn!(%error, "Could not stop the scheduling engine");
            } else {
                info!("Stopped the idle scheduling engine");
            }
        }
    }

    pub async fn status(&self) -> SidecarStatus {
        let is_healthy = self.check_health().await;
        let child_guard = self.child.lock().await;
        let is_running = child_guard.is_some() || is_healthy;
        let pid = child_guard.as_ref().and_then(Child::id);
        drop(child_guard);

        let idle_seconds =
            (chrono::Utc::now().timestamp() - self.last_activity.load(Ordering::Relaxed)).max(0);
        SidecarStatus {
            is_running,
            is_healthy,
            endpoint_url: self.endpoint_url.clone(),
            pid,
            active_jobs: self.active_jobs.load(Ordering::Relaxed),
            idle_seconds,
        }
    }

    fn start_idle_watchdog(&self) {
        let child = Arc::clone(&self.child);
        let last_activity = Arc::clone(&self.last_activity);
        let active_jobs = Arc::clone(&self.active_jobs);
        let timeout_secs = self.idle_timeout_secs;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;
                if active_jobs.load(Ordering::Relaxed) > 0 {
                    continue;
                }

                let now = chrono::Utc::now().timestamp();
                let last = last_activity.load(Ordering::Relaxed);
                if now - last <= timeout_secs {
                    continue;
                }

                let mut guard = child.lock().await;
                if let Some(mut process) = guard.take() {
                    info!(
                        "Stopping the scheduling engine after {} seconds of inactivity",
                        now - last
                    );
                    if let Err(error) = process.kill().await {
                        warn!(%error, "Could not stop the idle scheduling engine");
                    }
                }
            }
        });
    }
}

pub struct ApplicationUpdateGuard(Arc<std::sync::atomic::AtomicBool>);

impl Drop for ApplicationUpdateGuard {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

fn solver_target() -> Option<&'static str> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    return Some("x86_64-pc-windows-msvc");
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    return Some("aarch64-apple-darwin");
    #[allow(unreachable_code)]
    None
}

fn find_development_binary() -> Option<PathBuf> {
    if !cfg!(debug_assertions) {
        return None;
    }

    let target_file = if cfg!(windows) {
        "runner.exe"
    } else {
        "runner"
    };
    let directories = [
        PathBuf::from("apps/solver-service/target"),
        PathBuf::from("../solver-service/target"),
        PathBuf::from("../../apps/solver-service/target"),
    ];
    let mut candidates = Vec::new();
    for directory in directories {
        let Ok(entries) = std::fs::read_dir(directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if name.starts_with("orbis-solver-") && name.ends_with(target_file) && path.is_file() {
                candidates.push(path);
            }
        }
    }
    candidates.sort();
    candidates.pop()
}

impl Drop for SolverSidecarManager {
    fn drop(&mut self) {
        if let Ok(mut guard) = self.child.try_lock()
            && let Some(mut child) = guard.take()
        {
            let _ = child.start_kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn application_updates_wait_for_jobs_and_reject_new_jobs() {
        let data_dir = std::env::temp_dir().join(format!("orbis-sidecar-{}", uuid::Uuid::new_v4()));
        let manager = SolverSidecarManager::new(&data_dir).expect("manager");

        manager.begin_job("job-1").await.expect("start job");
        assert!(manager.begin_application_update().is_err());
        manager.finish_job("job-1").await;
        manager.finish_job("job-1").await;

        let update_guard = manager.begin_application_update().expect("begin update");
        assert!(manager.begin_job("job-2").await.is_err());
        drop(update_guard);
        manager
            .begin_job("job-2")
            .await
            .expect("start job after update");
        manager.finish_job("job-2").await;
    }
}
