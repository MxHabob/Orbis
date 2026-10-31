use orbis_domain::errors::DomainError;
use serde_json::Value;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use url::Url;

use super::hikvision::{digest_authorization, parse_digest_challenge};

const RTSP_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);
const MAX_RTSP_HEADERS: usize = 16 * 1024;

pub struct RtspAdapter;

impl RtspAdapter {
    pub async fn check_connection(
        settings: &Value,
        secrets: &HashMap<String, String>,
    ) -> Result<(), DomainError> {
        let endpoint = settings
            .get("rtsp_url")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|endpoint| !endpoint.is_empty())
            .ok_or_else(|| {
                DomainError::Validation(
                    "The RTSP integration is missing the required 'rtsp_url' setting".into(),
                )
            })?;
        let url = Url::parse(endpoint)
            .map_err(|_| DomainError::Validation("RTSP stream URL is invalid".into()))?;
        if url.scheme() != "rtsp"
            || url.host_str().is_none()
            || url.username() != ""
            || url.password().is_some()
            || url.fragment().is_some()
        {
            return Err(DomainError::Validation(
                "RTSP stream URL must use rtsp:// and must not contain embedded credentials or a fragment"
                    .into(),
            ));
        }
        let username = settings
            .get("username")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|username| !username.is_empty())
            .ok_or_else(|| {
                DomainError::Validation(
                    "The RTSP integration is missing the required 'username' setting".into(),
                )
            })?;
        let password = secrets
            .get("password")
            .map(String::as_str)
            .filter(|password| !password.is_empty())
            .ok_or_else(|| {
                DomainError::Validation(
                    "The RTSP integration is missing the required 'password' credential".into(),
                )
            })?;

        let first_response = rtsp_options(&url, None).await?;
        if first_response.status_code == 401 {
            let challenge = first_response
                .headers
                .iter()
                .find_map(|header| {
                    let (name, value) = header.split_once(':')?;
                    name.eq_ignore_ascii_case("www-authenticate")
                        .then_some(value.trim())
                })
                .filter(|value| {
                    value
                        .get(..7)
                        .is_some_and(|scheme| scheme.eq_ignore_ascii_case("digest "))
                })
                .ok_or_else(|| {
                    DomainError::Validation("RTSP camera must support Digest authentication".into())
                })?;
            let challenge = parse_digest_challenge(challenge)?;
            let authorization = digest_authorization(
                &challenge,
                "OPTIONS",
                url.as_str(),
                username,
                password,
                &hex::encode(rand::random::<[u8; 16]>()),
            )?;
            let response = rtsp_options(&url, Some(&authorization)).await?;
            if !(200..300).contains(&response.status_code) {
                return Err(DomainError::Validation(format!(
                    "RTSP camera rejected the connection check (RTSP {})",
                    response.status_code
                )));
            }
        } else if !(200..300).contains(&first_response.status_code) {
            return Err(DomainError::Validation(format!(
                "RTSP camera rejected the connection check (RTSP {})",
                first_response.status_code
            )));
        }
        Ok(())
    }
}

struct RtspResponse {
    status_code: u16,
    headers: Vec<String>,
}

async fn rtsp_options(url: &Url, authorization: Option<&str>) -> Result<RtspResponse, DomainError> {
    let host = url
        .host_str()
        .ok_or_else(|| DomainError::Validation("RTSP stream URL has no host".into()))?;
    let port = url.port().unwrap_or(554);
    let exchange = async {
        let mut stream = TcpStream::connect((host, port))
            .await
            .map_err(|_| DomainError::Validation("Could not reach the RTSP camera".into()))?;
        let request_target = url.as_str();
        let mut request =
            format!("OPTIONS {request_target} RTSP/1.0\r\nCSeq: 1\r\nUser-Agent: Orbis/1.0\r\n");
        if let Some(authorization) = authorization {
            request.push_str(&format!("Authorization: {authorization}\r\n"));
        }
        request.push_str("Connection: close\r\n\r\n");
        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|_| DomainError::Validation("Could not query the RTSP camera".into()))?;

        let mut response = Vec::with_capacity(1024);
        let mut buffer = [0_u8; 1024];
        while response.len() < MAX_RTSP_HEADERS {
            let read = stream
                .read(&mut buffer)
                .await
                .map_err(|_| DomainError::Validation("Could not read the RTSP response".into()))?;
            if read == 0 {
                break;
            }
            response.extend_from_slice(&buffer[..read]);
            if response.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        if response.len() >= MAX_RTSP_HEADERS {
            return Err(DomainError::Validation(
                "RTSP camera returned oversized response headers".into(),
            ));
        }
        let response = String::from_utf8_lossy(&response);
        let mut lines = response.lines();
        let status_code = lines
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|status| status.parse::<u16>().ok())
            .ok_or_else(|| {
                DomainError::Validation("RTSP camera returned an invalid response".into())
            })?;
        Ok(RtspResponse {
            status_code,
            headers: lines.map(str::to_owned).collect(),
        })
    };
    tokio::time::timeout(RTSP_TIMEOUT, exchange)
        .await
        .map_err(|_| DomainError::Validation("RTSP camera connection timed out".into()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rejects_rtsp_urls_with_embedded_credentials() {
        let settings = serde_json::json!({
            "rtsp_url": "rtsp://user:pass@camera.example.com/stream",
            "username": "user"
        });
        assert!(
            RtspAdapter::check_connection(
                &settings,
                &HashMap::from([("password".to_owned(), "secret".to_owned())])
            )
            .await
            .is_err()
        );
    }
}
