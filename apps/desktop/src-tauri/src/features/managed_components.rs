use base64::{Engine, engine::general_purpose::STANDARD};
use minisign_verify::{PublicKey, Signature};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tracing::warn;

const MAX_MANIFEST_BYTES: usize = 64 * 1024;
const MAX_ARTIFACT_BYTES: u64 = 512 * 1024 * 1024;

#[derive(Debug, Deserialize)]
struct SignedManifest {
    payload: String,
    signature: String,
}

#[derive(Debug, Deserialize)]
struct ComponentManifest {
    schema_version: u32,
    component_id: String,
    version: String,
    #[serde(default)]
    min_application_version: Option<String>,
    #[serde(default)]
    max_application_version: Option<String>,
    platforms: HashMap<String, ComponentArtifact>,
}

#[derive(Debug, Deserialize)]
struct ComponentArtifact {
    file_name: String,
    url: String,
    sha256: String,
    #[serde(default)]
    size_bytes: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct InstalledComponent {
    pub component_id: String,
    pub version: String,
    pub executable_path: PathBuf,
}

pub struct ManagedComponentManager {
    root_dir: PathBuf,
    manifest_url: Option<url::Url>,
    public_key: Option<PublicKey>,
    client: reqwest::Client,
    operation_lock: Mutex<()>,
    application_version: String,
}

impl ManagedComponentManager {
    pub fn new(
        app_data_dir: &Path,
        manifest_url: Option<&str>,
        encoded_public_key: Option<&str>,
        application_version: &str,
    ) -> Result<Self, String> {
        let manifest_url = manifest_url
            .filter(|value| !value.trim().is_empty())
            .map(|value| parse_https_url(value, "component manifest"))
            .transpose()?;
        let public_key = encoded_public_key
            .filter(|value| !value.trim().is_empty())
            .map(|encoded| {
                let key_bytes = STANDARD.decode(encoded.trim()).map_err(|error| {
                    format!("The component verification key is invalid: {error}")
                })?;
                let key_text = std::str::from_utf8(&key_bytes).map_err(|error| {
                    format!("The component verification key is invalid: {error}")
                })?;
                PublicKey::decode(key_text)
                    .map_err(|error| format!("The component verification key is invalid: {error}"))
            })
            .transpose()?;
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(300))
            .redirect(reqwest::redirect::Policy::custom(|attempt| {
                if attempt.url().scheme() == "https" {
                    attempt.follow()
                } else {
                    attempt.stop()
                }
            }))
            .build()
            .map_err(|error| format!("Could not create the component download client: {error}"))?;

        Ok(Self {
            root_dir: app_data_dir.join("components"),
            manifest_url,
            public_key,
            client,
            operation_lock: Mutex::new(()),
            application_version: application_version.to_string(),
        })
    }

    pub async fn ensure_installed(
        &self,
        component_id: &str,
        target: &str,
    ) -> Result<InstalledComponent, String> {
        validate_component_id(component_id)?;
        let _guard = self.operation_lock.lock().await;
        let download_result = if self.manifest_url.is_some() && self.public_key.is_some() {
            self.download_latest(component_id, target).await
        } else {
            Err("The component update source and verification key are not configured.".to_string())
        };
        match download_result {
            Ok(component) => Ok(component),
            Err(download_error) => match self.find_cached(component_id, target, None).await {
                Ok(Some(component)) => {
                    warn!(
                        %download_error,
                        component_id,
                        version = %component.version,
                        "Using a previously verified component version"
                    );
                    Ok(component)
                }
                Ok(None) => Err(download_error),
                Err(cache_error) => Err(format!(
                    "{download_error} A cached {component_id} component could not be verified: {cache_error}"
                )),
            },
        }
    }

    async fn download_latest(
        &self,
        component_id: &str,
        target: &str,
    ) -> Result<InstalledComponent, String> {
        let response = self
            .client
            .get(
                self.manifest_url
                    .as_ref()
                    .ok_or_else(|| "The component update source is not configured.".to_string())?
                    .clone(),
            )
            .send()
            .await
            .map_err(|error| format!("Could not fetch the {component_id} manifest: {error}"))?;
        if !response.status().is_success() {
            return Err(format!(
                "Could not fetch the {component_id} manifest (HTTP {}).",
                response.status()
            ));
        }
        let manifest_bytes = read_limited_response(response, MAX_MANIFEST_BYTES as u64)
            .await
            .map_err(|error| format!("Could not read the {component_id} manifest: {error}"))?;
        let envelope = parse_signed_manifest(&manifest_bytes)?;
        verify_signature(
            self.public_key
                .as_ref()
                .ok_or_else(|| "The component verification key is not configured.".to_string())?,
            &envelope,
        )?;
        let manifest = parse_component_manifest(&envelope)?;
        let artifact = validate_component_manifest(
            &manifest,
            component_id,
            target,
            &self.application_version,
        )?;

        let version_dir = self.root_dir.join(component_id).join(&manifest.version);
        tokio::fs::create_dir_all(&version_dir)
            .await
            .map_err(|error| format!("Could not prepare {component_id} storage: {error}"))?;
        let executable_path = version_dir.join(&artifact.file_name);

        match sha256_file(&executable_path).await {
            Ok(actual) if actual.eq_ignore_ascii_case(&artifact.sha256) => {
                write_atomically(&version_dir, "component-manifest.json", &manifest_bytes).await?;
                return Ok(InstalledComponent {
                    component_id: component_id.to_string(),
                    version: manifest.version,
                    executable_path,
                });
            }
            Ok(_) => {
                tokio::fs::remove_file(&executable_path)
                    .await
                    .map_err(|error| {
                        format!("Could not replace the cached {component_id}: {error}")
                    })?;
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "Could not verify the cached {component_id}: {error}"
                ));
            }
        }

        let response = self
            .client
            .get(&artifact.url)
            .send()
            .await
            .map_err(|error| format!("Could not download {component_id}: {error}"))?;
        if !response.status().is_success() {
            return Err(format!(
                "Could not download {component_id} (HTTP {}).",
                response.status()
            ));
        }
        if response
            .content_length()
            .is_some_and(|length| length > MAX_ARTIFACT_BYTES)
        {
            return Err(format!(
                "The {component_id} download exceeds the permitted size."
            ));
        }

        let temporary_path = version_dir.join(format!(".component-{}.tmp", uuid::Uuid::new_v4()));
        let download_result = async {
            let mut file = tokio::fs::File::create(&temporary_path)
                .await
                .map_err(|error| format!("Could not save {component_id}: {error}"))?;
            let mut hasher = Sha256::new();
            let mut downloaded_bytes = 0_u64;
            let mut response = response;
            while let Some(chunk) = response
                .chunk()
                .await
                .map_err(|error| format!("Could not read {component_id}: {error}"))?
            {
                downloaded_bytes = downloaded_bytes.saturating_add(chunk.len() as u64);
                if downloaded_bytes > MAX_ARTIFACT_BYTES
                    || artifact
                        .size_bytes
                        .is_some_and(|expected| downloaded_bytes > expected)
                {
                    return Err(format!(
                        "The {component_id} download exceeds its declared size."
                    ));
                }
                hasher.update(&chunk);
                file.write_all(&chunk)
                    .await
                    .map_err(|error| format!("Could not save {component_id}: {error}"))?;
            }
            file.flush()
                .await
                .map_err(|error| format!("Could not finish saving {component_id}: {error}"))?;
            drop(file);

            if artifact
                .size_bytes
                .is_some_and(|expected| expected != downloaded_bytes)
            {
                return Err(format!(
                    "The {component_id} size does not match its signed manifest."
                ));
            }
            let actual_sha256 = format!("{:x}", hasher.finalize());
            if !actual_sha256.eq_ignore_ascii_case(&artifact.sha256) {
                return Err(format!("{component_id} failed its integrity check."));
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut permissions = std::fs::metadata(&temporary_path)
                    .map_err(|error| format!("Could not inspect {component_id}: {error}"))?
                    .permissions();
                permissions.set_mode(0o755);
                std::fs::set_permissions(&temporary_path, permissions)
                    .map_err(|error| format!("Could not prepare {component_id}: {error}"))?;
            }
            tokio::fs::rename(&temporary_path, &executable_path)
                .await
                .map_err(|error| format!("Could not install {component_id}: {error}"))?;
            write_atomically(&version_dir, "component-manifest.json", &manifest_bytes).await?;
            Ok(())
        }
        .await;

        if let Err(error) = tokio::fs::remove_file(&temporary_path).await {
            if error.kind() != io::ErrorKind::NotFound {
                warn!(%error, component_id, "Could not remove incomplete component download");
            }
        }
        download_result?;
        Ok(InstalledComponent {
            component_id: component_id.to_string(),
            version: manifest.version,
            executable_path,
        })
    }

    async fn find_cached(
        &self,
        component_id: &str,
        target: &str,
        skip_version: Option<&str>,
    ) -> Result<Option<InstalledComponent>, String> {
        let Some(public_key) = self.public_key.as_ref() else {
            return Ok(None);
        };
        let component_dir = self.root_dir.join(component_id);
        let mut entries = match tokio::fs::read_dir(&component_dir).await {
            Ok(entries) => entries,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(format!(
                    "Could not inspect cached {component_id} versions: {error}"
                ));
            }
        };
        let mut candidates = Vec::new();
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|error| format!("Could not inspect cached {component_id} versions: {error}"))?
        {
            let version = entry.file_name().to_string_lossy().to_string();
            if parse_version(&version).is_some()
                && entry.file_type().await.is_ok_and(|kind| kind.is_dir())
            {
                candidates.push((version, entry.path()));
            }
        }
        candidates.sort_by(|left, right| parse_version(&right.0).cmp(&parse_version(&left.0)));

        for (version, version_dir) in candidates {
            if skip_version == Some(version.as_str()) {
                continue;
            }
            let manifest_path = version_dir.join("component-manifest.json");
            let bytes = match tokio::fs::read(&manifest_path).await {
                Ok(bytes) => bytes,
                Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
                Err(error) => {
                    warn!(%error, path = %manifest_path.display(), "Could not read cached component manifest");
                    continue;
                }
            };
            if bytes.len() > MAX_MANIFEST_BYTES {
                continue;
            }
            let envelope = match parse_signed_manifest(&bytes) {
                Ok(envelope) => envelope,
                Err(error) => {
                    warn!(%error, component_id, "Ignoring malformed cached component manifest");
                    continue;
                }
            };
            if let Err(error) = verify_signature(public_key, &envelope) {
                warn!(%error, component_id, "Ignoring unsigned cached component manifest");
                continue;
            }
            let manifest = match parse_component_manifest(&envelope) {
                Ok(manifest) => manifest,
                Err(error) => {
                    warn!(%error, component_id, "Ignoring invalid cached component manifest");
                    continue;
                }
            };
            if manifest.version != version {
                continue;
            }
            let artifact = match validate_component_manifest(
                &manifest,
                component_id,
                target,
                &self.application_version,
            ) {
                Ok(artifact) => artifact,
                Err(error) => {
                    warn!(%error, component_id, version, "Ignoring incompatible cached component");
                    continue;
                }
            };
            let executable_path = version_dir.join(&artifact.file_name);
            match sha256_file(&executable_path).await {
                Ok(actual) if actual.eq_ignore_ascii_case(&artifact.sha256) => {
                    return Ok(Some(InstalledComponent {
                        component_id: component_id.to_string(),
                        version,
                        executable_path,
                    }));
                }
                Ok(_) => warn!(
                    component_id,
                    version, "Ignoring cached component with an invalid checksum"
                ),
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => {
                    warn!(%error, component_id, version, "Could not verify cached component")
                }
            }
        }
        Ok(None)
    }

    pub async fn previous_verified(
        &self,
        component_id: &str,
        target: &str,
        failed_version: &str,
    ) -> Result<Option<InstalledComponent>, String> {
        validate_component_id(component_id)?;
        self.find_cached(component_id, target, Some(failed_version))
            .await
    }
}

fn validate_component_id(component_id: &str) -> Result<(), String> {
    if component_id.is_empty()
        || component_id.len() > 64
        || !component_id.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"-_.".contains(&byte)
        })
    {
        return Err("The component identifier is invalid.".to_string());
    }
    Ok(())
}

fn parse_https_url(value: &str, label: &str) -> Result<url::Url, String> {
    let url =
        url::Url::parse(value).map_err(|error| format!("The {label} URL is invalid: {error}"))?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || url.username() != ""
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(format!("The {label} URL must be a plain HTTPS URL."));
    }
    Ok(url)
}

fn parse_signed_manifest(bytes: &[u8]) -> Result<SignedManifest, String> {
    serde_json::from_slice(bytes)
        .map_err(|error| format!("The signed component manifest is invalid: {error}"))
}

fn verify_signature(key: &PublicKey, envelope: &SignedManifest) -> Result<(), String> {
    if envelope.payload.len() > MAX_MANIFEST_BYTES || envelope.signature.len() > 4096 {
        return Err("The component manifest exceeds the allowed size.".to_string());
    }
    let signature = Signature::decode(&envelope.signature)
        .map_err(|error| format!("The component signature is invalid: {error}"))?;
    key.verify(envelope.payload.as_bytes(), &signature, false)
        .map_err(|error| format!("The component signature check failed: {error}"))
}

fn parse_component_manifest(envelope: &SignedManifest) -> Result<ComponentManifest, String> {
    serde_json::from_str(&envelope.payload)
        .map_err(|error| format!("The signed component manifest payload is invalid: {error}"))
}

fn validate_component_manifest<'a>(
    manifest: &'a ComponentManifest,
    component_id: &str,
    target: &str,
    application_version: &str,
) -> Result<&'a ComponentArtifact, String> {
    if manifest.schema_version != 1 || manifest.component_id != component_id {
        return Err("The component manifest identity or schema is unsupported.".to_string());
    }
    parse_version(&manifest.version)
        .ok_or_else(|| "The component version is invalid.".to_string())?;
    let app_version = parse_version(application_version)
        .ok_or_else(|| "The application version is invalid.".to_string())?;
    let minimum = parse_optional_version(manifest.min_application_version.as_deref(), "minimum")?;
    let maximum = parse_optional_version(manifest.max_application_version.as_deref(), "maximum")?;
    if minimum.is_some_and(|minimum| app_version < minimum)
        || maximum.is_some_and(|maximum| app_version > maximum)
        || minimum
            .zip(maximum)
            .is_some_and(|(minimum, maximum)| minimum > maximum)
    {
        return Err("The component is not compatible with this Orbis version.".to_string());
    }
    let artifact = manifest
        .platforms
        .get(target)
        .ok_or_else(|| format!("The component does not support platform {target}."))?;
    if artifact.file_name.is_empty()
        || artifact.file_name.len() > 128
        || artifact.file_name == "."
        || artifact.file_name == ".."
        || !artifact
            .file_name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        || artifact.sha256.len() != 64
        || !artifact.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
        || artifact
            .size_bytes
            .is_some_and(|size| size == 0 || size > MAX_ARTIFACT_BYTES)
    {
        return Err("The component artifact metadata is invalid.".to_string());
    }
    parse_https_url(&artifact.url, "component artifact")?;
    Ok(artifact)
}

fn parse_version(version: &str) -> Option<(u64, u64, u64)> {
    let mut parsed = [0; 3];
    let parts: Vec<_> = version.split('.').collect();
    if parts.len() != parsed.len() {
        return None;
    }
    for (index, part) in parts.iter().enumerate() {
        if part.is_empty()
            || !part.bytes().all(|byte| byte.is_ascii_digit())
            || (part.len() > 1 && part.starts_with('0'))
        {
            return None;
        }
        parsed[index] = part.parse().ok()?;
    }
    Some((parsed[0], parsed[1], parsed[2]))
}

fn parse_optional_version(
    version: Option<&str>,
    bound: &str,
) -> Result<Option<(u64, u64, u64)>, String> {
    version
        .map(|value| {
            parse_version(value)
                .ok_or_else(|| format!("The component {bound} application version is invalid."))
        })
        .transpose()
}

async fn read_limited_response(
    mut response: reqwest::Response,
    max_bytes: u64,
) -> Result<Vec<u8>, String> {
    if response
        .content_length()
        .is_some_and(|length| length > max_bytes)
    {
        return Err("Response exceeds the allowed size.".to_string());
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|error| error.to_string())? {
        if bytes.len().saturating_add(chunk.len()) as u64 > max_bytes {
            return Err("Response exceeds the allowed size.".to_string());
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

async fn write_atomically(
    directory: &Path,
    file_name: &str,
    contents: &[u8],
) -> Result<(), String> {
    let temporary_path = directory.join(format!(".{file_name}-{}.tmp", uuid::Uuid::new_v4()));
    let result = async {
        tokio::fs::write(&temporary_path, contents)
            .await
            .map_err(|error| format!("Could not stage component metadata: {error}"))?;
        let destination = directory.join(file_name);
        if tokio::fs::try_exists(&destination)
            .await
            .map_err(|error| format!("Could not inspect component metadata: {error}"))?
        {
            tokio::fs::remove_file(&destination)
                .await
                .map_err(|error| format!("Could not replace component metadata: {error}"))?;
        }
        tokio::fs::rename(&temporary_path, destination)
            .await
            .map_err(|error| format!("Could not install component metadata: {error}"))
    }
    .await;
    if result.is_err()
        && let Err(error) = tokio::fs::remove_file(&temporary_path).await
        && error.kind() != io::ErrorKind::NotFound
    {
        warn!(%error, "Could not remove temporary component metadata");
    }
    result
}

async fn sha256_file(path: &Path) -> io::Result<String> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUBLIC_KEY: &str = "RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3";
    const TEST_SIGNATURE: &str = "untrusted comment: signature from minisign secret key\nRUQf6LRCGA9i559r3g7V1qNyJDApGip8MfqcadIgT9CuhV3EMhHoN1mGTkUidF/z7SrlQgXdy8ofjb7bNJJylDOocrCo8KLzZwo=\ntrusted comment: timestamp:1633700835\tfile:test\tprehashed\nwLMDjy9FLAuxZ3q4NlEvkgtyhrr0gtTu6KC4KBJdITbbOeAi1zBIYo0v4iTgt8jJpIidRJnp94ABQkJAgAooBQ==";

    #[test]
    fn verifies_component_payload_signatures() {
        let key = PublicKey::from_base64(PUBLIC_KEY).expect("test public key");
        let envelope = SignedManifest {
            payload: "test".to_string(),
            signature: TEST_SIGNATURE.to_string(),
        };
        assert!(verify_signature(&key, &envelope).is_ok());
        let tampered = SignedManifest {
            payload: "tampered".to_string(),
            signature: TEST_SIGNATURE.to_string(),
        };
        assert!(verify_signature(&key, &tampered).is_err());
    }

    #[test]
    fn rejects_invalid_component_identifiers_and_urls() {
        assert!(validate_component_id("orbis-solver").is_ok());
        assert!(validate_component_id("../solver").is_err());
        assert!(parse_https_url("http://example.com/runtime.json", "manifest").is_err());
        assert!(parse_https_url("https://example.com/runtime.json", "manifest").is_ok());
    }

    #[test]
    fn validates_platforms_and_application_compatibility() {
        let mut manifest: ComponentManifest = serde_json::from_str(
            r#"{
              "schema_version": 1,
              "component_id": "orbis-solver",
              "version": "2.3.4",
              "min_application_version": "1.2.0",
              "max_application_version": "3.0.0",
              "platforms": {
                "x86_64-pc-windows-msvc": {
                  "file_name": "orbis-solver.exe",
                  "url": "https://downloads.example.com/orbis-solver.exe",
                  "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                  "size_bytes": 100
                }
              }
            }"#,
        )
        .expect("valid manifest");
        assert!(
            validate_component_manifest(
                &manifest,
                "orbis-solver",
                "x86_64-pc-windows-msvc",
                "2.0.0"
            )
            .is_ok()
        );
        assert!(
            validate_component_manifest(
                &manifest,
                "orbis-solver",
                "x86_64-pc-windows-msvc",
                "1.0.0"
            )
            .is_err()
        );
        assert!(
            validate_component_manifest(&manifest, "orbis-solver", "linux-x64", "2.0.0").is_err()
        );
        manifest
            .platforms
            .get_mut("x86_64-pc-windows-msvc")
            .expect("Windows artifact")
            .file_name = "..".to_string();
        assert!(
            validate_component_manifest(
                &manifest,
                "orbis-solver",
                "x86_64-pc-windows-msvc",
                "2.0.0"
            )
            .is_err()
        );
    }
}
