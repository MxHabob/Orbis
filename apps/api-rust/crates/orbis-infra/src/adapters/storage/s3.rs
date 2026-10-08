use async_trait::async_trait;
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use orbis_app::ports::storage::{ObjectStream, StorageAdapter};
use orbis_domain::entities::storage::{
    PresignMethod, PresignedUrlRequest, StorageCapabilities, StoredObjectMetadata,
};
use orbis_domain::errors::StorageError;

type HmacSha256 = Hmac<Sha256>;

/// S3-compatible storage configuration.
#[derive(Debug, Clone)]
pub struct S3Config {
    pub endpoint: String, // e.g. "https://s3.us-east-1.amazonaws.com" or "http://127.0.0.1:9000"
    pub bucket: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub force_path_style: bool,
}

/// S3-compatible storage adapter (supporting AWS S3, MinIO, RustFS, R2, Wasabi).
pub struct S3StorageAdapter {
    config: S3Config,
    http_client: reqwest::Client,
}

impl S3StorageAdapter {
    pub fn new(config: S3Config) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap_or_default();

        Self {
            config,
            http_client,
        }
    }

    /// Build the full URL for an object key based on path-style or virtual-hosted configuration.
    fn object_url(&self, key: &str) -> String {
        let clean_key = key.trim_start_matches('/');
        let endpoint = self.config.endpoint.trim_end_matches('/');

        if self.config.force_path_style {
            format!("{}/{}/{}", endpoint, self.config.bucket, clean_key)
        } else if endpoint.starts_with("https://") {
            let host = endpoint.trim_start_matches("https://");
            format!("https://{}.{}/{}", self.config.bucket, host, clean_key)
        } else if endpoint.starts_with("http://") {
            let host = endpoint.trim_start_matches("http://");
            format!("http://{}.{}/{}", self.config.bucket, host, clean_key)
        } else {
            format!("{}/{}/{}", endpoint, self.config.bucket, clean_key)
        }
    }

    /// Compute AWS SigV4 signature components for authenticated requests.
    fn sign_headers(
        &self,
        method: &str,
        url_str: &str,
        payload_hash: &str,
    ) -> Result<HashMap<String, String>, StorageError> {
        let parsed_url = reqwest::Url::parse(url_str)
            .map_err(|e| StorageError::Connection(format!("Invalid S3 URL: {e}")))?;

        let host = parsed_url.host_str().unwrap_or("localhost").to_string();
        let path = parsed_url.path();
        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();

        let mut headers = HashMap::new();
        headers.insert("host".to_string(), host.clone());
        headers.insert("x-amz-date".to_string(), amz_date.clone());
        headers.insert("x-amz-content-sha256".to_string(), payload_hash.to_string());

        let canonical_headers = format!(
            "host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n",
            host, payload_hash, amz_date
        );
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";

        let canonical_request = format!(
            "{}\n{}\n\n{}\n{}\n{}",
            method, path, canonical_headers, signed_headers, payload_hash
        );

        let canonical_request_hash = hex::encode(Sha256::digest(canonical_request.as_bytes()));
        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.config.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, canonical_request_hash
        );

        // Derive signing key
        let k_secret = format!("AWS4{}", self.config.secret_access_key);
        let k_date = hmac_sha256(k_secret.as_bytes(), date_stamp.as_bytes());
        let k_region = hmac_sha256(&k_date, self.config.region.as_bytes());
        let k_service = hmac_sha256(&k_region, b"s3");
        let k_signing = hmac_sha256(&k_service, b"aws4_request");
        let signature = hex::encode(hmac_sha256(&k_signing, string_to_sign.as_bytes()));

        let auth_header = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.config.access_key_id, credential_scope, signed_headers, signature
        );

        headers.insert("Authorization".to_string(), auth_header);
        Ok(headers)
    }
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

fn urlencoding_encode(s: &str) -> String {
    let mut res = String::new();
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                res.push(b as char);
            }
            _ => {
                res.push_str(&format!("%{:02X}", b));
            }
        }
    }
    res
}

#[async_trait]
impl StorageAdapter for S3StorageAdapter {
    fn provider_id(&self) -> &'static str {
        "s3"
    }

    fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities {
            supports_presigned_urls: true,
            supports_multipart_upload: true,
            supports_direct_streaming: true,
            supports_range_reads: true,
            supports_atomic_move: false,
            supports_local_path: false,
        }
    }

    async fn check_health(&self) -> Result<(), StorageError> {
        let key = format!(".orbis-health-check/{}", Uuid::new_v4());
        self.put_bytes(&key, b"orbis-storage-health-check", "text/plain")
            .await?;
        self.delete_object(&key).await
    }

    async fn put_object(
        &self,
        key: &str,
        mut stream: ObjectStream,
        _size_bytes: u64,
        mime_type: &str,
    ) -> Result<StoredObjectMetadata, StorageError> {
        let mut buffer = Vec::new();
        stream
            .read_to_end(&mut buffer)
            .await
            .map_err(|e| StorageError::Io(format!("Failed to read stream for S3 upload: {e}")))?;

        self.put_bytes(key, &buffer, mime_type).await
    }

    async fn put_bytes(
        &self,
        key: &str,
        bytes: &[u8],
        mime_type: &str,
    ) -> Result<StoredObjectMetadata, StorageError> {
        let url = self.object_url(key);
        let payload_hash = hex::encode(Sha256::digest(bytes));
        let signed_headers = self.sign_headers("PUT", &url, &payload_hash)?;

        let mut req = self
            .http_client
            .put(&url)
            .header("Content-Type", mime_type)
            .body(bytes.to_vec());

        for (k, v) in signed_headers {
            req = req.header(k, v);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| StorageError::Connection(format!("S3 PUT request failed: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(StorageError::Io(format!(
                "S3 PUT failed with status {status}: {err_body}"
            )));
        }

        let etag = resp
            .headers()
            .get("etag")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.trim_matches('"').to_string());

        Ok(StoredObjectMetadata {
            key: key.to_string(),
            size_bytes: bytes.len() as u64,
            mime_type: mime_type.to_string(),
            sha256_checksum: payload_hash,
            etag,
            last_modified: Utc::now(),
            custom_metadata: HashMap::new(),
        })
    }

    async fn get_object(&self, key: &str) -> Result<ObjectStream, StorageError> {
        let bytes = self.get_bytes(key).await?;
        let cursor = std::io::Cursor::new(bytes);
        Ok(Box::pin(cursor))
    }

    async fn get_bytes(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        let url = self.object_url(key);
        let payload_hash = hex::encode(Sha256::digest(b""));
        let signed_headers = self.sign_headers("GET", &url, &payload_hash)?;

        let mut req = self.http_client.get(&url);
        for (k, v) in signed_headers {
            req = req.header(k, v);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| StorageError::Connection(format!("S3 GET request failed: {e}")))?;

        if resp.status().as_u16() == 404 {
            return Err(StorageError::NotFound(format!(
                "S3 object not found: {key}"
            )));
        }

        if !resp.status().is_success() {
            return Err(StorageError::Io(format!(
                "S3 GET failed with status: {}",
                resp.status()
            )));
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| StorageError::Io(format!("Failed to read S3 response body: {e}")))?;

        Ok(bytes.to_vec())
    }

    async fn head_object(&self, key: &str) -> Result<StoredObjectMetadata, StorageError> {
        let url = self.object_url(key);
        let payload_hash = hex::encode(Sha256::digest(b""));
        let signed_headers = self.sign_headers("HEAD", &url, &payload_hash)?;

        let mut req = self.http_client.head(&url);
        for (k, v) in signed_headers {
            req = req.header(k, v);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| StorageError::Connection(format!("S3 HEAD request failed: {e}")))?;

        if resp.status().as_u16() == 404 {
            return Err(StorageError::NotFound(format!(
                "S3 object not found: {key}"
            )));
        }

        if !resp.status().is_success() {
            return Err(StorageError::Io(format!(
                "S3 HEAD failed: {}",
                resp.status()
            )));
        }

        let size = resp
            .headers()
            .get("content-length")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let mime = resp
            .headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let etag = resp
            .headers()
            .get("etag")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.trim_matches('"').to_string());

        Ok(StoredObjectMetadata {
            key: key.to_string(),
            size_bytes: size,
            mime_type: mime,
            sha256_checksum: String::new(),
            etag,
            last_modified: Utc::now(),
            custom_metadata: HashMap::new(),
        })
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        match self.head_object(key).await {
            Ok(_) => Ok(true),
            Err(StorageError::NotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn delete_object(&self, key: &str) -> Result<(), StorageError> {
        let url = self.object_url(key);
        let payload_hash = hex::encode(Sha256::digest(b""));
        let signed_headers = self.sign_headers("DELETE", &url, &payload_hash)?;

        let mut req = self.http_client.delete(&url);
        for (k, v) in signed_headers {
            req = req.header(k, v);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| StorageError::Connection(format!("S3 DELETE request failed: {e}")))?;

        if !resp.status().is_success() && resp.status().as_u16() != 404 {
            return Err(StorageError::Io(format!(
                "S3 DELETE failed: {}",
                resp.status()
            )));
        }

        Ok(())
    }

    async fn copy_object(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> Result<StoredObjectMetadata, StorageError> {
        let url = self.object_url(target_key);
        let copy_source = format!(
            "{}/{}",
            self.config.bucket,
            source_key.trim_start_matches('/')
        );
        let payload_hash = hex::encode(Sha256::digest(b""));

        let mut signed_headers = self.sign_headers("PUT", &url, &payload_hash)?;
        signed_headers.insert("x-amz-copy-source".to_string(), copy_source.clone());

        let mut req = self
            .http_client
            .put(&url)
            .header("x-amz-copy-source", copy_source);
        for (k, v) in signed_headers {
            req = req.header(k, v);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| StorageError::Connection(format!("S3 COPY request failed: {e}")))?;

        if !resp.status().is_success() {
            return Err(StorageError::Io(format!(
                "S3 COPY failed: {}",
                resp.status()
            )));
        }

        self.head_object(target_key).await
    }

    async fn move_object(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> Result<StoredObjectMetadata, StorageError> {
        let meta = self.copy_object(source_key, target_key).await?;
        self.delete_object(source_key).await?;
        Ok(meta)
    }

    async fn generate_presigned_url(
        &self,
        request: PresignedUrlRequest,
    ) -> Result<String, StorageError> {
        let method = match request.method {
            PresignMethod::Get => "GET",
            PresignMethod::Put => "PUT",
        };

        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();
        let expires = request.expires_in_seconds.to_string();

        let base_url = self.object_url(&request.key);
        let parsed_url = reqwest::Url::parse(&base_url)
            .map_err(|e| StorageError::Connection(format!("Invalid URL: {e}")))?;

        let host = parsed_url.host_str().unwrap_or("localhost").to_string();
        let path = parsed_url.path();

        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.config.region);
        let credential_param = format!("{}/{}", self.config.access_key_id, credential_scope);

        let query_params = format!(
            "X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential={}&X-Amz-Date={}&X-Amz-Expires={}&X-Amz-SignedHeaders=host",
            urlencoding_encode(&credential_param),
            amz_date,
            expires
        );

        let canonical_headers = format!("host:{}\n", host);
        let signed_headers = "host";
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\nUNSIGNED-PAYLOAD",
            method, path, query_params, canonical_headers, signed_headers
        );

        let canonical_hash = hex::encode(Sha256::digest(canonical_request.as_bytes()));
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, canonical_hash
        );

        let k_secret = format!("AWS4{}", self.config.secret_access_key);
        let k_date = hmac_sha256(k_secret.as_bytes(), date_stamp.as_bytes());
        let k_region = hmac_sha256(&k_date, self.config.region.as_bytes());
        let k_service = hmac_sha256(&k_region, b"s3");
        let k_signing = hmac_sha256(&k_service, b"aws4_request");
        let signature = hex::encode(hmac_sha256(&k_signing, string_to_sign.as_bytes()));

        let final_url = format!(
            "{}?{}&X-Amz-Signature={}",
            base_url, query_params, signature
        );
        Ok(final_url)
    }
}
