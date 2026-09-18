//! Solver Sidecar Manager
//!
//! Manages the lifecycle of the embedded Timefold `solver-service` native binary.
//!
//! Architectural Principles:
//! 1. **Zero-Resource Idle**: The sidecar is spawned on-demand when a timetable or exam
//!    solving operation begins, and automatically stopped after an idle period (e.g., 5 min)
//!    to conserve RAM and CPU.
//! 2. **Process Sandboxing**: On Windows, the process is spawned with `CREATE_NO_WINDOW`
//!    to run silently without popup windows.
//! 3. **Clean Shutdown**: Guarantees termination when the main Orbis Desktop application exits.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::process::Child;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

const DEFAULT_PORT: u16 = 8081;
const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_millis(500);
const MAX_STARTUP_WAIT: Duration = Duration::from_secs(8);
const DEFAULT_IDLE_TIMEOUT_SECS: i64 = 300; // 5 minutes

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
    endpoint_url: String,
    last_activity: Arc<AtomicI64>,
    active_jobs: Arc<AtomicUsize>,
    idle_timeout_secs: i64,
    http_client: reqwest::Client,
}

impl SolverSidecarManager {
    pub fn new() -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(HEALTH_CHECK_TIMEOUT)
            .build()
            .unwrap_or_default();

        let manager = Self {
            child: Arc::new(Mutex::new(None)),
            endpoint_url: format!("http://127.0.0.1:{}", DEFAULT_PORT),
            last_activity: Arc::new(AtomicI64::new(chrono::Utc::now().timestamp())),
            active_jobs: Arc::new(AtomicUsize::new(0)),
            idle_timeout_secs: DEFAULT_IDLE_TIMEOUT_SECS,
            http_client,
        };

        manager.start_idle_watchdog();
        manager
    }

    pub fn endpoint_url(&self) -> &str {
        &self.endpoint_url
    }

    pub fn mark_activity(&self) {
        self.last_activity.store(chrono::Utc::now().timestamp(), Ordering::Relaxed);
    }

    pub fn increment_active_jobs(&self) {
        self.active_jobs.fetch_add(1, Ordering::Relaxed);
        self.mark_activity();
    }

    pub fn decrement_active_jobs(&self) {
        let _ = self.active_jobs.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |val| {
            Some(val.saturating_sub(1))
        });
        self.mark_activity();
    }

    /// Check whether the sidecar HTTP endpoint responds with 200 OK to `/health`
    pub async fn check_health(&self) -> bool {
        let health_url = format!("{}/health", self.endpoint_url);
        match self.http_client.get(&health_url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Ensure the sidecar process is running and healthy. If not, spawn it.
    pub async fn ensure_running(&self) -> Result<(), String> {
        self.mark_activity();

        // 1. If already healthy and responding, we are good
        if self.check_health().await {
            debug!("Solver sidecar is already healthy and reachable");
            return Ok(());
        }

        let mut child_guard = self.child.lock().await;

        // Double check under lock
        if self.check_health().await {
            return Ok(());
        }

        // 2. Kill any stale child process
        if let Some(mut old_child) = child_guard.take() {
            let _ = old_child.kill().await;
        }

        // 3. Locate the sidecar executable
        let exe_path = Self::find_sidecar_binary()
            .ok_or_else(|| "Solver sidecar executable not found in bundle or workspace".to_string())?;

        info!(path = ?exe_path, "Spawning solver sidecar native binary");

        // 4. Spawn process
        let mut cmd = tokio::process::Command::new(&exe_path);
        cmd.env("QUARKUS_HTTP_PORT", DEFAULT_PORT.to_string())
            .env("QUARKUS_LOG_LEVEL", "INFO")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        #[cfg(windows)]
        {
            // 0x08000000 = CREATE_NO_WINDOW
            cmd.creation_flags(0x08000000);
        }

        let child = cmd.spawn().map_err(|e| {
            format!("Failed to spawn solver sidecar ({}): {e}", exe_path.display())
        })?;

        *child_guard = Some(child);
        drop(child_guard);

        // 5. Poll health check until ready
        let start = std::time::Instant::now();
        while start.elapsed() < MAX_STARTUP_WAIT {
            if self.check_health().await {
                info!("Solver sidecar started and ready in {:?}", start.elapsed());
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        warn!("Solver sidecar spawned but did not respond to health check within timeout");
        Err("Solver sidecar startup timeout".to_string())
    }

    /// Gracefully stop the sidecar process to release 100% of memory and CPU
    pub async fn stop(&self) {
        let mut child_guard = self.child.lock().await;
        if let Some(mut child) = child_guard.take() {
            info!("Stopping solver sidecar to free desktop resources (idle timeout)");
            let _ = child.kill().await;
        }
    }

    /// Returns the current runtime status of the sidecar
    pub async fn status(&self) -> SidecarStatus {
        let is_healthy = self.check_health().await;
        let child_guard = self.child.lock().await;
        let is_running = child_guard.is_some() || is_healthy;
        let pid = child_guard.as_ref().and_then(|c| c.id());
        drop(child_guard);

        let now = chrono::Utc::now().timestamp();
        let last = self.last_activity.load(Ordering::Relaxed);
        let idle_seconds = (now - last).max(0);

        SidecarStatus {
            is_running,
            is_healthy,
            endpoint_url: self.endpoint_url.clone(),
            pid,
            active_jobs: self.active_jobs.load(Ordering::Relaxed),
            idle_seconds,
        }
    }

    /// Background task that terminates the sidecar when idle and no jobs are active
    fn start_idle_watchdog(&self) {
        let child_arc = Arc::clone(&self.child);
        let last_activity_arc = Arc::clone(&self.last_activity);
        let active_jobs_arc = Arc::clone(&self.active_jobs);
        let timeout_secs = self.idle_timeout_secs;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;

                let active = active_jobs_arc.load(Ordering::Relaxed);
                if active > 0 {
                    continue; // Do not shutdown while active solving jobs exist
                }

                let now = chrono::Utc::now().timestamp();
                let last = last_activity_arc.load(Ordering::Relaxed);
                if now - last > timeout_secs {
                    let mut guard = child_arc.lock().await;
                    if let Some(mut child) = guard.take() {
                        info!("Watchdog: terminating idle solver sidecar after {}s inactivity", now - last);
                        let _ = child.kill().await;
                    }
                }
            }
        });
    }

    /// Searches standard Tauri bundle and development locations for the binary
    fn find_sidecar_binary() -> Option<PathBuf> {
        let mut candidates = Vec::new();

        // 1. Current executable directory (Tauri bundled externalBin)
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(exe_dir) = current_exe.parent() {
                // Windows bundle pattern
                candidates.push(exe_dir.join("solver-service.exe"));
                candidates.push(exe_dir.join("solver-service"));
                candidates.push(exe_dir.join("binaries").join("solver-service.exe"));
                candidates.push(exe_dir.join("binaries").join("solver-service"));
            }
        }

        // 2. Relative binaries folder (Tauri target triple convention in dev/build)
        #[cfg(target_os = "windows")]
        let target_triple = "x86_64-pc-windows-msvc";
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        let target_triple = "aarch64-apple-darwin";
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        let target_triple = "x86_64-apple-darwin";
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        let target_triple = "x86_64-unknown-linux-gnu";

        let sidecar_name = format!("solver-service-{}", target_triple);
        let sidecar_name_exe = format!("solver-service-{}.exe", target_triple);

        candidates.push(PathBuf::from("src-tauri/binaries").join(&sidecar_name_exe));
        candidates.push(PathBuf::from("src-tauri/binaries").join(&sidecar_name));
        candidates.push(PathBuf::from("binaries").join(&sidecar_name_exe));
        candidates.push(PathBuf::from("binaries").join(&sidecar_name));

        // 3. Direct Maven native build output (during dev)
        candidates.push(PathBuf::from("../../apps/solver-service/target/solver-service-2.0.0-runner.exe"));
        candidates.push(PathBuf::from("../../apps/solver-service/target/solver-service-2.0.0-runner"));
        candidates.push(PathBuf::from("../solver-service/target/solver-service-2.0.0-runner.exe"));
        candidates.push(PathBuf::from("../solver-service/target/solver-service-2.0.0-runner"));
        candidates.push(PathBuf::from("apps/solver-service/target/solver-service-2.0.0-runner.exe"));
        candidates.push(PathBuf::from("apps/solver-service/target/solver-service-2.0.0-runner"));

        for candidate in candidates {
            if candidate.is_file() {
                return Some(candidate);
            }
        }

        None
    }
}

impl Drop for SolverSidecarManager {
    fn drop(&mut self) {
        // Attempt to kill child process synchronously on drop
        if let Ok(mut guard) = self.child.try_lock() {
            if let Some(mut child) = guard.take() {
                let _ = child.start_kill();
            }
        }
    }
}
