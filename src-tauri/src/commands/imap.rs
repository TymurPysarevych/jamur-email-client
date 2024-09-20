use crate::commands::helper::helper_keyring::save_keyring_entry;
use crate::database::keychain_entry_repository::KEYCHAIN_KEY_IMAP_PASSWORD;
use crate::database::{keychain_entry_repository, simple_mail_credentials_repository};
use crate::structs::keychain_entry::KeychainEntry;
use crate::structs::simple_mail_credentials::WebSimpleMailCredentials;

#[tauri::command]
pub fn save_imap_config(web_creds: WebSimpleMailCredentials) -> Result<(), ()> {
    let config = web_creds.config.clone();

    save_keyring_entry(
        KEYCHAIN_KEY_IMAP_PASSWORD,
        &config.clone().keychain_id,
        &web_creds.password,
    );

    keychain_entry_repository::save_keychain_entry_imap(&KeychainEntry {
        key: KEYCHAIN_KEY_IMAP_PASSWORD.to_string(),
        id: web_creds.config.clone().keychain_id,
    });
    simple_mail_credentials_repository::save(&web_creds.config.clone());
    Ok(())
}
