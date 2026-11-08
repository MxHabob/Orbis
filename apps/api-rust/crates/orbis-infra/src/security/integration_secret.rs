use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hkdf::Hkdf;
use orbis_domain::errors::DomainError;
use sha2::Sha256;

pub struct IntegrationSecretCipher {
    encryption_key: [u8; 32],
}

impl IntegrationSecretCipher {
    pub fn new(master_key: &str) -> Self {
        let hkdf = Hkdf::<Sha256>::new(
            Some(b"orbis:integration-secret-encryption:v1"),
            master_key.as_bytes(),
        );
        let mut encryption_key = [0u8; 32];
        hkdf.expand(b"integration-secret-storage", &mut encryption_key)
            .expect("32-byte HKDF output is valid");
        Self { encryption_key }
    }

    pub fn encrypt(&self, secret: &str) -> Result<String, DomainError> {
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|_| DomainError::Internal("Invalid integration encryption key".into()))?;
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), secret.as_bytes())
            .map_err(|_| DomainError::Internal("Unable to encrypt integration secret".into()))?;
        let mut payload = nonce_bytes.to_vec();
        payload.extend_from_slice(&ciphertext);
        Ok(format!("v1:{}", URL_SAFE_NO_PAD.encode(payload)))
    }

    pub fn decrypt(&self, value: &str) -> Result<String, DomainError> {
        let Some(encoded_payload) = value.strip_prefix("v1:") else {
            return Ok(value.to_owned());
        };
        let payload = URL_SAFE_NO_PAD
            .decode(encoded_payload)
            .map_err(|_| DomainError::Internal("Invalid encrypted integration secret".into()))?;
        if payload.len() <= 12 {
            return Err(DomainError::Internal(
                "Invalid encrypted integration secret".into(),
            ));
        }
        let (nonce_bytes, ciphertext) = payload.split_at(12);
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|_| DomainError::Internal("Invalid integration encryption key".into()))?;
        let plaintext = cipher
            .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
            .map_err(|_| DomainError::Internal("Unable to decrypt integration secret".into()))?;
        String::from_utf8(plaintext)
            .map_err(|_| DomainError::Internal("Invalid decrypted integration secret".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypted_integration_secrets_round_trip_and_are_key_bound() {
        let cipher = IntegrationSecretCipher::new("stable test master key");
        let encrypted = cipher.encrypt("camera-password").unwrap();

        assert!(encrypted.starts_with("v1:"));
        assert_ne!(encrypted, "camera-password");
        assert_eq!(cipher.decrypt(&encrypted).unwrap(), "camera-password");
        assert!(
            IntegrationSecretCipher::new("different key")
                .decrypt(&encrypted)
                .is_err()
        );
    }

    #[test]
    fn legacy_plaintext_secrets_remain_readable_for_migration() {
        assert_eq!(
            IntegrationSecretCipher::new("master key")
                .decrypt("legacy secret")
                .unwrap(),
            "legacy secret"
        );
    }
}
