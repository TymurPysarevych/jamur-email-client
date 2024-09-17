use crate::database::keychain_entry_repository::KEYRING_SERVICE_IMAP_PASSWORD;
use crate::database::{keychain_entry_repository, simple_mail_credentials_repository};
use crate::structs::keychain_entry::KeychainEntry;
use crate::structs::simple_mail_credentials::WebSimpleMailCredentials;

#[tauri::command]
pub fn save_imap_config(web_creds: WebSimpleMailCredentials) -> Result<(), ()> {
    let config = web_creds.config.clone();
    keychain_entry_repository::save_keychain_entry_imap(&KeychainEntry {
        key: KEYRING_SERVICE_IMAP_PASSWORD.to_string(),
        id: config.clone().keychain_id,
    });
    simple_mail_credentials_repository::save(&config.clone());
    Ok(())
}
