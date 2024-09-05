extern crate chrono;
extern crate encoding_rs;
extern crate imap;
extern crate native_tls;

use crate::commands::google::oauth::renew_token;
use crate::commands::helper::helper_messages::*;
use crate::database::keychain_entry_repository::fetch_keychain_entry_google;
use crate::structs::access_token::AccessToken;
use crate::structs::google::email::{EmailLight, EmailLightResponse};
use crate::structs::imap_email::WebEmail;
use crate::structs::keychain_entry::KeychainEntry;
use dotenv::dotenv;
use log::info;
use reqwest::Client;
use std::env::var;

#[tauri::command]
pub async fn fetch_messages(
    server: String,
    login: String,
    password: String,
) -> Result<Vec<WebEmail>, ()> {
    let mut imap_session = open_imap_session(&server, &login, &password).await;

    let messages_stream = imap_session.fetch("1:*", "RFC822").ok();
    imap_session.logout().ok();
    let messages = messages_stream.unwrap();

    let web_emails: Vec<WebEmail> = messages
        .iter()
        .map(|message| parse_message(message))
        .collect::<Vec<WebEmail>>();
    // web_emails.sort_by(|a, b| b.delivered_at.cmp(&a.delivered_at));
    Ok(web_emails)
}

#[tauri::command]
pub async fn fetch_by_query(
    _server: String,
    _login: String,
    _password: String,
    since: String,
) -> Result<Vec<WebEmail>, ()> {
    dotenv().ok();
    let env_server = var("SERVER").expect("SERVER must be set.");
    let env_login = var("LOGIN").expect("LOGIN must be set.");
    let env_password = var("PASSWORD").expect("PASSWORD must be set.");
    let mut imap_session = open_imap_session(
        env_server.as_str(),
        env_login.as_str(),
        env_password.as_str(),
    )
    .await;

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
pub async fn fetch_gmail_messages(handle: tauri::AppHandle) -> Result<(), ()> {
    let google_keychain_entries = fetch_keychain_entry_google();
    let handle_clone = handle.clone();
    for entry in google_keychain_entries {
        let access_token = renew_token(&handle_clone, &entry.user).await;

        let all_emails_light = fetch_gmail_light_response(entry, access_token).await;

        // details           https://gmail.googleapis.com/gmail/v1/users/{entry.user}/messages/{id}
    }
    Ok(())
}
