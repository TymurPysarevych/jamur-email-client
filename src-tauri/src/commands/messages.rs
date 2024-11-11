extern crate chrono;
extern crate encoding_rs;
extern crate imap;
extern crate native_tls;

use crate::commands::google::oauth::renew_token;
use crate::commands::helper::helper_messages::*;
use crate::database::email_repository;
use crate::database::keychain_entry_repository::fetch_keychain_entry_google;
use crate::snacks::snacks::send_snacks;
use crate::structs::google::email::GEmail;
use crate::structs::imap_email::{WebEmail, WebEmailPreview};
use crate::structs::keychain_entry::KeychainEntry;
use crate::structs::snack::{SnackHorizontal, SnackSeverity, SnackVertical};
use chrono::NaiveDateTime;
use log::error;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub async fn fetch_messages(app: AppHandle, keychain_entry: KeychainEntry, folder: String) {
    let mut web_emails: Vec<WebEmailPreview> = vec![];

    let db_emails = match email_repository::fetch_all_by_folder_path(folder.clone()) {
        Ok(m) => m,
        Err(e) => {
            error!(
                "Error while fetching all emails in folder: {} \n {:?}",
                folder, e
            );
            send_snacks(
                "Error while fetching emails".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            vec![]
        }
    };

    if db_emails.len() == 0 {
        let mut imap_session = open_imap_session(keychain_entry, &*folder, &app).await;
        let messages_stream = imap_session.fetch("1:*", "RFC822").ok();
        imap_session.logout().ok();
        if messages_stream.is_some() {
            let messages = messages_stream.unwrap();
            web_emails = messages
                .iter()
                .map(|message| parse_message(message, folder.clone(), &app))
                .collect::<Vec<WebEmailPreview>>();
        }
    } else {
        let last_email = match email_repository::fetch_latest_email_by_folder_path(folder.clone()) {
            Ok(m) => m,
            Err(e) => {
                send_snacks(
                    "Error while fetching emails".to_string(),
                    SnackSeverity::Error,
                    SnackVertical::Top,
                    SnackHorizontal::Right,
                    &app,
                );
                panic!(
                    "Error while fetching all emails in folder: {} \n {:?}",
                    folder, e
                );
            }
        };

        web_emails = fetch_messages_by_query(
            app.clone(),
            keychain_entry,
            last_email.delivered_at.clone(),
            folder,
        )
            .await
            .unwrap();
    }

    web_emails.extend(db_emails);

    web_emails.sort_by(|a, b| b.delivered_at.cmp(&a.delivered_at));

    app.emit("new_emails", web_emails).unwrap_or_else(|e| {
        error!("Error while emitting new emails: {:?}", e);
        send_snacks(
            "Error while emitting new emails".to_string(),
            SnackSeverity::Error,
            SnackVertical::Top,
            SnackHorizontal::Right,
            &app,
        );
    });
}

#[tauri::command]
pub async fn fetch_messages_by_query(
    app: AppHandle,
    keychain_entry: KeychainEntry,
    since: NaiveDateTime,
    folder: String,
) -> Result<Vec<WebEmailPreview>, ()> {
    let mut imap_session = open_imap_session(keychain_entry, &*folder, &app).await;
    let formated_since = since.format("%d-%b-%Y").to_string();
    println!("Fetching emails since: {}", formated_since);
    let uids = imap_session
        .uid_search(format!("SINCE {}", formated_since))
        .unwrap();

    let mut web_emails: Vec<WebEmailPreview> = vec![];

    for uid in uids {
        let messages_stream = imap_session
            .uid_fetch(format!("{}", uid), "BODY[]")
            .ok()
            .unwrap();
        let message = messages_stream.first();
        if message.is_some() {
            web_emails.push(parse_message(message.unwrap(), folder.clone(), &app));
        }
    }
    imap_session.logout().ok();
    Ok(web_emails)
}

#[tauri::command]
pub fn fetch_message_by_id(app: AppHandle, id: i32) -> WebEmail {
    email_repository::fetch_by_id(id, &app).or_else(|e| {
        error!("Error while fetching email by id: {:?}", e);
        send_snacks(
            "Error while fetching email".to_string(),
            SnackSeverity::Error,
            SnackVertical::Top,
            SnackHorizontal::Right,
            &app,
        );
        Err(())
    }).unwrap()
}

#[tauri::command]
pub async fn fetch_gmail_messages(app: AppHandle) -> Vec<GEmail> {
    let google_keychain_entries = fetch_keychain_entry_google();
    let handle_clone = app.clone();

    let mut mails: Vec<GEmail> = vec![];

    for entry in google_keychain_entries {
        let access_token = renew_token(&handle_clone, &entry.id).await;

        let all_emails_light = fetch_gmail_light_response(&entry, &access_token, &app).await;

        for email_light in all_emails_light.messages {
            mails.push(fetch_gmail_message(&access_token.token, email_light.id, &entry.id, &app).await)
        }
    }

    mails
}
