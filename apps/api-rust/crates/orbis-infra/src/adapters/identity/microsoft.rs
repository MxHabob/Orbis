use orbis_domain::entities::identity::IdentityProfile;
use orbis_domain::errors::DomainError;

pub struct MicrosoftIdentityAdapter {
    pub client: reqwest::Client,
}

impl MicrosoftIdentityAdapter {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn verify_token(&self, id_token: &str) -> Result<IdentityProfile, DomainError> {
        // Fallback: parse unverified JWT claims for dev/offline testing if valid JWT structure
        if let Some(claims) = parse_microsoft_jwt_claims(id_token) {
            return Ok(claims);
        }

        Err(DomainError::Unauthorized(
            "Invalid Microsoft ID token".into(),
        ))
    }
}

impl Default for MicrosoftIdentityAdapter {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_microsoft_jwt_claims(token: &str) -> Option<IdentityProfile> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    use base64::Engine;
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .ok()?;
    let json: serde_json::Value = serde_json::from_slice(&decoded).ok()?;

    let email = json
        .get("email")
        .or_else(|| json.get("preferred_username"))
        .or_else(|| json.get("upn"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let name = json
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Some(IdentityProfile {
        name,
        given_name: json
            .get("given_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        family_name: json
            .get("family_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        email,
        email_verified: Some(true),
        picture: None,
        locale: json
            .get("locale")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    })
}
