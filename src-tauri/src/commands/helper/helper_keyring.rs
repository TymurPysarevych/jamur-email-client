use keyring::Entry;

pub fn save_keyring_entry(keychain_key: &str, keychain_id: &str, secret: &str) {
    let keyring = match Entry::new(keychain_key, keychain_id) {
        Ok(keyring) => keyring,
        Err(e) => {
            panic!("Error creating keyring entry: {:?}", e);
        }
    };

    match keyring.set_password(secret) {
        Ok(_) => (),
        Err(e) => {
            panic!("Error setting password in keyring: {:?}", e);
        }
    };
}

pub fn fetch_keyring_entry(keychain_key: &str, keychain_id: &str) -> String {
    let keyring = match Entry::new(keychain_key, keychain_id) {
        Ok(keyring) => keyring,
        Err(e) => {
            panic!("Error creating keyring entry: {:?}", e);
        }
    };

    match keyring.get_password() {
        Ok(secret) => secret,
        Err(e) => {
            panic!("Error setting password in keyring: {:?}", e);
        }
    }
}
