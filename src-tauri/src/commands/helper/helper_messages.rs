use crate::structs::access_token::AccessToken;
use crate::structs::google::email::{EmailLightResponse, GEmail};
use crate::structs::imap_email::{WebAttachment, WebEmail};
use crate::structs::keychain_entry::KeychainEntry;
use base64::engine::general_purpose;
use base64::Engine;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use chrono::{DateTime, Utc};
use encoding_rs::UTF_8;
use imap::types::Fetch;
use imap::{Client, Session};
use log::{error, warn};
use mail_parser::{BodyPartIterator, Header, Message, MessageParser, MimeHeaders};
use native_tls::{TlsConnector, TlsStream};
use regex::Regex;
use std::collections::HashSet;
use std::net::TcpStream;
use std::time::{Duration, UNIX_EPOCH};

pub async fn open_imap_session(
    host: &str,
    port: u16,
    login: &str,
    password: &str,
) -> Session<TlsStream<TcpStream>> {
    let imap_addr = (host, port);
    let tcp_stream = match TcpStream::connect(imap_addr) {
        Ok(tcp) => tcp,
        Err(e) => {
            error!("Failed to connect to {}:{}", imap_addr.0, imap_addr.1);
            panic!("{e}");
        }
    };
    let tls = match TlsConnector::new() {
        Ok(tls) => tls,
        Err(e) => {
            error!("Failed to create TLS connector");
            panic!("{e}");
        }
    };
    let tls_stream = match tls.connect(host, tcp_stream) {
        Ok(stream) => stream,
        Err(e) => {
            error!("Failed to connect to TLS stream");
            panic!("{e}");
        }
    };

    let client = Client::new(tls_stream);
    let mut imap_session = match client.login(login, password) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to login to IMAP server");
            panic!("{}", e.0);
        }
    };
    imap_session
        .select("INBOX")
        .expect("Failed to select INBOX");
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

pub fn parse_message(message: &Fetch) -> WebEmail {
    let body_raw = message.body().expect("message did not have a body!");
    let message = decode_message(body_raw);

    let all_attachments = build_attachments(&message);
    let mut bodies: Vec<String> = vec![];
    let mut attachments: Vec<WebAttachment> = vec![];

    if message.html_body_count().gt(&0) {
        let html_bodies = fetch_html_bodies(message.html_bodies(), all_attachments);
        bodies.extend(
            html_bodies
                .iter()
                .map(|b| b.0.clone())
                .collect::<Vec<String>>(),
        );
        attachments.extend(
            html_bodies
                .iter()
                .flat_map(|b| b.1.clone())
                .collect::<HashSet<WebAttachment>>()
                .into_iter()
                .collect::<Vec<WebAttachment>>(),
        );
    } else if message.text_body_count().gt(&0) {
        bodies.extend(fetch_text_bodies(message.text_bodies()));
        attachments.extend(all_attachments);
    }

    let delivered_at = get_deliver_date(&message);
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

    let mail = WebEmail {
        id: message.message_id().unwrap().to_string(),
        delivered_at,
        from: from_addresses
            .map(|a| a.unwrap().to_string())
            .collect::<Vec<String>>(),
        to: to_addresses
            .map(|a| a.unwrap().to_string())
            .collect::<Vec<String>>(),
        subject: message.subject().unwrap().to_string(),
        bodies,
        attachments,
    };
    mail
}

fn replace_images(
    mut body: String,
    mut attachments: Vec<WebAttachment>,
) -> (String, Vec<WebAttachment>) {
    let regex = match Regex::new(r#"src="cid:[^"]*""#) {
        Ok(r) => r,
        Err(e) => {
            warn!("Failed to compile regex: {e}");
            return (body.clone(), attachments.clone());
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
            return (body.clone(), attachments.clone());
        }
    };

    let mut replaced_body = "".to_string();
    let mut attachments_to_be_deleted: HashSet<WebAttachment> = HashSet::new();
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
    (body.clone(), attachments.clone())
}

fn build_attachments(message: &Message) -> Vec<WebAttachment> {
    let mut attachments: Vec<WebAttachment> = vec![];

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

            attachments.push(WebAttachment {
                filename,
                content_id,
                content: part.contents().to_vec(),
                encoding,
            });
        }
    });

    attachments
}

fn decode_message(body_raw: &[u8]) -> Message {
    let mail = MessageParser::default()
        .parse(body_raw)
        .ok_or_else(|| {
            println!("Failed to parse message");
        })
        .unwrap();

    mail.into_owned()
}

fn fetch_html_bodies(
    iterator: BodyPartIterator,
    attachments: Vec<WebAttachment>,
) -> Vec<(String, Vec<WebAttachment>)> {
    let mut bodies: Vec<(String, Vec<WebAttachment>)> = vec![];

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

pub async fn fetch_gmail_light_response(
    entry: &KeychainEntry,
    access_token: &AccessToken,
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
            panic!("Error getting all messages from Google: {:?}", error);
        }
    };

    match all_gmails_response.json::<EmailLightResponse>().await {
        Ok(vec) => vec,
        Err(e) => {
            panic!("Error parsing JSON: {:?}", e);
        }
    }
}

pub async fn fetch_gmail_message(access_token: &String, id: String, user: &String) -> GEmail {
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
            panic!("Error getting message from Google: {:?}", error);
        }
    };

    let mut mail_raw: GEmail = match message_response.json::<GEmail>().await {
        Ok(gemail) => gemail,
        Err(e) => {
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
