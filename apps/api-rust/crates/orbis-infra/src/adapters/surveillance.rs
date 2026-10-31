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
    provider: &str,
) -> Result<(&'a str, &'a str), DomainError> {
    let username = settings
        .get("username")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|username| !username.is_empty())
        .ok_or_else(|| {
            DomainError::Validation(format!(
                "The {provider} integration is missing the required 'username' setting"
            ))
        })?;
    let password = secrets
        .get("password")
        .map(String::as_str)
        .filter(|password| !password.is_empty())
        .ok_or_else(|| {
            DomainError::Validation(format!(
                "The {provider} integration is missing the required 'password' credential"
            ))
        })?;
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
    if username.trim().is_empty() || password.is_empty() {
        return Err(DomainError::Validation(format!(
            "{provider} username and password are required"
        )));
    }
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
    if first_response.status() != reqwest::StatusCode::UNAUTHORIZED {
        return Err(DomainError::Validation(
            if first_response.status().is_success() {
                format!(
                    "{provider} camera allows unauthenticated access; credentials could not be verified"
                )
            } else {
                format!(
                    "{provider} camera rejected the connection check (HTTP {})",
                    first_response.status().as_u16()
                )
            },
        ));
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
