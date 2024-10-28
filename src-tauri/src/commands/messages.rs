extern crate chrono;
extern crate encoding_rs;
extern crate imap;
extern crate native_tls;

use crate::commands::google::oauth::renew_token;
use crate::commands::helper::helper_messages::*;
use crate::database::keychain_entry_repository::fetch_keychain_entry_google;
use crate::structs::google::email::GEmail;
use crate::structs::imap_email::{WebEmail, WebEmailPreview};
use crate::structs::keychain_entry::KeychainEntry;
use log::{error, info};
use std::time::Duration;
use chrono::NaiveDateTime;
use diesel::{QueryResult, RunQueryDsl};
use diesel::result::Error;
use imap::types::{Fetch, ZeroCopy};
use tauri::{AppHandle, Manager};
use crate::database::email_repository;
use crate::database::schema::email::folder_path;

#[tauri::command]
pub async fn fetch_messages(app: AppHandle, keychain_entry: KeychainEntry, folder: String) {
    let mut web_emails: Vec<WebEmailPreview> = vec![];

    let mut db_emails = match email_repository::fetch_all_by_folder_path(folder.clone()) {
        Ok(m) => m,
        Err(e) => {
            error!("Error while fetching all emails in folder: {} \n {:?}", folder, e);
            vec![]
        }
    };

    if db_emails.len() == 0 {
        let mut imap_session = open_imap_session(keychain_entry, &*folder).await;
        let messages_stream = imap_session.fetch("1:*", "RFC822").ok();
        imap_session.logout().ok();
        let messages = match messages_stream {
            None => panic!("No messages"),
            Some(m) => m,
        };
        web_emails = messages
            .iter()
            .map(|message| parse_message(message, folder.clone()))
            .collect::<Vec<WebEmailPreview>>();
    } else {
        let last_email = db_emails.first();

        if last_email.is_some() {
            web_emails = fetch_by_query(keychain_entry, last_email.unwrap().delivered_at.clone(), folder).await.unwrap();
        }
    }

    web_emails.extend(db_emails);

    web_emails.sort_by(|a, b| b.delivered_at.cmp(&a.delivered_at));

    app.emit_all("new_emails", web_emails).expect("Could not emit email");
}

#[tauri::command]
pub async fn fetch_by_query(
    keychain_entry: KeychainEntry,
    since: NaiveDateTime,
    folder: String,
) -> Result<Vec<WebEmailPreview>, ()> {
    let mut imap_session = open_imap_session(keychain_entry, &*folder).await;
    let formated_since = since.format("%d-%b-%Y").to_string();
    let uids = imap_session
        .uid_search(format!("NEW SINCE {}", formated_since))
        .unwrap();

    let mut web_emails: Vec<WebEmailPreview> = vec![];

    for uid in uids {
        let messages_stream = imap_session.uid_fetch(format!("{}", uid), "BODY[]").ok().unwrap();
        let message = messages_stream.first();
        if message.is_some() {
            web_emails.push(parse_message(message.unwrap(), folder.clone()));
        }
    }
    imap_session.logout().ok();
    Ok(web_emails)
}

#[tauri::command]
pub async fn fetch_gmail_messages(handle: AppHandle) -> Vec<GEmail> {
    let google_keychain_entries = fetch_keychain_entry_google();
    let handle_clone = handle.clone();

    let mut mails: Vec<GEmail> = vec![];

    for entry in google_keychain_entries {
        let access_token = renew_token(&handle_clone, &entry.id).await;

        let all_emails_light = fetch_gmail_light_response(&entry, &access_token).await;

        for email_light in all_emails_light.messages {
            mails.push(fetch_gmail_message(&access_token.token, email_light.id, &entry.id).await)
        }
    }

    mails
}
