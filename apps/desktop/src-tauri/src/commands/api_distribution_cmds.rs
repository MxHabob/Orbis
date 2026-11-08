use base64::{Engine, engine::general_purpose::STANDARD};
use minisign_verify::{PublicKey, Signature};
use serde::{Deserialize, Serialize};

const EXPECTED_PLATFORMS: [&str; 5] = [
    "linux-x86_64",
    "linux-aarch64",
    "windows-x86_64",
    "macos-aarch64",
    "docker-bundle",
];

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiDistributionArtifact {
    pub name: String,
    pub platform: String,
    pub url: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiDistributionManifest {
    pub schema_version: u32,
    pub version: String,
    pub tag: String,
    pub artifacts: Vec<ApiDistributionArtifact>,
    pub docker_image: String,
    pub docker_digest: String,
}

#[tauri::command]
pub fn api_distribution_verify_manifest(
    manifest: String,
    signature: String,
) -> Result<ApiDistributionManifest, String> {
    let public_key = option_env!("TAURI_UPDATER_PUBLIC_KEY")
        .filter(|key| !key.trim().is_empty())
        .ok_or_else(|| {
            "The API release verification key is not available in this application build."
                .to_string()
        })?;
    verify_signed_manifest(&manifest, &signature, public_key)
}

fn verify_signed_manifest(
    contents: &str,
    signature: &str,
    encoded_public_key: &str,
) -> Result<ApiDistributionManifest, String> {
    if contents.len() > 64 * 1024 || signature.len() > 4 * 1024 {
        return Err("The API release manifest or signature exceeds the allowed size.".to_string());
    }

    let public_key = decode_public_key(encoded_public_key)?;
    let signature = decode_signature(signature)?;
    public_key
        .verify(contents.as_bytes(), &signature, false)
        .map_err(|error| format!("The API release manifest signature check failed: {error}"))?;

    let manifest: ApiDistributionManifest = serde_json::from_str(contents)
        .map_err(|error| format!("The API release manifest is invalid: {error}"))?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

fn decode_public_key(encoded: &str) -> Result<PublicKey, String> {
    let decoded = STANDARD
        .decode(encoded.trim())
        .map_err(|error| format!("The API release verification key is invalid: {error}"))?;
    let text = std::str::from_utf8(&decoded)
        .map_err(|error| format!("The API release verification key is invalid: {error}"))?;
    PublicKey::decode(text)
        .map_err(|error| format!("The API release verification key is invalid: {error}"))
}

fn decode_signature(encoded: &str) -> Result<Signature, String> {
    let decoded = STANDARD
        .decode(encoded.trim())
        .map_err(|error| format!("The API release manifest signature is invalid: {error}"))?;
    let text = std::str::from_utf8(&decoded)
        .map_err(|error| format!("The API release manifest signature is invalid: {error}"))?;
    Signature::decode(text)
        .map_err(|error| format!("The API release manifest signature is invalid: {error}"))
}

fn validate_manifest(manifest: &ApiDistributionManifest) -> Result<(), String> {
    if manifest.schema_version != 1 {
        return Err("The API release manifest uses an unsupported schema version.".to_string());
    }

    let version = manifest.version.as_str();
    if !is_semver(version) || manifest.tag != format!("v{version}") {
        return Err("The API release manifest version or tag is invalid.".to_string());
    }

    if manifest.docker_image != format!("ghcr.io/mxhabob/orbis-api@{}", manifest.docker_digest) {
        return Err("The API release manifest contains an unexpected Docker image.".to_string());
    }
    if !manifest.docker_digest.starts_with("sha256:")
        || manifest.docker_digest.len() != 71
        || !manifest.docker_digest[7..]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(
            "The API release manifest contains an invalid Docker image digest.".to_string(),
        );
    }

    let mut seen = std::collections::HashSet::new();
    for artifact in &manifest.artifacts {
        if !EXPECTED_PLATFORMS.contains(&artifact.platform.as_str()) {
            return Err(format!(
                "The API release manifest contains an unsupported platform: {}",
                artifact.platform
            ));
        }
        if !seen.insert(artifact.platform.as_str()) {
            return Err(format!(
                "The API release manifest contains a duplicate platform: {}",
                artifact.platform
            ));
        }
        if !is_expected_artifact_name(version, &artifact.platform, &artifact.name) {
            return Err(format!(
                "The API release manifest contains an invalid artifact name for {}.",
                artifact.platform
            ));
        }
        let expected_url = format!(
            "https://github.com/MxHabob/Orbis/releases/download/{}/{}",
            manifest.tag, artifact.name
        );
        if artifact.url != expected_url {
            return Err(format!(
                "The API release manifest contains an unexpected download URL for {}.",
                artifact.platform
            ));
        }
        if artifact.sha256.len() != 64
            || !artifact.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(format!(
                "The API release manifest contains an invalid checksum for {}.",
                artifact.platform
            ));
        }
    }

    if seen.len() != EXPECTED_PLATFORMS.len() {
        return Err(
            "The API release manifest is missing one or more supported platforms.".to_string(),
        );
    }

    Ok(())
}

fn is_semver(version: &str) -> bool {
    let mut parts = version.split('.');
    let Some(major) = parts.next() else {
        return false;
    };
    let Some(minor) = parts.next() else {
        return false;
    };
    let Some(patch) = parts.next() else {
        return false;
    };

    parts.next().is_none()
        && [major, minor, patch]
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
}

fn is_expected_artifact_name(version: &str, platform: &str, name: &str) -> bool {
    let extension = if platform == "windows-x86_64" {
        "zip"
    } else {
        "tar.gz"
    };
    let expected = if platform == "docker-bundle" {
        format!("Orbis-API-{version}-docker-bundle.zip")
    } else {
        format!("Orbis-API-{version}-{platform}.{extension}")
    };
    name == expected
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUBLIC_KEY: &str = "RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3";
    const SIGNATURE: &str = "untrusted comment: signature from minisign secret key\nRUQf6LRCGA9i559r3g7V1qNyJDApGip8MfqcadIgT9CuhV3EMhHoN1mGTkUidF/z7SrlQgXdy8ofjb7bNJJylDOocrCo8KLzZwo=\ntrusted comment: timestamp:1633700835\tfile:test\tprehashed\nwLMDjy9FLAuxZ3q4NlEvkgtyhrr0gtTu6KC4KBJdITbbOeAi1zBIYo0v4iTgt8jJpIidRJnp94ABQkJAgAooBQ==";

    #[test]
    fn verifies_minisign_signature_and_rejects_tampering() {
        let public_key_file = format!("untrusted comment: minisign public key\n{PUBLIC_KEY}\n");
        let encoded_public_key = STANDARD.encode(public_key_file);
        let encoded_signature = STANDARD.encode(SIGNATURE);
        let public_key = decode_public_key(&encoded_public_key).unwrap();
        let signature = decode_signature(&encoded_signature).unwrap();
        assert!(public_key.verify(b"test", &signature, false).is_ok());
        assert!(public_key.verify(b"tampered", &signature, false).is_err());
    }

    #[test]
    fn validates_release_manifest_platforms_and_names() {
        let manifest = ApiDistributionManifest {
            schema_version: 1,
            version: "1.2.3".to_string(),
            tag: "v1.2.3".to_string(),
            artifacts: EXPECTED_PLATFORMS
                .iter()
                .map(|platform| ApiDistributionArtifact {
                    name: if *platform == "docker-bundle" {
                        "Orbis-API-1.2.3-docker-bundle.zip".to_string()
                    } else {
                        let extension = if *platform == "windows-x86_64" {
                            "zip"
                        } else {
                            "tar.gz"
                        };
                        format!("Orbis-API-1.2.3-{platform}.{extension}")
                    },
                    platform: (*platform).to_string(),
                    url: format!(
                        "https://github.com/MxHabob/Orbis/releases/download/v1.2.3/{}",
                        if *platform == "docker-bundle" {
                            "Orbis-API-1.2.3-docker-bundle.zip".to_string()
                        } else {
                            let extension = if *platform == "windows-x86_64" {
                                "zip"
                            } else {
                                "tar.gz"
                            };
                            format!("Orbis-API-1.2.3-{platform}.{extension}")
                        }
                    ),
                    sha256: "a".repeat(64),
                })
                .collect(),
            docker_image: format!("ghcr.io/mxhabob/orbis-api@sha256:{}", "a".repeat(64)),
            docker_digest: format!("sha256:{}", "a".repeat(64)),
        };
        assert!(validate_manifest(&manifest).is_ok());

        let mut tampered = manifest;
        tampered.artifacts.pop();
        assert!(validate_manifest(&tampered).is_err());
    }

    #[test]
    fn verifies_staged_release_manifest_when_ci_requests_it() {
        let Ok(manifest_path) = std::env::var("ORBIS_API_RELEASE_MANIFEST") else {
            return;
        };
        let signature_path = format!("{manifest_path}.sig");
        let contents = std::fs::read_to_string(&manifest_path).unwrap();
        let signature = std::fs::read_to_string(signature_path).unwrap();
        let public_key = std::env::var("TAURI_UPDATER_PUBLIC_KEY").unwrap();
        verify_signed_manifest(&contents, &signature, &public_key).unwrap();
    }
}
