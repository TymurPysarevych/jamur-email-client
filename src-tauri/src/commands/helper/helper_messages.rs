use crate::commands::helper::helper_keyring::fetch_keyring_entry;
use crate::database::email_repository;
use crate::database::keychain_entry_repository::KEYCHAIN_KEY_IMAP_PASSWORD;
use crate::database::simple_mail_credentials_repository::fetch_by_keychain_id;
use crate::structs::access_token::AccessToken;
use crate::structs::google::email::{EmailLightResponse, GEmail};
use crate::structs::imap_email::{Attachment, WebEmail, WebEmailPreview};
use crate::structs::keychain_entry::KeychainEntry;
use base64::engine::general_purpose;
use base64::engine::general_purpose::URL_SAFE;
use base64::Engine;
use chrono::{DateTime, NaiveDateTime};
use encoding_rs::UTF_8;
use imap::types::Fetch;
use imap::{Client, Session};
use log::{error, warn};
use mail_parser::{BodyPartIterator, Message, MessageParser, MimeHeaders};
use native_tls::{TlsConnector, TlsStream};
use regex::Regex;
use std::collections::HashSet;
use std::net::TcpStream;
use tauri::AppHandle;
use crate::snacks::snacks::send_snacks;
use crate::structs::snack::{SnackHorizontal, SnackSeverity, SnackVertical};

async fn login_imap_session(
    host: &str,
    port: u16,
    login: &str,
    password: &str,
    folder: &str,
    app: &AppHandle
) -> Session<TlsStream<TcpStream>> {
    let imap_addr = (host, port);
    let tcp_stream = match TcpStream::connect(imap_addr) {
        Ok(tcp) => tcp,
        Err(e) => {
            send_snacks(
                format!("Failed to connect to {}:{}", imap_addr.0, imap_addr.1),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            error!("Failed to connect to {}:{}", imap_addr.0, imap_addr.1);
            panic!("{e}");
        }
    };
    let tls = match TlsConnector::new() {
        Ok(tls) => tls,
        Err(e) => {
            send_snacks(
                "Failed to create TLS connector".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            error!("Failed to create TLS connector");
            panic!("{e}");
        }
    };
    let tls_stream = match tls.connect(host, tcp_stream) {
        Ok(stream) => stream,
        Err(e) => {
            send_snacks(
                "Failed to connect to TLS stream".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            error!("Failed to connect to TLS stream");
            panic!("{e}");
        }
    };

    let client = Client::new(tls_stream);
    let mut imap_session = match client.login(login, password) {
        Ok(s) => s,
        Err(e) => {
            send_snacks(
                "Failed to login to IMAP server".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            error!("Failed to login to IMAP server");
            panic!("{}", e.0);
        }
    };

    if folder.eq("") {
        imap_session
            .select("INBOX").unwrap_or_else(|e| {
                send_snacks(
                    "Failed to select INBOX".to_string(),
                    SnackSeverity::Error,
                    SnackVertical::Top,
                    SnackHorizontal::Right,
                    &app,
                );
                panic!("{e}");
            });
    } else {
        imap_session.select(folder).unwrap_or_else(|e| {
            send_snacks(
                format!("Failed to select folder {}", folder),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("{e}");
        });
    }

    imap_session
}

pub async fn open_imap_session(
    keychain_entry: KeychainEntry,
    folder: &str,
    app: &AppHandle
) -> Session<TlsStream<TcpStream>> {
    let simple_mail_creds = fetch_by_keychain_id(&keychain_entry.id);
    let login = &simple_mail_creds.username;
    let password = &fetch_keyring_entry(KEYCHAIN_KEY_IMAP_PASSWORD, &keychain_entry.id, &app);
    let port = match u16::try_from(simple_mail_creds.imap_port) {
        Ok(p) => p,
        Err(e) => {
            panic!("{}", e);
        }
    };
    let host = &*simple_mail_creds.imap_host;

    login_imap_session(host, port, login, password, folder, app).await
}

fn get_deliver_date(mail: &Message) -> NaiveDateTime {
    for part in mail.clone().parts {
        for header in part.headers {
            if header.name().eq("X-Delivery-Time") {
                let delivered = header.clone();

                let str_timestamp = delivered.value.as_text().unwrap();
                let timestamp = str_timestamp.parse::<i64>().unwrap();
                let date_time = match DateTime::from_timestamp(timestamp, 0) {
                    None => panic!("Failed to parse date"),
                    Some(d) => d,
                };
                return date_time.naive_utc();
            } else if header.name().eq("Received") {
                let option_received = header.value.as_received().unwrap().date;
                if option_received.is_some() {
                    let received = option_received.unwrap();
                    let date_time = match DateTime::from_timestamp(received.to_timestamp(), 0) {
                        None => panic!("Failed to parse date"),
                        Some(d) => d,
                    };
                    return date_time.naive_utc();
                }
            } else if header.name().eq("Date") {
                let date = header.value.as_datetime().unwrap();
                let date_time = match DateTime::from_timestamp(date.to_timestamp(), 0) {
                    None => panic!("Failed to parse date"),
                    Some(d) => d,
                };
                return date_time.naive_utc();
            }
        }
    }

    panic!("Failed to find delivery date");
}

pub fn parse_message(message: &Fetch, folder_path: String, app: &AppHandle) -> WebEmailPreview {
    let body_raw = message.body().expect("message did not have a body!");
    let message = decode_message(body_raw, app);

    let message_id = match message.message_id() {
        None => "".to_string(),
        Some(s) => s.to_string(),
    };
    let delivered_at = get_deliver_date(&message);
    let is_new_email = email_repository::email_already_exists(&message_id, &delivered_at);

    if is_new_email.is_none() {
        let mut all_attachments = build_attachments(&message);
        let mut html_bodies: Vec<String> = vec![];
        let mut text_bodies: Vec<String> = vec![];

        if message.html_body_count().gt(&0) {
            let bodies = fetch_html_bodies(message.html_bodies(), &mut all_attachments);
            html_bodies.extend(bodies);
        }

        if message.text_body_count().gt(&0) {
            text_bodies.extend(fetch_text_bodies(message.text_bodies()));
        }

        let from_addresses = message
            .from()
            .unwrap()
            .as_list()
            .into_iter()
            .flat_map(|a| a.first())
            .map(|a| a.address.clone());
        let to_addresses = message
            .to()
            .unwrap()
            .as_list()
            .into_iter()
            .flat_map(|a| a.first())
            .map(|a| a.address.clone());

        let subject = match message.subject() {
            None => "".to_string(),
            Some(s) => s.to_string(),
        };

        let mut mail = WebEmail {
            id: None,
            email_id: message_id.clone(),
            delivered_at,
            from: from_addresses
                .map(|a| a.unwrap().to_string())
                .collect::<Vec<String>>(),
            to: to_addresses
                .map(|a| a.unwrap().to_string())
                .collect::<Vec<String>>(),
            subject,
            folder_path,
            html_bodies,
            text_bodies,
            attachments: all_attachments,
        };

        match email_repository::save_full_email(&mut mail) {
            Ok(_) => {}
            Err(e) => {
                send_snacks(
                    format!("Error while saving email: {}", mail.subject),
                    SnackSeverity::Error,
                    SnackVertical::Top,
                    SnackHorizontal::Right,
                    &app,
                );
                panic!("Error while saving email: {} \n {}", mail.subject, e)
            }
        };

        email_repository::fetch_by_id_preview(mail.id.unwrap())
    } else {
        email_repository::fetch_by_id_preview(is_new_email.unwrap().id.unwrap())
    }
}

fn replace_images(mut body: String, attachments: &mut Vec<Attachment>) -> String {
    let regex = match Regex::new(r#"src="cid:[^"]*""#) {
        Ok(r) => r,
        Err(e) => {
            warn!("Failed to compile regex: {e}");
            return body.clone();
        }
    };

    let caps = regex
        .captures_iter(&*body)
        .map(|cap| cap[0].to_string())
        .collect::<HashSet<String>>();
    let file_regex = match Regex::new(r#":([^@]+)@*"#) {
        Ok(r) => r,
        Err(e) => {
            warn!("Failed to compile regex: {e}");
            return body.clone();
        }
    };

    let mut replaced_body = "".to_string();
    let mut attachments_to_be_deleted: HashSet<Attachment> = HashSet::new();
    caps.into_iter().for_each(|cap| {
        replaced_body = body.clone();

        let filename_captures = match file_regex.captures(&*cap) {
            None => {
                error!("Failed to find filename in {cap}");
                return;
            }
            Some(f) => f,
        };

        let filename = match filename_captures.get(1) {
            None => "".to_string(),
            Some(f) => f.as_str().to_string(),
        };

        let clean_cid = filename.replace("\"", "");

        let file_ending = clean_cid.split(".").last().unwrap();

        let optional_attachment = attachments.iter().find(|a| {
            clean_cid.contains(a.filename.as_str()) || clean_cid.contains(a.content_id.as_str())
        });
        if optional_attachment.is_some() {
            let attachment = optional_attachment.unwrap();
            let content = general_purpose::STANDARD.encode(&attachment.content);

            let cid_replacement = format!("src=\"data:image/{};base64,{}\"", file_ending, content);
            replaced_body = body.replace(&cap, &cid_replacement);
            body = replaced_body.clone();
            attachments_to_be_deleted.insert(attachment.clone());
        } else {
            // if this happens, the attachment was lost in the process.
            // Probably due to email servers not sending the attachment as expected.
            replaced_body = body.clone();
        }
    });
    attachments.retain(|a| !attachments_to_be_deleted.contains(a));
    body.clone()
}

fn build_attachments(message: &Message) -> Vec<Attachment> {
    let mut attachments: Vec<Attachment> = vec![];

    message.attachments.iter().for_each(|attachment_index| {
        let optional_part = message.parts.get(*attachment_index);
        if optional_part.is_some() {
            let mut filename = "".to_string();
            let mut content_id = "".to_string();

            let part = optional_part.unwrap();

            for header in part.headers.iter() {
                if header.name().eq("Content-Type") || header.name().eq("Content-Disposition") {
                    let optional_header_content_type = header.value.as_content_type();
                    if optional_header_content_type.is_some() {
                        let optional_attributes =
                            optional_header_content_type.unwrap().attributes();
                        if optional_attributes.is_some() {
                            let attributes = optional_attributes.unwrap();
                            let filename_attribute = attributes
                                .iter()
                                .find(|a| a.0.eq("name") || a.0.eq("filename"));
                            if filename_attribute.is_some() {
                                filename = filename_attribute.unwrap().1.to_string();
                            }
                        }
                    }
                } else if header.name().eq("Content-ID") {
                    let optional_header_content_id = header.value.as_text_list();
                    if optional_header_content_id.is_some() {
                        content_id = match optional_header_content_id.unwrap().first() {
                            None => {
                                panic!("Content-ID header was empty")
                            }
                            Some(s) => s.to_string(),
                        }
                    }
                }
                if !filename.is_empty() && !content_id.is_empty() {
                    break;
                }
            }

            let encoding = part
                .content_transfer_encoding()
                .or(Some(""))
                .unwrap()
                .to_string();

            attachments.push(Attachment {
                id: None,
                filename,
                content_id,
                content: part.contents().to_vec(),
                encoding,
                email_id: None,
            });
        }
    });

    attachments
}

fn decode_message<'a>(body_raw: &'a [u8], app: &'a AppHandle) -> Message<'a> {
    let mail = MessageParser::default()
        .parse(body_raw)
        .ok_or_else(|| {
            error!("Failed to parse email");
            send_snacks(
                "Failed to parse email".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
        })
        .unwrap();

    mail.into_owned()
}

fn fetch_html_bodies(iterator: BodyPartIterator, attachments: &mut Vec<Attachment>) -> Vec<String> {
    let mut bodies: Vec<String> = vec![];

    iterator.for_each(|b| {
        let (cow, _, _) = UTF_8.decode(&b.contents());
        bodies.push(replace_images(cow.to_string(), attachments));
    });

    bodies
}

fn fetch_text_bodies(iterator: BodyPartIterator) -> Vec<String> {
    let mut bodies = vec![];

    iterator.for_each(|b| {
        let body = b.contents();
        let (cow, _, _) = UTF_8.decode(&body);
        bodies.push(cow.to_string());
    });

    bodies
}

pub async fn fetch_gmail_light_response(
    entry: &KeychainEntry,
    access_token: &AccessToken,
    app: &AppHandle,
) -> EmailLightResponse {
    let uri_all_gmails = format!(
        "https://gmail.googleapis.com/gmail/v1/users/{}/messages",
        entry.id
    );

    let client = reqwest::Client::new();

    let all_gmails = client
        .get(uri_all_gmails)
        .header("Authorization", format!("Bearer {}", access_token.token))
        .send()
        .await;

    let all_gmails_response = match all_gmails {
        Ok(response) => response,
        Err(error) => {
            send_snacks(
                "Error getting all messages from Google".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Error getting all messages from Google: {:?}", error);
        }
    };

    match all_gmails_response.json::<EmailLightResponse>().await {
        Ok(vec) => vec,
        Err(e) => {
            send_snacks(
                "Error parsing JSON".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Error parsing JSON: {:?}", e);
        }
    }
}

pub async fn fetch_gmail_message(access_token: &String, id: String, user: &String, app: &AppHandle) -> GEmail {
    let uri = format!(
        "https://gmail.googleapis.com/gmail/v1/users/{}/messages/{}",
        user, id
    );

    let client = reqwest::Client::new();

    let message = client
        .get(uri)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await;

    let message_response = match message {
        Ok(response) => response,
        Err(error) => {
            send_snacks(
                "Error getting message from Google".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Error getting message from Google: {:?}", error);
        }
    };

    let mut mail_raw: GEmail = match message_response.json::<GEmail>().await {
        Ok(gemail) => gemail,
        Err(e) => {
            send_snacks(
                "Error parsing JSON".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Error parsing JSON: {:?}", e);
        }
    };

    let mut parts = mail_raw.payload.parts.clone();
    for part in parts.iter_mut().flatten() {
        if let Some(body) = &part.body.data {
            if let Ok(decoded) = URL_SAFE.decode(body) {
                if let Ok(body_str) = String::from_utf8(decoded) {
                    part.body.data = Some(body_str);
                } else {
                    error!("Failed to convert body to UTF-8 string");
                }
            } else {
                error!("Failed to decode body");
            }
        }
    }

    mail_raw.payload.set_decoded_parts(parts);

    mail_raw
}
