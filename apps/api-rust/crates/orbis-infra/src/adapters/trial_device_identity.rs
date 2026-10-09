use ed25519_dalek::SigningKey;
use rand::RngCore;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const TRIAL_KEY_FILE: &str = "trial_device_key.dpapi";

pub fn load_or_create(app_data_dir: &Path) -> Result<SigningKey, String> {
    let path = app_data_dir.join(TRIAL_KEY_FILE);
    if path.exists() {
        return load(&path);
    }

    fs::create_dir_all(app_data_dir)
        .map_err(|error| format!("Failed to create trial key directory: {error}"))?;
    let mut seed = [0_u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut seed);
    let protected = protect(&seed)?;

    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(mut file) => {
            file.write_all(&protected)
                .and_then(|()| file.sync_all())
                .map_err(|error| format!("Failed to persist protected trial key: {error}"))?;
            Ok(SigningKey::from_bytes(&seed))
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => load(&path),
        Err(error) => Err(format!("Failed to create protected trial key: {error}")),
    }
}

fn load(path: &Path) -> Result<SigningKey, String> {
    let protected =
        fs::read(path).map_err(|error| format!("Failed to read protected trial key: {error}"))?;
    let seed = unprotect(&protected)?;
    let seed: [u8; 32] = seed
        .try_into()
        .map_err(|_| "Protected trial key has an invalid size".to_owned())?;
    Ok(SigningKey::from_bytes(&seed))
}

#[cfg(windows)]
fn protect(value: &[u8]) -> Result<Vec<u8>, String> {
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData,
    };

    let input = CRYPT_INTEGER_BLOB {
        cbData: value
            .len()
            .try_into()
            .map_err(|_| "Trial key is too large".to_owned())?,
        pbData: value.as_ptr().cast_mut(),
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: null_mut(),
    };
    // DPAPI binds the private key to the current Windows user profile.
    let success = unsafe {
        CryptProtectData(
            &input,
            null(),
            null(),
            null(),
            null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if success == 0 {
        return Err(format!(
            "Windows could not protect the trial device key: {}",
            std::io::Error::last_os_error()
        ));
    }
    if output.pbData.is_null() {
        return Err("Windows returned an empty protected trial key".to_owned());
    }
    if output.cbData == 0 {
        unsafe {
            windows_sys::Win32::Foundation::LocalFree(output.pbData.cast());
        }
        return Err("Windows returned an empty protected trial key".to_owned());
    }
    let protected =
        unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }.to_vec();
    unsafe {
        windows_sys::Win32::Foundation::LocalFree(output.pbData.cast());
    }
    Ok(protected)
}

#[cfg(not(windows))]
fn protect(_value: &[u8]) -> Result<Vec<u8>, String> {
    Err("Central trial registration requires Windows DPAPI key protection".to_owned())
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
            .map_err(|_| "Protected trial key is too large".to_owned())?,
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
            "Windows could not unlock the trial device key: {}",
            std::io::Error::last_os_error()
        ));
    }
    if output.pbData.is_null() {
        return Err("Windows returned an empty trial device key".to_owned());
    }
    if output.cbData == 0 {
        unsafe {
            windows_sys::Win32::Foundation::LocalFree(output.pbData.cast());
        }
        return Err("Windows returned an empty trial device key".to_owned());
    }
    let seed =
        unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }.to_vec();
    unsafe {
        windows_sys::Win32::Foundation::LocalFree(output.pbData.cast());
    }
    Ok(seed)
}

#[cfg(not(windows))]
fn unprotect(_value: &[u8]) -> Result<Vec<u8>, String> {
    Err("Central trial registration requires Windows DPAPI key protection".to_owned())
}
