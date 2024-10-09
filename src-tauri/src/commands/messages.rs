extern crate chrono;
extern crate encoding_rs;
extern crate imap;
extern crate native_tls;

use crate::commands::google::oauth::renew_token;
use crate::commands::helper::helper_messages::*;
use crate::database::keychain_entry_repository::{
    fetch_keychain_entry_google,
};
use crate::structs::google::email::GEmail;
use crate::structs::imap_email::{WebEmail};
use crate::structs::keychain_entry::KeychainEntry;
use log::{info};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn fetch_messages(app: AppHandle, keychain_entry: KeychainEntry, folder: String) {
    let mut imap_session = open_imap_session(keychain_entry, &*folder).await;

    let messages_stream = imap_session.fetch("1:*", "RFC822").ok();
    imap_session.logout().ok();
    let messages = messages_stream.unwrap();

    let mut web_emails: Vec<WebEmail> = messages
        .iter()
        .map(|message| parse_message(message))
        .collect::<Vec<WebEmail>>();
    web_emails.sort_by(|a, b| b.delivered_at.cmp(&a.delivered_at));
    web_emails.iter().for_each(|email| {
        app.emit_all("new_email", email).expect("Could not emit email");
        thread::sleep(Duration::from_millis(200));
    });
}

#[tauri::command]
pub async fn fetch_by_query(
    keychain_entry: KeychainEntry,
    since: String,
) -> Result<Vec<WebEmail>, ()> {
    let mut imap_session = open_imap_session(keychain_entry, "").await;

    // since = 20-Jul-2024

    let uids = imap_session
        .uid_search(format!("SEEN SINCE {}", since))
        .unwrap();
    info!("{} messages seen since {}", uids.len(), since);

    let mut web_emails: Vec<WebEmail> = vec![];

    for uid in uids {
        let messages_stream = imap_session.uid_fetch(format!("{}", uid), "BODY[]").ok();

        let messages = messages_stream.unwrap();

        let message = messages.first().unwrap();

        web_emails.push(parse_message(message));
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
