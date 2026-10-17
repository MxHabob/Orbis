use base64::Engine;
use ed25519_dalek::SigningKey;
use fs2::FileExt;
use keyring::{Entry, Error as KeyringError};
use rand::RngCore;
use std::fs::{self, OpenOptions};
use std::path::Path;
use zeroize::{Zeroize, Zeroizing};

const TRIAL_KEY_SERVICE: &str = "com.mulverse.orbis.trial-device";
const TRIAL_KEY_ACCOUNT: &str = "device";
const INSTALLATION_ID_ACCOUNT: &str = "installation-id";
const DEPLOYMENT_KEY_SERVICE: &str = "com.mulverse.orbis.deployment";
const DEPLOYMENT_KEY_ACCOUNT: &str = "identity";
#[cfg(windows)]
const LEGACY_TRIAL_KEY_FILE: &str = "trial_device_key.dpapi";
const TRIAL_KEY_LOCK_FILE: &str = ".trial-device-key.lock";

pub fn load_or_create(app_data_dir: &Path, installation_id: &str) -> Result<SigningKey, String> {
    load_or_create_with_service(app_data_dir, installation_id, TRIAL_KEY_SERVICE)
}

/// Load Orbis's deployment identity from the OS key store, creating it only
/// when the key store explicitly reports that no identity exists.
pub fn load_or_create_deployment_identity(app_data_dir: &Path) -> Result<SigningKey, String> {
    let lock = lock_identity_store(app_data_dir)?;
    let result = (|| {
        let entry =
            Entry::new(DEPLOYMENT_KEY_SERVICE, DEPLOYMENT_KEY_ACCOUNT).map_err(|error| {
                format!("Failed to open the operating system's secure key store: {error}")
            })?;
        match entry.get_password() {
            Ok(encoded_seed) => decode_seed(&encoded_seed),
            Err(KeyringError::NoEntry) => {
                let mut seed = Zeroizing::new([0_u8; 32]);
                rand::rngs::OsRng.fill_bytes(&mut *seed);
                store_seed(&entry, &seed)?;
                Ok(SigningKey::from_bytes(&seed))
            }
            Err(error) => Err(format!(
                "Failed to read the deployment identity from the operating system's secure key store: {error}"
            )),
        }
    })();
    let unlock_result = FileExt::unlock(&lock)
        .map_err(|error| format!("Failed to unlock deployment identity: {error}"));
    match (result, unlock_result) {
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(error),
        (Ok(key), Ok(())) => Ok(key),
    }
}

pub fn load_deployment_identity() -> Result<SigningKey, String> {
    let entry = Entry::new(DEPLOYMENT_KEY_SERVICE, DEPLOYMENT_KEY_ACCOUNT).map_err(|error| {
        format!("Failed to open the operating system's secure key store: {error}")
    })?;
    match entry.get_password() {
        Ok(encoded_seed) => decode_seed(&encoded_seed),
        Err(KeyringError::NoEntry) => Err("The deployment identity is not initialized".to_owned()),
        Err(error) => Err(format!(
            "Failed to read the deployment identity from the operating system's secure key store: {error}"
        )),
    }
}

pub fn load_or_create_installation_id(app_data_dir: &Path) -> Result<String, String> {
    load_or_create_installation_id_with_service(
        app_data_dir,
        TRIAL_KEY_SERVICE,
        INSTALLATION_ID_ACCOUNT,
    )
}

pub(crate) fn load_or_create_installation_id_with_service(
    app_data_dir: &Path,
    service: &str,
    account: &str,
) -> Result<String, String> {
    let lock = lock_identity_store(app_data_dir)?;
    let result = (|| {
        let entry = Entry::new(service, account).map_err(|error| {
            format!("Failed to open the operating system's secure key store: {error}")
        })?;
        match entry.get_password() {
            Ok(installation_id) => {
                validate_installation_id(&installation_id)?;
                remove_plaintext_installation_id(app_data_dir);
                Ok(installation_id)
            }
            Err(KeyringError::NoEntry) => {
                let legacy_path = app_data_dir.join("installation_id");
                let installation_id = if legacy_path.exists() {
                    let id = fs::read_to_string(&legacy_path)
                        .map_err(|error| {
                            format!("Failed to read the previous installation ID: {error}")
                        })?
                        .trim()
                        .to_owned();
                    validate_installation_id(&id)?;
                    id
                } else {
                    orbis_domain::entities::license::generate_installation_id()
                };
                entry.set_password(&installation_id).map_err(|error| {
                    format!("Failed to save installation identity in the operating system's secure key store: {error}")
                })?;
                match entry.get_password() {
                    Ok(saved) if saved == installation_id => {
                        remove_plaintext_installation_id(app_data_dir);
                        Ok(installation_id)
                    }
                    Ok(_) => Err(
                        "The operating system's secure key store returned a different installation ID"
                            .to_owned(),
                    ),
                    Err(error) => Err(format!(
                        "Failed to verify the saved installation ID in the operating system's secure key store: {error}"
                    )),
                }
            }
            Err(error) => Err(format!(
                "Failed to read the installation identity from the operating system's secure key store: {error}"
            )),
        }
    })();
    let unlock_result =
        FileExt::unlock(&lock).map_err(|error| format!("Failed to unlock trial identity: {error}"));
    match (result, unlock_result) {
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(error),
        (Ok(installation_id), Ok(())) => Ok(installation_id),
    }
}

fn validate_installation_id(installation_id: &str) -> Result<(), String> {
    if installation_id.is_empty()
        || installation_id.len() > 128
        || !installation_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    {
        return Err("The stored installation ID is invalid".to_owned());
    }
    Ok(())
}

fn remove_plaintext_installation_id(app_data_dir: &Path) {
    if let Err(error) = fs::remove_file(app_data_dir.join("installation_id"))
        && error.kind() != std::io::ErrorKind::NotFound
    {
        tracing::warn!(%error, "Could not remove the plaintext installation ID");
    }
}

fn lock_identity_store(app_data_dir: &Path) -> Result<std::fs::File, String> {
    fs::create_dir_all(app_data_dir)
        .map_err(|error| format!("Failed to create trial key directory: {error}"))?;
    let lock_path = app_data_dir.join(TRIAL_KEY_LOCK_FILE);
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(lock_path)
        .map_err(|error| format!("Failed to open trial identity lock: {error}"))?;
    lock.lock_exclusive()
        .map_err(|error| format!("Failed to lock trial identity: {error}"))?;
    Ok(lock)
}

fn load_or_create_with_service(
    app_data_dir: &Path,
    installation_id: &str,
    service: &str,
) -> Result<SigningKey, String> {
    let lock = lock_identity_store(app_data_dir)?;
    let result = load_or_create_locked(app_data_dir, installation_id, service);
    let unlock_result =
        FileExt::unlock(&lock).map_err(|error| format!("Failed to unlock trial identity: {error}"));
    match (result, unlock_result) {
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(error),
        (Ok(key), Ok(())) => Ok(key),
    }
}

fn load_or_create_locked(
    app_data_dir: &Path,
    installation_id: &str,
    service: &str,
) -> Result<SigningKey, String> {
    let entry = Entry::new(service, TRIAL_KEY_ACCOUNT).map_err(|error| {
        format!("Failed to open the operating system's secure key store: {error}")
    })?;

    match entry.get_password() {
        Ok(encoded_seed) => decode_seed(&encoded_seed),
        Err(KeyringError::NoEntry) => {
            if let Some(legacy_key) = load_installation_key(service, installation_id)? {
                let seed = Zeroizing::new(legacy_key.to_bytes());
                store_seed(&entry, &seed)?;
                remove_installation_key(service, installation_id);
                return Ok(legacy_key);
            }

            #[cfg(windows)]
            if let Some(seed) = load_legacy_seed(app_data_dir)? {
                store_seed(&entry, &seed)?;
                remove_legacy_key(app_data_dir);
                return Ok(SigningKey::from_bytes(&seed));
            }

            let mut seed = Zeroizing::new([0_u8; 32]);
            rand::rngs::OsRng.fill_bytes(&mut *seed);
            store_seed(&entry, &seed)?;
            Ok(SigningKey::from_bytes(&seed))
        }
        Err(error) => Err(format!(
            "Failed to read the trial identity from the operating system's secure key store: {error}"
        )),
    }
}

fn load_installation_key(
    service: &str,
    installation_id: &str,
) -> Result<Option<SigningKey>, String> {
    let account = format!("installation:{installation_id}");
    let entry = Entry::new(service, &account).map_err(|error| {
        format!("Failed to open the previous trial identity in the operating system's secure key store: {error}")
    })?;
    match entry.get_password() {
        Ok(encoded_seed) => decode_seed(&encoded_seed).map(Some),
        Err(KeyringError::NoEntry) => Ok(None),
        Err(error) => Err(format!(
            "Failed to read the previous trial identity from the operating system's secure key store: {error}"
        )),
    }
}

fn remove_installation_key(service: &str, installation_id: &str) {
    let account = format!("installation:{installation_id}");
    let result = Entry::new(service, &account)
        .map_err(|error| error.to_string())
        .and_then(|entry| match entry.delete_credential() {
            Ok(()) | Err(KeyringError::NoEntry) => Ok(()),
            Err(error) => Err(error.to_string()),
        });
    if let Err(error) = result {
        tracing::warn!(%error, "Could not remove the migrated installation-scoped trial identity");
    }
}

fn decode_seed(encoded_seed: &str) -> Result<SigningKey, String> {
    let mut seed = Zeroizing::new(
        base64::engine::general_purpose::STANDARD
            .decode(encoded_seed)
            .map_err(|_| "The stored trial identity is invalid".to_owned())?,
    );
    let seed_array = Zeroizing::new(
        seed.as_slice()
            .try_into()
            .map_err(|_| "The stored trial identity has an invalid size".to_owned())?,
    );
    seed.zeroize();
    Ok(SigningKey::from_bytes(&seed_array))
}

fn store_seed(entry: &Entry, seed: &[u8; 32]) -> Result<(), String> {
    use base64::Engine;

    let encoded_seed = base64::engine::general_purpose::STANDARD.encode(seed);
    entry.set_password(&encoded_seed).map_err(|error| {
        format!("Failed to save trial identity in the operating system's secure key store: {error}")
    })?;
    match entry.get_password() {
        Ok(saved) if saved == encoded_seed => Ok(()),
        Ok(_) => Err(
            "The operating system's secure key store returned a different trial identity"
                .to_owned(),
        ),
        Err(error) => Err(format!(
            "Failed to verify the saved trial identity in the operating system's secure key store: {error}"
        )),
    }
}

#[cfg(windows)]
fn load_legacy_seed(app_data_dir: &Path) -> Result<Option<Zeroizing<[u8; 32]>>, String> {
    let path = app_data_dir.join(LEGACY_TRIAL_KEY_FILE);
    if !path.exists() {
        return Ok(None);
    }

    let protected = fs::read(path).map_err(|error| {
        format!("Failed to read the previous protected trial identity: {error}")
    })?;
    let mut seed = Zeroizing::new(unprotect(&protected)?);
    let seed_array = Zeroizing::new(
        seed.as_slice()
            .try_into()
            .map_err(|_| "The previous protected trial identity has an invalid size".to_owned())?,
    );
    seed.zeroize();
    Ok(Some(seed_array))
}

#[cfg(windows)]
fn remove_legacy_key(app_data_dir: &Path) {
    if let Err(error) = fs::remove_file(app_data_dir.join(LEGACY_TRIAL_KEY_FILE)) {
        tracing::warn!(%error, "Could not remove the migrated DPAPI trial identity file");
    }
}

#[cfg(windows)]
fn unprotect(value: &[u8]) -> Result<Vec<u8>, String> {
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptUnprotectData,
    };

    let input = CRYPT_INTEGER_BLOB {
        cbData: value
            .len()
            .try_into()
            .map_err(|_| "The previous protected trial identity is too large".to_owned())?,
        pbData: value.as_ptr().cast_mut(),
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: null_mut(),
    };
    let success = unsafe {
        CryptUnprotectData(
            &input,
            null_mut(),
            null(),
            null(),
            null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if success == 0 {
        return Err(format!(
            "Windows could not unlock the previous protected trial identity: {}",
            std::io::Error::last_os_error()
        ));
    }
    if output.pbData.is_null() {
        return Err("Windows returned an empty previous trial identity".to_owned());
    }
    if output.cbData == 0 {
        unsafe {
            windows_sys::Win32::Foundation::LocalFree(output.pbData.cast());
        }
        return Err("Windows returned an empty previous trial identity".to_owned());
    }
    let seed =
        unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }.to_vec();
    unsafe {
        std::ptr::write_bytes(output.pbData, 0, output.cbData as usize);
    }
    unsafe {
        windows_sys::Win32::Foundation::LocalFree(output.pbData.cast());
    }
    Ok(seed)
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn native_secure_store_keeps_trial_identity_stable() {
        let directory =
            std::env::temp_dir().join(format!("orbis-trial-key-test-{}", Uuid::new_v4()));
        let service = format!("{TRIAL_KEY_SERVICE}.test-{}", Uuid::new_v4());
        let installation_id = Uuid::new_v4().to_string();
        let first = load_or_create_with_service(&directory, &installation_id, &service)
            .expect("Windows Credential Manager should save the generated key");
        let second = load_or_create_with_service(&directory, &Uuid::new_v4().to_string(), &service)
            .expect("Windows Credential Manager should unlock the saved key");
        assert_eq!(first.to_bytes(), second.to_bytes());
        remove_keyring_entry(&service, TRIAL_KEY_ACCOUNT);
        fs::remove_dir_all(directory).expect("test directory should be cleaned up");
    }

    #[test]
    fn secure_installation_identity_ignores_plaintext_changes() {
        let directory =
            std::env::temp_dir().join(format!("orbis-installation-id-test-{}", Uuid::new_v4()));
        let service = format!("{TRIAL_KEY_SERVICE}.test-{}", Uuid::new_v4());
        let account = format!("{INSTALLATION_ID_ACCOUNT}-{}", Uuid::new_v4());
        let first = load_or_create_installation_id_with_service(&directory, &service, &account)
            .expect("secure store should save the installation ID");
        fs::write(directory.join("installation_id"), "attacker-controlled")
            .expect("test should write a modified plaintext ID");
        let second = load_or_create_installation_id_with_service(&directory, &service, &account)
            .expect("secure store should return the original installation ID");
        assert_eq!(first, second);
        remove_keyring_entry(&service, &account);
        fs::remove_dir_all(directory).expect("test directory should be cleaned up");
    }

    #[test]
    fn plaintext_installation_identity_is_migrated_to_secure_store() {
        let directory =
            std::env::temp_dir().join(format!("orbis-installation-id-test-{}", Uuid::new_v4()));
        let service = format!("{TRIAL_KEY_SERVICE}.test-{}", Uuid::new_v4());
        let account = format!("{INSTALLATION_ID_ACCOUNT}-{}", Uuid::new_v4());
        let legacy_id = orbis_domain::entities::license::generate_installation_id();
        fs::create_dir_all(&directory).expect("test directory should be created");
        fs::write(directory.join("installation_id"), &legacy_id)
            .expect("legacy ID should be written");

        let migrated = load_or_create_installation_id_with_service(&directory, &service, &account)
            .expect("legacy ID should migrate to the secure store");

        assert_eq!(migrated, legacy_id);
        assert!(!directory.join("installation_id").exists());
        remove_keyring_entry(&service, &account);
        fs::remove_dir_all(directory).expect("test data should be cleaned up");
    }

    fn remove_keyring_entry(service: &str, account: &str) {
        let entry = Entry::new(service, account).expect("test entry should be valid");
        match entry.delete_credential() {
            Ok(()) | Err(KeyringError::NoEntry) => {}
            Err(error) => panic!("test secure-store entry should be removed: {error}"),
        }
    }
}
