use anyhow::{Context, Result, anyhow};
use std::path::Path;
use std::process::{Command, Stdio};

pub const CONFIG_VOLUME: &str = "orbis_server_config";
pub const LICENSE_VOLUME: &str = "orbis_license_data";
pub const POSTGRES_VOLUME: &str = "orbis_postgres_data";

pub fn is_docker_available() -> bool {
    Command::new("docker")
        .args(["info", "--format", "{{.ServerVersion}}"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn run_docker(args: &[&str]) -> Result<String> {
    let out = Command::new("docker")
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute `docker {}`", args.join(" ")))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stdout = String::from_utf8_lossy(&out.stdout);
        return Err(anyhow!(
            "docker {} failed: {}{}",
            args.first().copied().unwrap_or_default(),
            stderr.trim(),
            if stdout.trim().is_empty() {
                ""
            } else {
                stdout.trim()
            }
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn ensure_volume(name: &str) -> Result<()> {
    run_docker(&["volume", "create", name])?;
    Ok(())
}

pub fn ensure_network(name: &str) -> Result<()> {
    let exists = Command::new("docker")
        .args(["network", "inspect", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !exists {
        run_docker(&["network", "create", name])?;
    }
    Ok(())
}

pub fn container_exists(name: &str) -> bool {
    Command::new("docker")
        .args(["container", "inspect", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn container_running(name: &str) -> bool {
    run_docker(&["inspect", "-f", "{{.State.Running}}", name])
        .map(|s| s == "true")
        .unwrap_or(false)
}

/// Ensures a managed PostgreSQL container is running and ready to accept connections.
pub fn ensure_postgres_container(
    container: &str,
    network: &str,
    image: &str,
    user: &str,
    password: &str,
    db_name: &str,
    port: u16,
) -> Result<()> {
    ensure_volume(POSTGRES_VOLUME)?;
    ensure_network(network)?;

    if !container_exists(container) {
        let port_map = format!("127.0.0.1:{port}:5432");
        let vol_map = format!("{POSTGRES_VOLUME}:/var/lib/postgresql/data");
        let env_user = format!("POSTGRES_USER={user}");
        let env_pass = format!("POSTGRES_PASSWORD={password}");
        let env_db = format!("POSTGRES_DB={db_name}");
        run_docker(&[
            "run",
            "-d",
            "--name",
            container,
            "--network",
            network,
            "--restart",
            "unless-stopped",
            "-p",
            &port_map,
            "-v",
            &vol_map,
            "-e",
            &env_user,
            "-e",
            &env_pass,
            "-e",
            &env_db,
            image,
        ])?;
    } else if !container_running(container) {
        run_docker(&["start", container])?;
    }

    // Wait briefly until pg_isready succeeds
    for _ in 0..20 {
        let ready = Command::new("docker")
            .args(["exec", container, "pg_isready", "-U", user, "-d", db_name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if ready {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Err(anyhow!(
        "PostgreSQL container '{container}' did not become ready in time"
    ))
}

/// Writes a host file into a Docker volume at `mount_dir/filename` with `10001:10001` ownership
/// in a single container invocation (no temporary containers).
pub fn write_file_to_volume(
    volume: &str,
    mount_dir: &str,
    local_file: &Path,
    filename: &str,
    image: &str,
) -> Result<()> {
    ensure_volume(volume)?;
    let bytes = std::fs::read(local_file)
        .with_context(|| format!("Failed to read {}", local_file.display()))?;
    let vol_map = format!("{volume}:{mount_dir}");
    let target_file = format!("{mount_dir}/{filename}");
    let script = format!(
        "mkdir -p '{mount_dir}' && cat > '{target_file}' && chown 10001:10001 '{target_file}'"
    );

    let mut child = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-i",
            "--user",
            "0",
            "--entrypoint",
            "sh",
            "-v",
            &vol_map,
            image,
            "-c",
            &script,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn docker run for volume write")?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(&bytes)?;
    }

    let out = child.wait_with_output()?;
    if !out.status.success() {
        return Err(anyhow!(
            "Failed writing {target_file} to volume {volume}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(())
}

/// Reads a file from a Docker volume into a host path in a single container invocation;
/// returns `Ok(false)` if the file does not exist.
pub fn copy_file_from_volume(
    volume: &str,
    mount_dir: &str,
    filename: &str,
    dest_file: &Path,
    image: &str,
) -> Result<bool> {
    let vol_map = format!("{volume}:{mount_dir}");
    let src_file = format!("{mount_dir}/{filename}");
    let script = format!("[ -f '{src_file}' ] && cat '{src_file}'");

    let out = Command::new("docker")
        .args([
            "run",
            "--rm",
            "--user",
            "0",
            "--entrypoint",
            "sh",
            "-v",
            &vol_map,
            image,
            "-c",
            &script,
        ])
        .output()
        .context("Failed to run docker for volume read")?;

    if !out.status.success() {
        return Ok(false);
    }
    std::fs::write(dest_file, &out.stdout)?;
    Ok(true)
}

pub fn run_migrations(image: &str, network: &str) -> Result<()> {
    ensure_network(network)?;
    let cfg_vol = format!("{CONFIG_VOLUME}:/etc/orbis");
    let lic_vol = format!("{LICENSE_VOLUME}:/var/lib/orbis/license");

    let status = Command::new("docker")
        .args([
            "run",
            "--rm",
            "--network",
            network,
            "-v",
            &cfg_vol,
            "-v",
            &lic_vol,
            "-e",
            "ORBIS_SERVER_CONFIG_FILE=/etc/orbis/server.json",
            "-e",
            "ORBIS_LICENSE_DATA_DIR=/var/lib/orbis/license",
            image,
            "migrate",
        ])
        .status()
        .context("Failed to run docker migrate")?;

    if !status.success() {
        return Err(anyhow!("Database migration failed inside Docker"));
    }
    Ok(())
}

pub fn start_or_recreate_api(
    container: &str,
    image: &str,
    network: &str,
    port: u16,
    recreate: bool,
) -> Result<()> {
    ensure_volume(CONFIG_VOLUME)?;
    ensure_volume(LICENSE_VOLUME)?;
    ensure_network(network)?;

    if recreate && container_exists(container) {
        let _ = run_docker(&["rm", "-f", container]);
    }

    if !container_exists(container) {
        let port_map = format!("127.0.0.1:{port}:3000");
        let cfg_vol = format!("{CONFIG_VOLUME}:/etc/orbis");
        let lic_vol = format!("{LICENSE_VOLUME}:/var/lib/orbis/license");
        run_docker(&[
            "run",
            "-d",
            "--name",
            container,
            "--network",
            network,
            "--restart",
            "unless-stopped",
            "--add-host",
            "host.docker.internal:host-gateway",
            "-p",
            &port_map,
            "-v",
            &cfg_vol,
            "-v",
            &lic_vol,
            "-e",
            "BIND_HOST=0.0.0.0",
            "-e",
            "PORT=3000",
            "-e",
            "ORBIS_SERVER_CONFIG_FILE=/etc/orbis/server.json",
            "-e",
            "ORBIS_LICENSE_DATA_DIR=/var/lib/orbis/license",
            image,
        ])?;
    } else if !container_running(container) {
        run_docker(&["start", container])?;
    } else {
        run_docker(&["restart", container])?;
    }
    Ok(())
}

pub fn stop_api(container: &str) -> Result<()> {
    if container_running(container) {
        run_docker(&["stop", container])?;
    }
    Ok(())
}

pub fn restart_api(container: &str) -> Result<()> {
    if container_exists(container) {
        run_docker(&["restart", container])?;
    }
    Ok(())
}

pub fn tail_logs(container: &str, lines: u32) -> Result<()> {
    let n = lines.to_string();
    Command::new("docker")
        .args(["logs", "--tail", &n, "-f", container])
        .status()
        .context("Failed to stream docker logs")?;
    Ok(())
}

pub fn pull_image(image: &str) -> Result<()> {
    let status = Command::new("docker")
        .args(["pull", image])
        .status()
        .context("Failed to pull docker image")?;
    if !status.success() {
        return Err(anyhow!("Failed to pull image {image}"));
    }
    Ok(())
}
