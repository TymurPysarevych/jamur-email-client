use crate::database::keychain_entry_repository;

#[tauri::command]
pub async fn credentials_exist() -> Result<bool, ()> {
    let credentials_found = keychain_entry_repository::fetch_keychain_entry_google().len() > 0;
    Ok(credentials_found)
}