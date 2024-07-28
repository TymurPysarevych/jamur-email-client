extern crate chrono;
extern crate imap;
extern crate native_tls;

use std::net::TcpStream;
use std::time::{Duration, UNIX_EPOCH};

use chrono::prelude::DateTime;
use chrono::Utc;
use dotenv::dotenv;
use imap::{Client, Session};
use imap::types::Fetch;
use mail_parser::{Header, Message, MessageParser};
use native_tls::{TlsConnector, TlsStream};

use crate::structs::email::Email;

#[tauri::command]
pub async fn fetch_messages(server: String, login: String, password: String) -> Result<Vec<Email>, ()> {
    dotenv().ok();
    let env_server = std::env::var("SERVER").expect("SERVER must be set.");
    let env_login = std::env::var("LOGIN").expect("LOGIN must be set.");
    let env_password = std::env::var("PASSWORD").expect("PASSWORD must be set.");
    
    let mut imap_session = open_imap_session(&env_server, &env_login, &env_password).await;

    let messages_stream = imap_session.fetch("1:*", "BODY[]").ok();

    let messages = messages_stream.unwrap();

    let web_emails: Vec<Email> = messages.iter()
        .map(|message| {
            parse_message(message)
        }).collect::<Vec<Email>>();

    imap_session.logout().ok();

    Ok(web_emails)
}

#[tauri::command]
pub async fn fetch_by_query(server: String, login: String, password: String, since: String) -> Result<Vec<Email>, ()> {
    let mut imap_session = open_imap_session(server.as_str(), login.as_str(), password.as_str()).await;
    // since = "19-Jul-2024";
    let uids = imap_session.uid_search(format!("SEEN SINCE {}", since)).unwrap();
    println!("-- {} messages seen since {}", uids.len(), since);
    
    let mut web_emails: Vec<Email> = vec![];

    for uid in uids {
        let messages_stream = imap_session.uid_fetch(format!("{}", uid), "BODY[]").ok();

        let messages = messages_stream.unwrap();

        let message = messages.first().unwrap();

        web_emails.push(parse_message(message));
    }

    Ok(web_emails)
}

async fn open_imap_session(server: &str, login: &str, password: &str) -> Session<TlsStream<TcpStream>> {
    let imap_addr = (server, 993);
    let tcp_stream = TcpStream::connect(imap_addr).unwrap();
    let tls = TlsConnector::new().unwrap();
    let tls_stream = tls.connect(server, tcp_stream).unwrap();

    let client = Client::new(tls_stream);
    println!("-- connected to {}:{}", imap_addr.0, imap_addr.1);

    let mut imap_session = client.login(login, password).unwrap();

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
    return "".to_string();
}

fn parse_message(message: &Fetch) -> Email {
    let body_raw = message.body().expect("message did not have a body!");
    let body = body_raw.iter().map(|&c| c as char).collect::<String>();
    let mail = MessageParser::new().parse(&body).unwrap();

    let delivered_at = get_deliver_date(&mail);

    let addresses = mail.from().unwrap().as_list().into_iter().flat_map(|a| a.first()).map(|a| a.address.clone());

    let mail = Email {
        delivered_at,
        to: addresses.map(|a| a.unwrap().to_string()).collect::<Vec<String>>().join(", "),
        subject: mail.subject().unwrap().to_string(),
        body: mail.body_html(0).unwrap().to_string(),
    };
    mail
}
