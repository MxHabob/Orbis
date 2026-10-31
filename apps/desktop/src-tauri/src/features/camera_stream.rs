use serde_json::json;
use std::collections::HashMap;
use std::net::{TcpListener, UdpSocket};
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tracing::{info, warn};
use url::Url;
use uuid::Uuid;

const STARTUP_TIMEOUT: Duration = Duration::from_secs(10);
const READY_POLL_INTERVAL: Duration = Duration::from_millis(100);

pub struct CameraStreamSource {
    pub id: Uuid,
    pub rtsp_url: String,
    pub username: String,
    pub password: String,
}

struct CameraRuntime {
    child: Child,
    config_path: PathBuf,
    api_url: String,
    api_username: String,
    api_password: String,
    whep_base_url: String,
}

pub struct CameraStreamManager {
    runtime: Mutex<Option<CameraRuntime>>,
    app_data_dir: PathBuf,
    http_client: reqwest::Client,
}

impl CameraStreamManager {
    pub fn new(app_data_dir: &std::path::Path) -> Self {
        Self {
            runtime: Mutex::new(None),
            app_data_dir: app_data_dir.join("camera-streams"),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn start_streams(
        &self,
        sources: &[CameraStreamSource],
    ) -> Result<HashMap<Uuid, String>, String> {
        if sources.is_empty() {
            self.stop().await;
            return Ok(HashMap::new());
        }

        let mut runtime_guard = self.runtime.lock().await;
        let runtime_healthy = match runtime_guard.as_mut() {
            Some(runtime) => {
                if runtime
                    .child
                    .try_wait()
                    .map_err(|error| {
                        format!("Could not inspect the camera relay process: {error}")
                    })?
                    .is_some()
                {
                    false
                } else {
                    self.is_ready(
                        &runtime.api_url,
                        &runtime.api_username,
                        &runtime.api_password,
                    )
                    .await
                }
            }
            None => false,
        };

        if !runtime_healthy {
            if let Some(mut previous) = runtime_guard.take() {
                let _ = previous.child.kill().await;
                let _ = std::fs::remove_file(previous.config_path);
            }
            *runtime_guard = Some(self.spawn_runtime().await?);
        }

        let runtime = runtime_guard
            .as_ref()
            .ok_or_else(|| "The camera relay did not start".to_string())?;
        let api_url = runtime.api_url.clone();
        let api_username = runtime.api_username.clone();
        let api_password = runtime.api_password.clone();
        let whep_base_url = runtime.whep_base_url.clone();
        let mut streams = HashMap::with_capacity(sources.len());

        for source in sources {
            let rtsp_source =
                make_authenticated_rtsp_url(&source.rtsp_url, &source.username, &source.password)?;
            let path_name = format!("camera-{}", source.id.simple());
            if let Err(error) = self
                .configure_path(
                    &api_url,
                    &api_username,
                    &api_password,
                    &path_name,
                    &rtsp_source,
                )
                .await
            {
                drop(runtime_guard);
                self.stop().await;
                return Err(error);
            }
            streams.insert(source.id, format!("{}/{path_name}/whep", whep_base_url));
        }

        info!(camera_count = streams.len(), "Local camera relay is ready");
        Ok(streams)
    }

    pub async fn stop(&self) {
        let mut runtime_guard = self.runtime.lock().await;
        if let Some(mut runtime) = runtime_guard.take() {
            info!("Stopping the local camera relay");
            if let Err(error) = runtime.child.kill().await {
                warn!(%error, "Could not stop the local camera relay cleanly");
            }
            if let Err(error) = std::fs::remove_file(runtime.config_path) {
                if error.kind() != std::io::ErrorKind::NotFound {
                    warn!(%error, "Could not remove the temporary camera relay config");
                }
            }
        }
    }

    async fn spawn_runtime(&self) -> Result<CameraRuntime, String> {
        let binary_path = find_sidecar_binary().ok_or_else(|| {
            "The bundled MediaMTX executable is missing. Rebuild Orbis with the camera relay included."
                .to_string()
        })?;
        let api_port = reserve_tcp_port()?;
        let whep_port = loop {
            let port = reserve_tcp_port()?;
            if port != api_port {
                break port;
            }
        };
        let udp_port = reserve_udp_port()?;
        let api_username = "orbis-control".to_string();
        let api_password = Uuid::new_v4().simple().to_string();

        std::fs::create_dir_all(&self.app_data_dir)
            .map_err(|error| format!("Could not create the camera relay directory: {error}"))?;

        let config_name = format!("mediamtx-{}.yml", Uuid::new_v4());
        let config_path = self.app_data_dir.join(&config_name);
        let temporary_config_path = self.app_data_dir.join(format!("{config_name}.tmp"));
        let config = format!(
            "logLevel: warn\n\
             api: yes\n\
             apiAddress: 127.0.0.1:{api_port}\n\
            authMethod: internal\n\
            authInternalUsers:\n\
            \x20 - user: any\n\
            \x20   pass: \"\"\n\
            \x20   ips: []\n\
            \x20   permissions:\n\
            \x20     - action: read\n\
            \x20       path: ~^camera-\n\
            \x20 - user: {api_username}\n\
            \x20   pass: {api_password}\n\
            \x20   ips: [127.0.0.1]\n\
            \x20   permissions:\n\
            \x20     - action: api\n\
            rtsp: no\n\
             webrtc: yes\n\
             webrtcAddress: 127.0.0.1:{whep_port}\n\
             webrtcLocalUDPAddress: 127.0.0.1:{udp_port}\n\
             webrtcAdditionalHosts:\n\
             \x20 - 127.0.0.1\n\
             webrtcAllowOrigins:\n\
             \x20 - tauri://localhost\n\
             \x20 - http://tauri.localhost\n\
             \x20 - https://tauri.localhost\n\
             \x20 - http://localhost:1520\n\
             \x20 - http://127.0.0.1:1520\n\
             paths: {{}}\n"
        );
        std::fs::write(&temporary_config_path, config)
            .map_err(|error| format!("Could not write camera relay configuration: {error}"))?;
        std::fs::rename(&temporary_config_path, &config_path)
            .map_err(|error| format!("Could not activate camera relay configuration: {error}"))?;

        let mut command = Command::new(&binary_path);
        command
            .arg(&config_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true);

        #[cfg(windows)]
        command.creation_flags(0x08000000);

        let mut child = command.spawn().map_err(|error| {
            let _ = std::fs::remove_file(&config_path);
            format!(
                "Could not start the bundled MediaMTX executable ({}): {error}",
                binary_path.display()
            )
        })?;
        let api_url = format!("http://127.0.0.1:{api_port}");
        let started_at = std::time::Instant::now();

        while started_at.elapsed() < STARTUP_TIMEOUT {
            if child
                .try_wait()
                .map_err(|error| format!("Could not inspect the camera relay process: {error}"))?
                .is_some()
            {
                let _ = std::fs::remove_file(&config_path);
                return Err("The camera relay exited before it became ready".to_string());
            }
            if self.is_ready(&api_url, &api_username, &api_password).await {
                if let Err(error) = std::fs::remove_file(&config_path) {
                    warn!(%error, "Could not remove the temporary camera relay config");
                }
                return Ok(CameraRuntime {
                    child,
                    config_path,
                    api_url,
                    api_username,
                    api_password,
                    whep_base_url: format!("http://127.0.0.1:{whep_port}"),
                });
            }
            tokio::time::sleep(READY_POLL_INTERVAL).await;
        }

        let _ = child.kill().await;
        let _ = std::fs::remove_file(config_path);
        Err("The camera relay did not become ready in time".to_string())
    }

    async fn is_ready(&self, api_url: &str, username: &str, password: &str) -> bool {
        self.http_client
            .get(format!("{api_url}/v3/paths/list"))
            .basic_auth(username, Some(password))
            .timeout(Duration::from_millis(500))
            .send()
            .await
            .is_ok_and(|response| response.status().is_success())
    }

    async fn configure_path(
        &self,
        api_url: &str,
        username: &str,
        password: &str,
        path_name: &str,
        rtsp_source: &str,
    ) -> Result<(), String> {
        let config = json!({
            "source": rtsp_source,
            "sourceOnDemand": true,
            "sourceOnDemandStartTimeout": "10s",
            "sourceOnDemandCloseAfter": "10s"
        });
        let path_url = format!("{api_url}/v3/config/paths/add/{path_name}");
        let response = self
            .http_client
            .post(&path_url)
            .json(&config)
            .basic_auth(username, Some(password))
            .timeout(Duration::from_secs(3))
            .send()
            .await
            .map_err(|error| format!("Could not configure camera stream: {error}"))?;

        if response.status().is_success() {
            return Ok(());
        }

        if response.status() == reqwest::StatusCode::BAD_REQUEST
            || response.status() == reqwest::StatusCode::CONFLICT
        {
            let response = self
                .http_client
                .patch(format!("{api_url}/v3/config/paths/patch/{path_name}"))
                .json(&config)
                .basic_auth(username, Some(password))
                .timeout(Duration::from_secs(3))
                .send()
                .await
                .map_err(|error| format!("Could not update camera stream: {error}"))?;
            if response.status().is_success() {
                return Ok(());
            }
            return Err(format!(
                "The camera relay rejected the stream configuration (HTTP {})",
                response.status()
            ));
        }

        Err(format!(
            "The camera relay rejected the stream configuration (HTTP {})",
            response.status()
        ))
    }
}

fn make_authenticated_rtsp_url(
    endpoint: &str,
    username: &str,
    password: &str,
) -> Result<String, String> {
    let mut url = Url::parse(endpoint.trim())
        .map_err(|_| "The camera RTSP stream URL is invalid".to_string())?;
    if url.scheme() != "rtsp"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || username.trim().is_empty()
        || password.is_empty()
    {
        return Err("The camera RTSP URL or credentials are invalid".to_string());
    }
    url.set_username(username)
        .map_err(|_| "The camera username cannot be used in an RTSP URL".to_string())?;
    url.set_password(Some(password))
        .map_err(|_| "The camera password cannot be used in an RTSP URL".to_string())?;
    Ok(url.to_string())
}

fn reserve_tcp_port() -> Result<u16, String> {
    TcpListener::bind(("127.0.0.1", 0))
        .and_then(|listener| listener.local_addr())
        .map(|address| address.port())
        .map_err(|error| format!("Could not reserve a local camera relay port: {error}"))
}

fn reserve_udp_port() -> Result<u16, String> {
    UdpSocket::bind(("127.0.0.1", 0))
        .and_then(|socket| socket.local_addr())
        .map(|address| address.port())
        .map_err(|error| format!("Could not reserve a local WebRTC port: {error}"))
}

fn find_sidecar_binary() -> Option<PathBuf> {
    let target_triple = target_triple()?;
    let filename = if cfg!(windows) {
        format!("mediamtx-{target_triple}.exe")
    } else {
        format!("mediamtx-{target_triple}")
    };

    let mut candidates = Vec::new();
    if let Ok(current_exe) = std::env::current_exe()
        && let Some(exe_dir) = current_exe.parent()
    {
        candidates.push(exe_dir.join(&filename));
        candidates.push(exe_dir.join("binaries").join(&filename));
        candidates.push(exe_dir.join("resources").join(&filename));
    }
    candidates.push(PathBuf::from("src-tauri").join("binaries").join(&filename));
    candidates.push(
        PathBuf::from("apps/desktop/src-tauri")
            .join("binaries")
            .join(&filename),
    );

    candidates.into_iter().find(|candidate| candidate.is_file())
}

fn target_triple() -> Option<&'static str> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    return Some("x86_64-pc-windows-msvc");
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    return Some("x86_64-apple-darwin");
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    return Some("aarch64-apple-darwin");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return Some("x86_64-unknown-linux-gnu");
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    return Some("aarch64-unknown-linux-gnu");
    #[allow(unreachable_code)]
    None
}

impl Drop for CameraStreamManager {
    fn drop(&mut self) {
        if let Ok(mut runtime_guard) = self.runtime.try_lock()
            && let Some(mut runtime) = runtime_guard.take()
        {
            let _ = runtime.child.start_kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_credentials_without_exposing_them_in_stream_endpoint() {
        let source = make_authenticated_rtsp_url(
            "rtsp://192.168.1.20:554/stream?channel=1",
            "camera user",
            "p@ss:word",
        )
        .expect("valid RTSP source");
        let parsed = Url::parse(&source).expect("valid URL");
        assert_eq!(parsed.username(), "camera%20user");
        assert_eq!(parsed.password(), Some("p%40ss%3Aword"));
        assert_eq!(parsed.path(), "/stream");
        assert_eq!(parsed.query(), Some("channel=1"));
    }

    #[test]
    fn rejects_credentials_embedded_in_rtsp_url() {
        assert!(
            make_authenticated_rtsp_url(
                "rtsp://admin:secret@192.168.1.20/stream",
                "admin",
                "secret"
            )
            .is_err()
        );
    }
}
