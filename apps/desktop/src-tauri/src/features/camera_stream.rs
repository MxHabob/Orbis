use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::net::{TcpListener, UdpSocket};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tracing::{info, warn};
use url::Url;
use uuid::Uuid;

const MEDIAMTX_VERSION: &str = "1.21.3";
const MAX_MEDIAMTX_ARCHIVE_BYTES: u64 = 100 * 1024 * 1024;
const STARTUP_TIMEOUT: Duration = Duration::from_secs(10);
const READY_POLL_INTERVAL: Duration = Duration::from_millis(100);

struct MediaMtxReleaseAsset {
    name: &'static str,
    sha256: &'static str,
}

pub struct CameraStreamSource {
    pub id: Uuid,
    pub rtsp_url: String,
    pub username: Option<String>,
    pub password: Option<String>,
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
            let rtsp_source = make_authenticated_rtsp_url(
                &source.rtsp_url,
                source.username.as_deref(),
                source.password.as_deref(),
            )?;
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
        let binary_path = self.ensure_sidecar_binary().await?;
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
                "Could not start the installed MediaMTX executable ({}): {error}",
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

    async fn ensure_sidecar_binary(&self) -> Result<PathBuf, String> {
        let target = target_triple()
            .ok_or_else(|| "The camera relay is not supported on this platform.".to_string())?;
        let asset = mediamtx_release_asset(target)
            .ok_or_else(|| "The camera relay is not available for this platform.".to_string())?;
        let binary_name = if cfg!(windows) {
            "mediamtx.exe"
        } else {
            "mediamtx"
        };
        let install_root = self.app_data_dir.join("mediamtx");
        let version_dir = install_root.join(format!("v{MEDIAMTX_VERSION}"));
        let binary_path = version_dir.join(binary_name);
        if binary_path.is_file() {
            return Ok(binary_path);
        }

        tokio::fs::create_dir_all(&version_dir)
            .await
            .map_err(|error| format!("Could not create the camera relay directory: {error}"))?;
        let staging_dir = install_root.join(format!(".install-{}", Uuid::new_v4()));
        tokio::fs::create_dir(&staging_dir)
            .await
            .map_err(|error| format!("Could not prepare the camera relay download: {error}"))?;

        let result = async {
            let archive_path = staging_dir.join(asset.name);
            let url = format!(
                "https://github.com/bluenviron/mediamtx/releases/download/v{MEDIAMTX_VERSION}/{}",
                asset.name
            );
            let mut response = self
                .http_client
                .get(url)
                .timeout(Duration::from_secs(120))
                .send()
                .await
                .map_err(|error| format!("Could not download the camera relay: {error}"))?;
            if !response.status().is_success() {
                return Err(format!(
                    "Could not download the camera relay (HTTP {}).",
                    response.status()
                ));
            }
            if response
                .content_length()
                .is_some_and(|length| length > MAX_MEDIAMTX_ARCHIVE_BYTES)
            {
                return Err("The camera relay download exceeds the permitted size.".to_string());
            }

            let mut archive = tokio::fs::File::create(&archive_path)
                .await
                .map_err(|error| format!("Could not save the camera relay download: {error}"))?;
            let mut hasher = Sha256::new();
            let mut downloaded_bytes = 0_u64;
            while let Some(chunk) = response
                .chunk()
                .await
                .map_err(|error| format!("Could not read the camera relay download: {error}"))?
            {
                downloaded_bytes = downloaded_bytes.saturating_add(chunk.len() as u64);
                if downloaded_bytes > MAX_MEDIAMTX_ARCHIVE_BYTES {
                    return Err("The camera relay download exceeds the permitted size.".to_string());
                }
                hasher.update(&chunk);
                archive.write_all(&chunk).await.map_err(|error| {
                    format!("Could not save the camera relay download: {error}")
                })?;
            }
            archive
                .flush()
                .await
                .map_err(|error| format!("Could not finish saving the camera relay: {error}"))?;
            drop(archive);

            let actual_checksum = format!("{:x}", hasher.finalize());
            if actual_checksum != asset.sha256 {
                return Err(
                    "The camera relay download failed its integrity check. Please try again."
                        .to_string(),
                );
            }

            let extraction_dir = staging_dir.join("extracted");
            tokio::fs::create_dir(&extraction_dir)
                .await
                .map_err(|error| format!("Could not prepare the camera relay: {error}"))?;
            let tar_args = if target.contains("windows") {
                "-xf"
            } else {
                "-xzf"
            };
            let extraction = Command::new("tar")
                .arg(tar_args)
                .arg(&archive_path)
                .arg("-C")
                .arg(&extraction_dir)
                .output()
                .await
                .map_err(|error| format!("Could not extract the camera relay: {error}"))?;
            if !extraction.status.success() {
                return Err(format!(
                    "Could not extract the camera relay archive: {}",
                    String::from_utf8_lossy(&extraction.stderr).trim()
                ));
            }

            let executable = find_extracted_binary(
                &extraction_dir,
                if cfg!(windows) {
                    "mediamtx.exe"
                } else {
                    "mediamtx"
                },
            )?
            .ok_or_else(|| "The camera relay archive is missing its executable.".to_string())?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut permissions = std::fs::metadata(&executable)
                    .map_err(|error| format!("Could not inspect the camera relay: {error}"))?
                    .permissions();
                permissions.set_mode(0o755);
                std::fs::set_permissions(&executable, permissions)
                    .map_err(|error| format!("Could not prepare the camera relay: {error}"))?;
            }
            if tokio::fs::try_exists(&binary_path)
                .await
                .map_err(|error| format!("Could not inspect the installed camera relay: {error}"))?
            {
                tokio::fs::remove_file(&binary_path)
                    .await
                    .map_err(|error| format!("Could not replace the camera relay: {error}"))?;
            }
            tokio::fs::rename(&executable, &binary_path)
                .await
                .map_err(|error| format!("Could not install the camera relay: {error}"))?;
            Ok(())
        }
        .await;

        if let Err(error) = tokio::fs::remove_dir_all(&staging_dir).await {
            warn!(%error, "Could not clean up the temporary camera relay download");
        }
        result?;
        Ok(binary_path)
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
    username: Option<&str>,
    password: Option<&str>,
) -> Result<String, String> {
    let mut url = Url::parse(endpoint.trim())
        .map_err(|_| "The camera RTSP stream URL is invalid".to_string())?;
    if url.scheme() != "rtsp"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err("The camera RTSP URL or credentials are invalid".to_string());
    }
    let username = username
        .map(str::trim)
        .filter(|username| !username.is_empty());
    let password = password.filter(|password| !password.is_empty());
    if username.is_none() && password.is_none() {
        return Ok(url.to_string());
    }
    url.set_username(username.unwrap_or_default())
        .map_err(|_| "The camera username cannot be used in an RTSP URL".to_string())?;
    url.set_password(Some(password.unwrap_or_default()))
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

fn find_extracted_binary(directory: &Path, filename: &str) -> Result<Option<PathBuf>, String> {
    for entry in std::fs::read_dir(directory)
        .map_err(|error| format!("Could not inspect the camera relay archive: {error}"))?
    {
        let entry = entry
            .map_err(|error| format!("Could not inspect the camera relay archive: {error}"))?;
        let path = entry.path();
        if entry
            .file_type()
            .map_err(|error| format!("Could not inspect the camera relay archive: {error}"))?
            .is_file()
            && entry.file_name() == filename
        {
            return Ok(Some(path));
        }
        if path.is_dir()
            && let Some(found) = find_extracted_binary(&path, filename)?
        {
            return Ok(Some(found));
        }
    }
    Ok(None)
}

fn mediamtx_release_asset(target: &str) -> Option<MediaMtxReleaseAsset> {
    let (name, sha256) = match target {
        "x86_64-pc-windows-msvc" => (
            "mediamtx_v1.21.3_windows_amd64.zip",
            "b2f5246b07cb72961df03b8d5e8cad43dc657bcf3f49a87bb031bdf22721af74",
        ),
        "x86_64-apple-darwin" => (
            "mediamtx_v1.21.3_darwin_amd64.tar.gz",
            "8a8416ee2ff5d17d9366498b6d88b76ab6d3afcda50b6d7321ccddb349bdbaf2",
        ),
        "aarch64-apple-darwin" => (
            "mediamtx_v1.21.3_darwin_arm64.tar.gz",
            "bc1a0762fd18ffb413bcd81dcb9a072ac159fbd1f27e79ac6f14598aaac5ed3f",
        ),
        "x86_64-unknown-linux-gnu" => (
            "mediamtx_v1.21.3_linux_amd64.tar.gz",
            "0a2ea7eba96023631efa1bea7804823a9145b043b6646a8a05798d4ac24ed635",
        ),
        "aarch64-unknown-linux-gnu" => (
            "mediamtx_v1.21.3_linux_arm64.tar.gz",
            "27f038237cb5f77a724f6afc58eca2798f2bc4edf9989895631a80eeb41a4fe2",
        ),
        _ => return None,
    };
    Some(MediaMtxReleaseAsset { name, sha256 })
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
            Some("camera user"),
            Some("p@ss:word"),
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
                Some("admin"),
                Some("secret")
            )
            .is_err()
        );
    }

    #[test]
    fn allows_public_rtsp_sources_without_credentials() {
        assert_eq!(
            make_authenticated_rtsp_url("rtsp://samples.example:8554/traffic", None, None)
                .expect("public RTSP source"),
            "rtsp://samples.example:8554/traffic"
        );
    }

    #[test]
    fn allows_username_only_rtsp_credentials() {
        let source =
            make_authenticated_rtsp_url("rtsp://camera.example/stream", Some("admin"), None)
                .expect("username-only RTSP source");
        assert_eq!(Url::parse(&source).unwrap().username(), "admin");
    }

    #[test]
    fn allows_password_only_rtsp_credentials() {
        let source =
            make_authenticated_rtsp_url("rtsp://camera.example/stream", None, Some("secret"))
                .expect("password-only RTSP source");
        let parsed = Url::parse(&source).unwrap();
        assert_eq!(parsed.username(), "");
        assert_eq!(parsed.password(), Some("secret"));
    }
}
