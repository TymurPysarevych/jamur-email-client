use crate::database::keychain_entry_repository;
use crate::structs::keychain_entry::KeychainEntry;

#[tauri::command]
pub async fn credentials_exist() -> Result<Vec<KeychainEntry>, ()> {
    let mut entries = vec![];
    entries.append(&mut keychain_entry_repository::fetch_keychain_entry_google());
    entries.append(&mut keychain_entry_repository::fetch_keychain_entry_imap());
    Ok(entries)
}
