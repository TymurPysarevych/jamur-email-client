use crate::database::keychain_entry_repository;

#[tauri::command]
pub async fn credentials_exist() -> Result<bool, ()> {
    let credentials_found_google = keychain_entry_repository::fetch_keychain_entry_google().len() > 0;
    let credentials_found_imap = keychain_entry_repository::fetch_keychain_entry_imap().len() > 0;
    Ok(credentials_found_google || credentials_found_imap)
}