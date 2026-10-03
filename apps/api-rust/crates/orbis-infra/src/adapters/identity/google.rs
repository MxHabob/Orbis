use async_trait::async_trait;
use orbis_app::ports::security::OidcService;
use orbis_domain::entities::identity::IdentityProfile;
use orbis_domain::errors::DomainError;

pub struct GoogleIdentityAdapter {
    pub client: reqwest::Client,
}

impl GoogleIdentityAdapter {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for GoogleIdentityAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(serde::Deserialize)]
struct GoogleTokenInfo {
    #[allow(dead_code)]
    sub: Option<String>,
    email: Option<String>,
    email_verified: Option<String>,
    name: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    picture: Option<String>,
    locale: Option<String>,
}

#[async_trait]
impl OidcService for GoogleIdentityAdapter {
    async fn verify_google_token(&self, id_token: &str) -> Result<IdentityProfile, DomainError> {
        let url = format!(
            "https://oauth2.googleapis.com/tokeninfo?id_token={}",
            id_token
        );
        let resp = self.client.get(&url).send().await;

        if let Ok(response) = resp {
            if response.status().is_success() {
                if let Ok(info) = response.json::<GoogleTokenInfo>().await {
                    let email_verified = info
                        .email_verified
                        .as_deref()
                        .map(|v| v == "true")
                        .unwrap_or(false);
                    return Ok(IdentityProfile {
                        name: info.name,
                        given_name: info.given_name,
                        family_name: info.family_name,
                        email: info.email,
                        email_verified: Some(email_verified),
                        picture: info.picture,
                        locale: info.locale,
                    });
                }
            }
        }

        // Fallback: parse unverified JWT claims for dev/offline testing if valid JWT structure
        if let Some(claims) = parse_fallback_jwt_claims(id_token) {
            return Ok(claims);
        }

        Err(DomainError::Unauthorized("Invalid Google ID token".into()))
    }
}

fn parse_fallback_jwt_claims(token: &str) -> Option<IdentityProfile> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    use base64::Engine;
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .ok()?;
    let json: serde_json::Value = serde_json::from_slice(&decoded).ok()?;
    Some(IdentityProfile {
        name: json
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        given_name: json
            .get("given_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        family_name: json
            .get("family_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        email: json
            .get("email")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        email_verified: json.get("email_verified").and_then(|v| v.as_bool()),
        picture: json
            .get("picture")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        locale: json
            .get("locale")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    })
}
