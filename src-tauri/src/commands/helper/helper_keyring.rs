use keyring::Entry;
use tauri::AppHandle;
use crate::snacks::snacks::send_snacks;
use crate::structs::snack::{SnackHorizontal, SnackSeverity, SnackVertical};

pub fn save_keyring_entry(keychain_key: &str, keychain_id: &str, secret: &str, app: &AppHandle) {
    let keyring = match Entry::new(keychain_key, keychain_id) {
        Ok(keyring) => keyring,
        Err(e) => {
            send_snacks(
                "Error creating keyring entry".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Error creating keyring entry: {:?}", e);
        }
    };

    match keyring.set_password(secret) {
        Ok(_) => (),
        Err(e) => {
            send_snacks(
                "Error setting password in keyring".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Error setting password in keyring: {:?}", e);
        }
    };
}

pub fn fetch_keyring_entry(keychain_key: &str, keychain_id: &str, app: &AppHandle) -> String {
    let keyring = match Entry::new(keychain_key, keychain_id) {
        Ok(keyring) => keyring,
        Err(e) => {
            send_snacks(
                "Error creating keyring entry".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Error creating keyring entry: {:?}", e);
        }
    };

    match keyring.get_password() {
        Ok(secret) => secret,
        Err(e) => {
            send_snacks(
                "Error getting password from keyring".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Error setting password in keyring: {:?}", e);
        }
    }
}
