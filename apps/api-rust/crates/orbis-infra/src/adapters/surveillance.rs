use orbis_domain::errors::DomainError;
use serde_json::Value;
use std::collections::HashMap;
use std::net::IpAddr;
use url::Url;

use super::hikvision::{digest_authorization, parse_digest_challenge};

const CONNECTION_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

pub fn camera_http_url(
    settings: &Value,
    provider: &str,
    default_port: u16,
) -> Result<String, DomainError> {
    let address = settings
        .get("ip_address")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            DomainError::Validation(format!(
                "The {provider} integration is missing the required 'ip_address' setting"
            ))
        })?;
    let host = match address.parse::<IpAddr>() {
        Ok(IpAddr::V4(address)) => address.to_string(),
        Ok(IpAddr::V6(address)) => format!("[{address}]"),
        Err(_) => match url::Host::parse(address) {
            Ok(url::Host::Domain(hostname)) => hostname,
            _ => {
                return Err(DomainError::Validation(format!(
                    "{provider} camera IP address or hostname is invalid"
                )));
            }
        },
    };
    let port = settings
        .get("port")
        .and_then(|value| {
            value
                .as_u64()
                .or_else(|| value.as_str().and_then(|port| port.parse().ok()))
        })
        .unwrap_or(default_port as u64);
    if !(1..=u16::MAX as u64).contains(&port) {
        return Err(DomainError::Validation(format!(
            "{provider} camera port must be between 1 and 65535"
        )));
    }

    Ok(format!("http://{host}:{port}"))
}

pub fn camera_credentials<'a>(
    settings: &'a Value,
    secrets: &'a HashMap<String, String>,
    _provider: &str,
) -> Result<(&'a str, &'a str), DomainError> {
    let username = settings
        .get("username")
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or_default();
    let password = secrets
        .get("password")
        .map(String::as_str)
        .unwrap_or_default();
    Ok((username, password))
}

pub fn validate_whep_url(settings: &Value, provider: &str) -> Result<String, DomainError> {
    let endpoint = settings
        .get("whep_url")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|endpoint| !endpoint.is_empty())
        .ok_or_else(|| {
            DomainError::Validation(format!(
                "The {provider} integration is missing the required 'whep_url' setting"
            ))
        })?;
    let url = Url::parse(endpoint).map_err(|_| {
        DomainError::Validation(format!("{provider} WHEP endpoint must be a valid URL"))
    })?;
    let is_loopback = url.host().is_some_and(|host| match host {
        url::Host::Ipv4(address) => address.is_loopback(),
        url::Host::Ipv6(address) => address.is_loopback(),
        url::Host::Domain(name) => name.eq_ignore_ascii_case("localhost"),
    });
    if !matches!(url.scheme(), "https") && !(url.scheme() == "http" && is_loopback) {
        return Err(DomainError::Validation(format!(
            "{provider} WHEP endpoint must use HTTPS (HTTP is allowed only for localhost)"
        )));
    }
    if url.host().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !url.path().trim_end_matches('/').ends_with("/whep")
    {
        return Err(DomainError::Validation(format!(
            "{provider} WHEP endpoint must end in /whep and must not contain credentials, query parameters, or a fragment"
        )));
    }
    Ok(url.to_string())
}

pub async fn check_http_digest_endpoint(
    api_url: &str,
    endpoint: &str,
    provider: &str,
    method: reqwest::Method,
    username: &str,
    password: &str,
    body: Option<&str>,
) -> Result<reqwest::Response, DomainError> {
    let mut url = Url::parse(api_url)
        .map_err(|_| DomainError::Validation(format!("{provider} camera address is invalid")))?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(DomainError::Validation(format!(
            "{provider} camera address must be an HTTP or HTTPS URL without embedded credentials"
        )));
    }
    let (path, query) = endpoint.split_once('?').unwrap_or((endpoint, ""));
    url.set_path(path);
    url.set_query((!query.is_empty()).then_some(query));

    let client = reqwest::Client::builder()
        .timeout(CONNECTION_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| {
            DomainError::Internal(format!("Could not initialize {provider} connection check"))
        })?;
    let request = |authorization: Option<&str>| {
        let mut request = client.request(method.clone(), url.clone());
        if let Some(body) = body {
            request = request
                .header(
                    reqwest::header::CONTENT_TYPE,
                    "application/soap+xml; charset=utf-8",
                )
                .body(body.to_owned());
        }
        if let Some(authorization) = authorization {
            request = request.header(reqwest::header::AUTHORIZATION, authorization);
        }
        request
    };

    let first_response = request(None)
        .send()
        .await
        .map_err(|_| DomainError::Validation(format!("Could not reach the {provider} camera")))?;
    if first_response.status().is_success() {
        return Ok(first_response);
    }
    if first_response.status() != reqwest::StatusCode::UNAUTHORIZED {
        return Err(DomainError::Validation(format!(
            "{provider} camera rejected the connection check (HTTP {})",
            first_response.status().as_u16()
        )));
    }
    if username.is_empty() && password.is_empty() {
        return Err(DomainError::Validation(format!(
            "{provider} camera requires authentication; add the camera username and/or password"
        )));
    }

    let challenge = first_response
        .headers()
        .get_all(reqwest::header::WWW_AUTHENTICATE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|value| {
            value
                .trim_start()
                .get(..7)
                .is_some_and(|scheme| scheme.eq_ignore_ascii_case("digest "))
        })
        .ok_or_else(|| {
            DomainError::Validation(format!(
                "{provider} camera did not provide an HTTP Digest authentication challenge"
            ))
        })?;
    let challenge = parse_digest_challenge(challenge)?;
    let uri = match url.query() {
        Some(query) => format!("{}?{query}", url.path()),
        None => url.path().to_string(),
    };
    let authorization = digest_authorization(
        &challenge,
        method.as_str(),
        &uri,
        username,
        password,
        &hex::encode(rand::random::<[u8; 16]>()),
    )?;
    let response = request(Some(&authorization))
        .send()
        .await
        .map_err(|_| DomainError::Validation(format!("Could not reach the {provider} camera")))?;
    if response.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(DomainError::Validation(format!(
            "{provider} camera rejected the supplied credentials"
        )));
    }
    if !response.status().is_success() {
        return Err(DomainError::Validation(format!(
            "{provider} rejected the connection check (HTTP {})",
            response.status().as_u16()
        )));
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    async fn read_http_request(stream: &mut tokio::net::TcpStream) -> String {
        let mut request = Vec::new();
        let mut buffer = [0; 1024];
        loop {
            let count = stream.read(&mut buffer).await.unwrap();
            if count == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..count]);
            if request.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        String::from_utf8_lossy(&request).into_owned()
    }

    async fn write_http_response(
        stream: &mut tokio::net::TcpStream,
        status: &str,
        extra_headers: &str,
    ) {
        stream
            .write_all(
                format!(
                    "HTTP/1.1 {status}\r\n{extra_headers}Content-Length: 0\r\nConnection: close\r\n\r\n"
                )
                .as_bytes(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn accepts_anonymous_api_access_without_camera_credentials() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = read_http_request(&mut stream).await;
            assert!(request.starts_with("GET /device HTTP/1.1"));
            assert!(!request.to_ascii_lowercase().contains("authorization:"));
            write_http_response(&mut stream, "200 OK", "").await;
        });

        let response = check_http_digest_endpoint(
            &format!("http://{address}"),
            "/device",
            "Camera",
            reqwest::Method::GET,
            "",
            "",
            None,
        )
        .await
        .expect("public camera API should be accepted");

        assert!(response.status().is_success());
        server.await.unwrap();
    }

    #[tokio::test]
    async fn retries_digest_auth_with_partial_credentials_when_challenged() {
        for (username, password) in [("admin", ""), ("", "password-only")] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                read_http_request(&mut stream).await;
                write_http_response(
                    &mut stream,
                    "401 Unauthorized",
                    "WWW-Authenticate: Digest realm=\"camera\", nonce=\"test\", qop=\"auth\"\r\n",
                )
                .await;

                let (mut stream, _) = listener.accept().await.unwrap();
                let request = read_http_request(&mut stream).await;
                assert!(
                    request
                        .to_ascii_lowercase()
                        .contains(&format!("authorization: digest username=\"{}\"", username))
                );
                write_http_response(&mut stream, "200 OK", "").await;
            });

            let response = check_http_digest_endpoint(
                &format!("http://{address}"),
                "/device",
                "Camera",
                reqwest::Method::GET,
                username,
                password,
                None,
            )
            .await
            .expect("partial digest credentials should be sent");

            assert!(response.status().is_success());
            server.await.unwrap();
        }
    }

    #[tokio::test]
    async fn requests_credentials_for_protected_api_when_none_were_provided() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            read_http_request(&mut stream).await;
            write_http_response(
                &mut stream,
                "401 Unauthorized",
                "WWW-Authenticate: Digest realm=\"camera\", nonce=\"test\", qop=\"auth\"\r\n",
            )
            .await;
        });

        let error = check_http_digest_endpoint(
            &format!("http://{address}"),
            "/device",
            "Camera",
            reqwest::Method::GET,
            "",
            "",
            None,
        )
        .await
        .expect_err("missing credentials should be requested");

        assert!(error.to_string().contains("requires authentication"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn reports_failed_digest_authentication_without_echoing_credentials() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            for attempt in 0..2 {
                let (mut stream, _) = listener.accept().await.unwrap();
                read_http_request(&mut stream).await;
                let headers = if attempt == 0 {
                    "WWW-Authenticate: Digest realm=\"camera\", nonce=\"test\", qop=\"auth\"\r\n"
                } else {
                    ""
                };
                write_http_response(&mut stream, "401 Unauthorized", headers).await;
            }
        });

        let error = check_http_digest_endpoint(
            &format!("http://{address}"),
            "/device",
            "Camera",
            reqwest::Method::GET,
            "admin",
            "secret-value",
            None,
        )
        .await
        .expect_err("rejected credentials should fail");

        assert!(
            error
                .to_string()
                .contains("rejected the supplied credentials")
        );
        assert!(!error.to_string().contains("secret-value"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn reports_connection_interruption() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            read_http_request(&mut stream).await;
        });

        let result = check_http_digest_endpoint(
            &format!("http://{address}"),
            "/device",
            "Camera",
            reqwest::Method::GET,
            "",
            "",
            None,
        )
        .await;

        assert!(result.is_err());
        server.await.unwrap();
    }

    #[test]
    fn validates_secure_whep_endpoints_without_credentials() {
        let settings = serde_json::json!({
            "whep_url": "https://stream.example.com/campus-east/whep"
        });
        assert_eq!(
            validate_whep_url(&settings, "Camera").expect("valid endpoint"),
            "https://stream.example.com/campus-east/whep"
        );
    }

    #[test]
    fn allows_http_only_for_local_whep_relays() {
        let settings = serde_json::json!({ "whep_url": "http://localhost:8889/campus/whep" });
        assert!(validate_whep_url(&settings, "Camera").is_ok());

        let settings = serde_json::json!({ "whep_url": "http://stream.example.com/campus/whep" });
        assert!(validate_whep_url(&settings, "Camera").is_err());
    }

    #[test]
    fn rejects_whep_endpoints_with_credentials_or_non_whep_paths() {
        for endpoint in [
            "https://user:pass@stream.example.com/campus/whep",
            "https://stream.example.com/campus/whep?token=secret",
            "https://stream.example.com/campus",
        ] {
            let settings = serde_json::json!({ "whep_url": endpoint });
            assert!(
                validate_whep_url(&settings, "Camera").is_err(),
                "{endpoint}"
            );
        }
    }
}
