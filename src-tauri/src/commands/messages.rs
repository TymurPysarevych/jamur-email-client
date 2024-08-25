extern crate chrono;
extern crate encoding_rs;
extern crate imap;
extern crate native_tls;

use crate::structs::email::{Attachment, Email};
use base64::engine::general_purpose;
use base64::Engine;
use chrono::prelude::DateTime;
use chrono::Utc;
use dotenv::dotenv;
use encoding_rs::*;
use imap::types::Fetch;
use imap::{Client, Session};
use log::{debug, info, warn};
use mail_parser::{BodyPartIterator, Header, Message, MessageParser, MimeHeaders};
use native_tls::{TlsConnector, TlsStream};
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::env::var;
use std::net::TcpStream;
use std::string::ToString;
use std::time::{Duration, UNIX_EPOCH};

#[tauri::command]
pub async fn fetch_messages(_server: String, _login: String, _password: String) -> Result<Vec<Email>, ()> {
    dotenv().ok();
    let env_server = var("SERVER").expect("SERVER must be set.");
    let env_login = var("LOGIN").expect("LOGIN must be set.");
    let env_password = var("PASSWORD").expect("PASSWORD must be set.");

    let mut imap_session = open_imap_session(&env_server, &env_login, &env_password).await;

    let messages_stream = imap_session.fetch("1:*", "RFC822").ok();
    info!("fetched messages");
    imap_session.logout().ok();
    info!("logged out");

    let messages = messages_stream.unwrap();

    let web_emails: Vec<Email> = messages.iter()
        .map(|message| {
            parse_message(message)
        }).collect::<Vec<Email>>();
    // web_emails.sort_by(|a, b| b.delivered_at.cmp(&a.delivered_at));

    Ok(web_emails)
}

#[tauri::command]
pub async fn fetch_by_query(_server: String, _login: String, _password: String, since: String) -> Result<Vec<Email>, ()> {
    dotenv().ok();
    let env_server = var("SERVER").expect("SERVER must be set.");
    let env_login = var("LOGIN").expect("LOGIN must be set.");
    let env_password = var("PASSWORD").expect("PASSWORD must be set.");
    let mut imap_session = open_imap_session(env_server.as_str(), env_login.as_str(), env_password.as_str()).await;

    // since = 20-Jul-2024

    let uids = imap_session.uid_search(format!("SEEN SINCE {}", since)).unwrap();
    info!("{} messages seen since {}", uids.len(), since);

    let mut web_emails: Vec<Email> = vec![];

    for uid in uids {
        let messages_stream = imap_session.uid_fetch(format!("{}", uid), "BODY[]").ok();

        let messages = messages_stream.unwrap();

        let message = messages.first().unwrap();

        web_emails.push(parse_message(message));
    }

    imap_session.logout().ok();

    Ok(web_emails)
}

async fn open_imap_session(server: &str, login: &str, password: &str) -> Session<TlsStream<TcpStream>> {
    let imap_addr = (server, 993);
    let tcp_stream = TcpStream::connect(imap_addr).unwrap();
    let tls = TlsConnector::new().unwrap();
    let tls_stream = tls.connect(server, tcp_stream).unwrap();

    let client = Client::new(tls_stream);
    info!("connected to {}:{}", imap_addr.0, imap_addr.1);

    let mut imap_session = client.login(login, password).unwrap();

    info!("fetching messages from INBOX");
    imap_session.select("INBOX").unwrap();
    imap_session
}

fn get_deliver_date(mail: &Message) -> String {
    let mut delivered: Option<Header> = None;

    for part in mail.clone().parts {
        for header in part.headers {
            if header.name().eq("X-Delivery-Time") {
                delivered = Some(header.clone());
                break;
            }
        }
    }

    let d = delivered.clone();

    if d.is_some() {
        let epoch = d.unwrap().value.as_text().unwrap().parse::<u64>().unwrap();
        let d = UNIX_EPOCH + Duration::from_secs(epoch);
        let datetime = DateTime::<Utc>::from(d);
        return datetime.format("%d-%m-%Y %H:%M:%S").to_string();
    }
    "".to_string()
}

fn parse_message(message: &Fetch) -> Email {
    let body_raw = message.body().expect("message did not have a body!");
    let message = decode_message(body_raw);

    let all_attachments = build_attachments(&message);
    let mut bodies: Vec<String> = vec![];
    let mut attachments: Vec<Attachment> = vec![];

    if message.html_body_count().gt(&0) {
        let html_bodies = fetch_html_bodies(message.html_bodies(), all_attachments);
        bodies.extend(html_bodies.iter().map(|b| b.0.clone()).collect::<Vec<String>>());
        attachments.extend(html_bodies.iter().flat_map(|b| b.1.clone()).collect::<HashSet<Attachment>>().into_iter().collect::<Vec<Attachment>>());
    } else if message.text_body_count().gt(&0) {
        bodies.extend(fetch_text_bodies(message.text_bodies()));
        attachments.extend(all_attachments);
    }

    let delivered_at = get_deliver_date(&message);
    let from_addresses = message.from().unwrap().as_list().into_iter().flat_map(|a| a.first()).map(|a| a.address.clone());
    let to_addresses = message.to().unwrap().as_list().into_iter().flat_map(|a| a.first()).map(|a| a.address.clone());

    let mail = Email {
        id: message.message_id().unwrap().to_string(),
        delivered_at,
        from: from_addresses.map(|a| a.unwrap().to_string()).collect::<Vec<String>>(),
        to: to_addresses.map(|a| a.unwrap().to_string()).collect::<Vec<String>>(),
        subject: message.subject().unwrap().to_string(),
        bodies,
        attachments,
    };
    mail
}

fn replace_images(mut body: String, mut attachments: Vec<Attachment>) -> (String, Vec<Attachment>) {
    let regex = Regex::new(r#"src="cid:[^"]*\.[a-zA-Z0-9]{3,4}""#).unwrap();

    let mut replaced_body = "".to_string();
    let mut caps = regex.captures_iter(&*body).map(|cap| cap[0].to_string()).collect::<HashSet<String>>();
    caps.into_iter().for_each(|cid| {
        replaced_body = body.clone();
        let clean_cid = cid.replace("src=\"cid:", "").replace("\"", "");

        let file_ending = clean_cid.split(".").last().unwrap();

        let optional_attachment = attachments.iter().find(|a| a.filename.eq(&clean_cid));
        if optional_attachment.is_some() {
            let attachment = optional_attachment.unwrap();
            let content = general_purpose::STANDARD.encode(&attachment.content);

            let cid_replacement = format!("src=\"data:image/{};base64,{}\"", file_ending, content);
            replaced_body = body.replace(&cid, &cid_replacement);
            body = replaced_body.clone();
            attachments = attachments.iter().cloned().filter(|a| !(a.filename == clean_cid)).collect::<Vec<Attachment>>();
        } else {
            debug!("Attachment with cid {clean_cid} not found");
            replaced_body = body.clone();
        }
    });
    (body.clone(), attachments.clone())
}

fn build_attachments(message: &Message) -> Vec<Attachment> {
    let mut attachments: Vec<Attachment> = vec![];

    message.attachments.iter().for_each(|attachment_index| {
        let optional_part = message.parts.get(*attachment_index);
        if optional_part.is_some() {
            let mut filename = "".to_string();

            let part = optional_part.unwrap();

            for header in part.headers.iter() {
                if header.name().eq("Content-Type") {
                    let optional_header_content_type = header.value.as_content_type();
                    if optional_header_content_type.is_some() {
                        let optional_attributes = optional_header_content_type.unwrap().attributes();
                        if optional_attributes.is_some() {
                            let attributes = optional_attributes.unwrap();
                            let filename_attribute = attributes.iter().find(|a| a.0.eq("name"));
                            if filename_attribute.is_some() {
                                filename = filename_attribute.unwrap().1.to_string();
                                break;
                            }
                        }
                    }
                }
            }

            let encoding = part.content_transfer_encoding().or(Some("")).unwrap().to_string();

            attachments.push(Attachment {
                filename,
                content: part.contents().to_vec(),
                encoding,
            });
        }
    });

    attachments
}

fn decode_message(body_raw: &[u8]) -> Message {
    let mail = MessageParser::default().parse(body_raw).ok_or_else(|| {
        println!("Failed to parse message");
    }).unwrap();

    mail.into_owned()
}

fn fetch_html_bodies(iterator: BodyPartIterator, attachments: Vec<Attachment>) -> Vec<(String, Vec<Attachment>)> {
    let mut bodies: Vec<(String, Vec<Attachment>)> = vec![];

    iterator.for_each(|b| {
        let (cow, _, _) = UTF_8.decode(&b.contents());
        bodies.push(replace_images(cow.to_string(), attachments.clone()));
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
