extern crate imap;
extern crate native_tls;
extern crate chrono;

use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};
use std::net::TcpStream;
use std::str::from_utf8;
use imap::{Client, Error, Session};
use mail_parser::{Header, Message, MessageParser};
use native_tls::{TlsConnector, TlsStream};

#[tauri::command]
pub async fn fetch_messages(server: &str, login: &str, password: &str) -> String {
    let mut imap_session = open_imap_session(server, login, password).await;

    let message = fetch_all(&mut imap_session).await;

    imap_session.logout().ok();

    format!("{:?}", message)
}

async fn fetch_first(imap_session: &mut Session<TlsStream<TcpStream>>) -> String {
    let option = imap_session.fetch("1", "BODY[]").ok();

    let copy = option.unwrap();
    let option_fetch = copy.first();
    let message = option_fetch.unwrap();

    let body_raw = message.body().expect("message did not have a body!");
    let body = from_utf8(body_raw)
        .expect("message was not valid utf-8")
        .to_string();
    println!("-- 1 message received, logging out");

    let mail = MessageParser::new().parse(&body).unwrap();

    let addresses = mail.from().unwrap().as_list().into_iter().flat_map(|a| a.first()).map(|a| a.address.clone());

    format!(
        "From: {:?}\nSubject: {:?}\n\n{:?}",
        addresses.map(|a| a.unwrap().to_string()).collect::<Vec<String>>().join(", "),
        mail.subject().unwrap(),
        mail.html_body
    )
}

async fn fetch_by_query(imap_session: &mut Session<TlsStream<TcpStream>>) -> Result<usize, Error> {
    let since = "19-Jul-2024";
    let uids = imap_session.uid_search(format!("SEEN SINCE {}", since)).unwrap();
    println!("-- {} messages seen since {}", uids.len(), since);

    for uid in uids {
        let messages_stream = imap_session.uid_fetch(format!("{}", uid), "BODY[]").ok();

        let copy = messages_stream.unwrap();
        let option_fetch = copy.first();
        let message = option_fetch.unwrap();

        let body = from_utf8(message.body().expect("message did not have a body!"))
            .expect("message was not valid utf-8")
            .to_string();
        let mail = MessageParser::new().parse(&body).unwrap();

        let delivered_at = get_deliver_date(&mail);


        let addresses = mail.from().unwrap().as_list().into_iter().flat_map(|a| a.first()).map(|a| a.address.clone());

        println!("{}", format!(
            "From: {:?}\nSubject: {:?}\n\n{:?}",
            addresses.map(|a| a.unwrap().to_string()).collect::<Vec<String>>().join(", "),
            mail.subject().unwrap(),
            mail.text_body
        ));
    }

    Ok(1)
}

async fn fetch_all(imap_session: &mut Session<TlsStream<TcpStream>>) -> String {
    let messages_stream = imap_session.fetch("1:*", "BODY[]").ok();

    let messages = messages_stream.unwrap();

    println!("-- {} messages received", messages.len());
    messages.len().to_string()
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
