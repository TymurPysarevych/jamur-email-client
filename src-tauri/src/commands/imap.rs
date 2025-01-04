use tauri::{AppHandle};
use crate::commands::helper::helper_keyring::save_keyring_entry;
use crate::commands::helper::helper_messages::open_imap_session;
use crate::database::keychain_entry_repository::KEYCHAIN_KEY_IMAP_PASSWORD;
use crate::database::{keychain_entry_repository, simple_mail_credentials_repository};
use crate::snacks::snacks::send_snacks;
use crate::structs::imap_email::{Folder, WebFolders};
use crate::structs::keychain_entry::KeychainEntry;
use crate::structs::simple_mail_credentials::WebSimpleMailCredentials;
use crate::structs::snack::{SnackHorizontal, SnackSeverity, SnackVertical};

#[tauri::command]
pub fn save_imap_config(app: AppHandle,web_creds: WebSimpleMailCredentials) -> Result<(), ()> {
    let config = web_creds.config.clone();

    save_keyring_entry(
        KEYCHAIN_KEY_IMAP_PASSWORD,
        &config.clone().keychain_id,
        &web_creds.password,
        &app,
    );

    keychain_entry_repository::save_keychain_entry_imap(&KeychainEntry {
        key: KEYCHAIN_KEY_IMAP_PASSWORD.to_string(),
        id: web_creds.config.clone().keychain_id,
    });
    simple_mail_credentials_repository::save(&web_creds.config.clone());
    Ok(())
}

#[tauri::command]
pub async fn fetch_imap_folders(app:AppHandle, keychain_entry: KeychainEntry) -> Result<WebFolders, ()> {
    let mut imap_session = open_imap_session(keychain_entry, "", &app).await;

    let folders = match imap_session.list(None, Some("*")) {
        Ok(l) => l,
        Err(e) => {
            send_snacks(
                "Failed to list IMAP folders".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Failed to list IMAP folders {}", e);
        }
    };

    let names: Vec<String> = folders.into_iter().map(|f| f.name().to_string()).collect();

    let delimiter = match folders.first() {
        None => {
            send_snacks(
                "No folders found".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("No folders found");
        },
        Some(f) => f.delimiter().or_else(|| Some("/")).unwrap().to_string(),
    };

    Ok(WebFolders {
        folders: build_folders_recursively(&names, &delimiter),
        delimiter,
    })
}

fn build_folders_recursively(names: &Vec<String>, delimiter: &str) -> Vec<Folder> {
    let mut folders: Vec<Folder> = Vec::new();

    for name in names {
        let parts: Vec<&str> = name.split(delimiter).collect();
        add_folder(&mut folders, &parts, None, "".to_string(), delimiter);
    }

    folders
}

fn add_folder(
    folders: &mut Vec<Folder>,
    parts: &[&str],
    parent: Option<String>,
    parent_path: String,
    delimiter: &str,
) {
    if parts.is_empty() {
        return;
    }

    let folder_name = parts[0].to_string();
    let remaining_parts = &parts[1..];
    let full_path = if parent_path.is_empty() {
        folder_name.clone()
    } else {
        format!("{}{}{}", parent_path, delimiter, folder_name)
    };

    if let Some(existing_folder) = folders.iter_mut().find(|f| f.folder_name == folder_name) {
        add_folder(
            &mut existing_folder.children,
            remaining_parts,
            Some(folder_name.clone()),
            full_path.clone(),
            delimiter,
        );
    } else {
        let mut new_folder = Folder {
            folder_name: folder_name.clone(),
            full_path: full_path.clone(),
            children: Box::new(vec![]),
            parent: parent.clone(),
        };
        add_folder(
            &mut new_folder.children,
            remaining_parts,
            Some(folder_name),
            full_path,
            delimiter,
        );
        folders.push(new_folder);
    }
}
