use async_trait::async_trait;
use md5::{Digest, Md5};
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::SurveillancePort;
use orbis_domain::errors::DomainError;
use reqwest::Client;
use reqwest::header::WWW_AUTHENTICATE;
use std::collections::HashMap;
use url::Url;

pub struct HikvisionAdapter {
    pub api_url: String,
    pub api_key: String,
    pub client: Client,
}

impl HikvisionAdapter {
    pub fn new(api_url: String, api_key: String) -> Self {
        Self {
            api_url,
            api_key,
            client: Client::new(),
        }
    }

    pub async fn check_connection(
        api_url: &str,
        username: &str,
        password: &str,
    ) -> Result<(), DomainError> {
        if username.trim().is_empty() || password.is_empty() {
            return Err(DomainError::Validation(
                "Hikvision username and password are required".into(),
            ));
        }

        let mut base_url = Url::parse(api_url)
            .map_err(|_| DomainError::Validation("Hikvision camera address is invalid".into()))?;
        if !matches!(base_url.scheme(), "http" | "https")
            || base_url.host().is_none()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
            || base_url.query().is_some()
            || base_url.fragment().is_some()
        {
            return Err(DomainError::Validation(
                "Hikvision camera address must be an HTTP or HTTPS URL without embedded credentials"
                    .into(),
            ));
        }
        base_url.set_path("/ISAPI/System/deviceInfo");

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| {
                DomainError::Validation("Could not initialize Hikvision connection check".into())
            })?;
        let response =
            client.get(base_url.clone()).send().await.map_err(|_| {
                DomainError::Validation("Could not reach the Hikvision camera".into())
            })?;
        if response.status() != reqwest::StatusCode::UNAUTHORIZED {
            return Err(DomainError::Validation(if response.status().is_success() {
                "Hikvision camera allows unauthenticated access; credentials could not be verified"
                    .into()
            } else {
                format!(
                    "Hikvision camera rejected the connection check (HTTP {})",
                    response.status().as_u16()
                )
            }));
        }

        let challenge = response
            .headers()
            .get_all(WWW_AUTHENTICATE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .find(|value| {
                value
                    .trim_start()
                    .get(..7)
                    .is_some_and(|scheme| scheme.eq_ignore_ascii_case("digest "))
            })
            .ok_or_else(|| {
                DomainError::Validation(
                    "Hikvision camera did not provide an authentication challenge".into(),
                )
            })?;
        let challenge = parse_digest_challenge(challenge)?;
        let uri = match base_url.query() {
            Some(query) => format!("{}?{query}", base_url.path()),
            None => base_url.path().to_string(),
        };
        let authorization = digest_authorization(
            &challenge,
            "GET",
            &uri,
            username,
            password,
            &hex::encode(rand::random::<[u8; 16]>()),
        )?;
        let response = client
            .get(base_url)
            .header(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&authorization).map_err(|_| {
                    DomainError::Validation("Hikvision authentication header is invalid".into())
                })?,
            )
            .send()
            .await
            .map_err(|_| DomainError::Validation("Could not reach the Hikvision camera".into()))?;

        if !response.status().is_success() {
            return Err(DomainError::Validation(format!(
                "Hikvision rejected the connection check (HTTP {})",
                response.status().as_u16()
            )));
        }

        Ok(())
    }
}

fn parse_digest_challenge(header: &str) -> Result<HashMap<String, String>, DomainError> {
    let (scheme, parameters) = header
        .trim()
        .split_once(char::is_whitespace)
        .ok_or_else(|| {
            DomainError::Validation("Hikvision authentication scheme is unsupported".into())
        })?;
    if !scheme.eq_ignore_ascii_case("digest") {
        return Err(DomainError::Validation(
            "Hikvision camera must support HTTP Digest authentication".into(),
        ));
    }

    let mut values = HashMap::new();
    let mut remaining = parameters.trim();
    while !remaining.is_empty() {
        remaining = remaining.trim_start_matches(|character: char| {
            character.is_ascii_whitespace() || character == ','
        });
        if remaining.is_empty() {
            break;
        }

        let separator = remaining.find('=').ok_or_else(|| {
            DomainError::Validation("Hikvision authentication challenge is malformed".into())
        })?;
        let name = remaining[..separator].trim().to_ascii_lowercase();
        if name.is_empty() {
            return Err(DomainError::Validation(
                "Hikvision authentication challenge is malformed".into(),
            ));
        }
        remaining = remaining[separator + 1..].trim_start();

        let (value, rest) = if let Some(quoted) = remaining.strip_prefix('"') {
            let mut value = String::new();
            let mut characters = quoted.char_indices();
            let mut end = None;
            while let Some((index, character)) = characters.next() {
                match character {
                    '\\' => {
                        let (_, escaped) = characters.next().ok_or_else(|| {
                            DomainError::Validation(
                                "Hikvision authentication challenge is malformed".into(),
                            )
                        })?;
                        value.push(escaped);
                    }
                    '"' => {
                        end = Some(index + character.len_utf8());
                        break;
                    }
                    _ => value.push(character),
                }
            }
            let end = end.ok_or_else(|| {
                DomainError::Validation("Hikvision authentication challenge is malformed".into())
            })?;
            (value, quoted[end..].trim_start())
        } else {
            match remaining.find(',') {
                Some(end) => (remaining[..end].trim().to_string(), &remaining[end..]),
                None => (remaining.trim().to_string(), ""),
            }
        };
        values.insert(name, value);
        remaining = rest;
    }

    if values.get("realm").is_none_or(String::is_empty)
        || values.get("nonce").is_none_or(String::is_empty)
    {
        return Err(DomainError::Validation(
            "Hikvision authentication challenge is missing its realm or nonce".into(),
        ));
    }

    Ok(values)
}

fn digest_authorization(
    challenge: &HashMap<String, String>,
    method: &str,
    uri: &str,
    username: &str,
    password: &str,
    cnonce: &str,
) -> Result<String, DomainError> {
    let realm = challenge.get("realm").ok_or_else(|| {
        DomainError::Validation("Hikvision digest challenge is missing its realm".into())
    })?;
    let nonce = challenge.get("nonce").ok_or_else(|| {
        DomainError::Validation("Hikvision digest challenge is missing its nonce".into())
    })?;
    let algorithm = challenge
        .get("algorithm")
        .map(String::as_str)
        .unwrap_or("MD5");
    if !algorithm.eq_ignore_ascii_case("md5") && !algorithm.eq_ignore_ascii_case("md5-sess") {
        return Err(DomainError::Validation(format!(
            "Hikvision digest algorithm '{algorithm}' is unsupported"
        )));
    }

    let qop = challenge.get("qop").and_then(|value| {
        value
            .split(',')
            .map(str::trim)
            .find(|candidate| candidate.eq_ignore_ascii_case("auth"))
    });
    if challenge.contains_key("qop") && qop.is_none() {
        return Err(DomainError::Validation(
            "Hikvision camera requires an unsupported digest quality of protection".into(),
        ));
    }

    let mut ha1 = md5_hex(&format!("{username}:{realm}:{password}"));
    if algorithm.eq_ignore_ascii_case("md5-sess") {
        ha1 = md5_hex(&format!("{ha1}:{nonce}:{cnonce}"));
    }
    let ha2 = md5_hex(&format!("{method}:{uri}"));
    let response = match qop {
        Some(qop) => md5_hex(&format!("{ha1}:{nonce}:00000001:{cnonce}:{qop}:{}", ha2)),
        None => md5_hex(&format!("{ha1}:{nonce}:{ha2}")),
    };

    let mut fields = vec![
        format!("username=\"{}\"", quote_digest_value(username)),
        format!("realm=\"{}\"", quote_digest_value(realm)),
        format!("nonce=\"{}\"", quote_digest_value(nonce)),
        format!("uri=\"{}\"", quote_digest_value(uri)),
        format!("response=\"{response}\""),
        format!("algorithm={algorithm}"),
    ];
    if let Some(qop) = qop {
        fields.push(format!("qop={qop}"));
        fields.push("nc=00000001".into());
        fields.push(format!("cnonce=\"{cnonce}\""));
    }
    if let Some(opaque) = challenge.get("opaque") {
        fields.push(format!("opaque=\"{}\"", quote_digest_value(opaque)));
    }

    Ok(format!("Digest {}", fields.join(", ")))
}

fn md5_hex(value: &str) -> String {
    hex::encode(Md5::digest(value.as_bytes()))
}

fn quote_digest_value(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_digest_challenge_with_quoted_commas() {
        let challenge = parse_digest_challenge(
            r#"Digest realm="camera, office", nonce="abc", qop="auth,auth-int", opaque="xyz""#,
        )
        .expect("valid challenge");

        assert_eq!(
            challenge.get("realm").map(String::as_str),
            Some("camera, office")
        );
        assert_eq!(challenge.get("nonce").map(String::as_str), Some("abc"));
        assert_eq!(
            challenge.get("qop").map(String::as_str),
            Some("auth,auth-int")
        );
    }

    #[test]
    fn creates_rfc_digest_authorization_header() {
        let challenge = parse_digest_challenge(
            r#"Digest realm="testrealm@host.com", qop="auth,auth-int", nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093", opaque="5ccc069c403ebaf9f0171e9517f40e41""#,
        )
        .expect("valid challenge");
        let authorization = digest_authorization(
            &challenge,
            "GET",
            "/dir/index.html",
            "Mufasa",
            "Circle Of Life",
            "0a4f113b",
        )
        .expect("authorization header");

        assert!(authorization.contains("response=\"6629fae49393a05397450978507c4ef1\""));
    }
}

#[async_trait]
impl SurveillancePort for HikvisionAdapter {
    async fn detect_face(
        &self,
        _ctx: &RequestContext,
        image_data: &[u8],
    ) -> Result<Option<String>, DomainError> {
        let endpoint = format!("{}/ISAPI/Intelligent/face/detect", self.api_url);

        let res = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "image/jpeg")
            .body(image_data.to_vec())
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Hikvision Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Hikvision API Error: {}",
                error_text
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from Hikvision: {}", e)))?;

        if let Some(matches) = body.get("matches").and_then(|m| m.as_array()) {
            if !matches.is_empty() {
                return Ok(Some(
                    matches[0]["subject_id"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                ));
            }
        }

        Ok(None)
    }
}
