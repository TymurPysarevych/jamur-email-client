use crate::database::db_init::establish_connection;
use crate::database::schema::email::dsl::email as dsl_email;
use crate::database::schema::{attachment, body, email as schema_email, recipient, sender};
use crate::structs::imap_email::{
    Attachment, Body, Email, Recipient, Sender, WebEmail, WebEmailPreview,
};
use chrono::NaiveDateTime;
use diesel::result::Error;
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use log::error;
use tauri::AppHandle;
use crate::snacks::snacks::send_snacks;
use crate::structs::snack::{SnackHorizontal, SnackSeverity, SnackVertical};

pub fn count_all_by_folder_path(folder_path: String, app: &AppHandle) -> i64 {
    let connection = &mut establish_connection();
    let query_result = dsl_email
        .filter(schema_email::folder_path.eq(folder_path))
        .count()
        .get_result(connection);
    match query_result {
        Ok(count) => count,
        Err(e) => {
            send_snacks(
                "Error counting keychain entries".to_string(),
                SnackSeverity::Error,
                SnackVertical::Top,
                SnackHorizontal::Right,
                &app,
            );
            panic!("Error counting keychain entries: {:?}", e);
        }
    }
}

pub fn fetch_latest_email_by_folder_path(folder_path: String) -> Option<Email> {
    let connection = &mut establish_connection();
    let latest_email = dsl_email
        .filter(schema_email::folder_path.eq(folder_path))
        .select(Email::as_select())
        .order(schema_email::delivered_at.desc())
        .first(connection);
    match latest_email {
        Ok(e) => Some(e),
        Err(_e) => {
            None
        }
    }
}

pub fn fetch_all() -> Result<Vec<WebEmail>, Error> {
    let connection = &mut establish_connection();

    let all_emails = dsl_email
        .select(Email::as_select())
        .load::<Email>(connection)?;

    let attachments = Attachment::belonging_to(&all_emails)
        .select(Attachment::as_select())
        .load::<Attachment>(connection)?;

    let recipients = Recipient::belonging_to(&all_emails)
        .select(Recipient::as_select())
        .load::<Recipient>(connection)?;

    let senders = Sender::belonging_to(&all_emails)
        .select(Sender::as_select())
        .load::<Sender>(connection)?;

    let bodies = Body::belonging_to(&all_emails)
        .select(Body::as_select())
        .load::<Body>(connection)?;

    let mut web_emails = Vec::new();
    for email in all_emails {
        web_emails.push(WebEmail {
            id: email.id,
            folder_path: email.folder_path.clone(),
            subject: email.subject.clone(),
            delivered_at: email.delivered_at.clone(),
            attachments: attachments.iter().map(|a| a.clone()).collect(),
            to: recipients.iter().map(|r| r.address.clone()).collect(),
            from: senders.iter().map(|s| s.address.clone()).collect(),
            html_bodies: bodies
                .iter()
                .filter(|b| b.is_html)
                .map(|b| b.content.clone())
                .collect(),
            text_bodies: bodies
                .iter()
                .filter(|b| !b.is_html)
                .map(|b| b.content.clone())
                .collect(),
            email_id: email.email_id,
        });
    }

    web_emails.sort_by(|a, b| a.delivered_at.cmp(&b.delivered_at));

    Ok(web_emails)
}

pub fn fetch_by_id_preview(id: i32) -> WebEmailPreview {
    let connection = &mut establish_connection();

    let all_emails = dsl_email
        .filter(schema_email::id.eq(id))
        .select(Email::as_select())
        .load::<Email>(connection)
        .unwrap_or_else(|e| {
            error!("Error while fetching email: {:?}", e);
            vec![]
        });

    let email = if all_emails.len() == 0 {
        return panic!("No email with ID: {}", id);
    } else if all_emails.len() > 1 {
        return panic!("Multiple emails with same ID: {}", id);
    } else {
        all_emails.first().unwrap()
    };

    map_email_to_web_email_preview(email.clone())
}

pub fn fetch_by_id(id: i32, app: &AppHandle) -> Result<WebEmail, Error> {
    let connection = &mut establish_connection();

    let all_emails = dsl_email
        .filter(schema_email::id.eq(id))
        .select(Email::as_select())
        .load::<Email>(connection)?;

    let email = if all_emails.len() == 0 {
        send_snacks(
            "No email found".to_string(),
            SnackSeverity::Error,
            SnackVertical::Top,
            SnackHorizontal::Right,
            &app,
        );
        return Err(Error::NotFound);
    } else if all_emails.len() > 1 {
        send_snacks(
            "Multiple emails found".to_string(),
            SnackSeverity::Error,
            SnackVertical::Top,
            SnackHorizontal::Right,
            &app,
        );
        return Err(Error::NotFound);
    } else {
        all_emails.first().unwrap()
    };

    let attachments = Attachment::belonging_to(email)
        .select(Attachment::as_select())
        .load::<Attachment>(connection)
        .unwrap_or_else(|e| {
            error!("Error while fetching attachments: {:?}", e);
            vec![]
        });

    let recipients = Recipient::belonging_to(email)
        .select(Recipient::as_select())
        .load::<Recipient>(connection)
        .unwrap_or_else(|e| {
            error!("Error while fetching recipients: {:?}", e);
            vec![]
        });

    let senders = Sender::belonging_to(email)
        .select(Sender::as_select())
        .load::<Sender>(connection)
        .unwrap_or_else(|e| {
            error!("Error while fetching senders: {:?}", e);
            vec![]
        });

    let bodies = Body::belonging_to(email)
        .select(Body::as_select())
        .load::<Body>(connection)
        .unwrap_or_else(|e| {
            error!("Error while fetching bodies: {:?}", e);
            vec![]
        });

    Ok(WebEmail {
        id: email.id.clone(),
        folder_path: email.folder_path.clone(),
        subject: email.subject.clone(),
        delivered_at: email.delivered_at.clone(),
        attachments: attachments.iter().map(|a| a.clone()).collect(),
        to: recipients.iter().map(|r| r.address.clone()).collect(),
        from: senders.iter().map(|s| s.address.clone()).collect(),
        html_bodies: bodies
            .iter()
            .filter(|b| b.is_html)
            .map(|b| b.content.clone())
            .collect(),
        text_bodies: bodies
            .iter()
            .filter(|b| !b.is_html)
            .map(|b| b.content.clone())
            .collect(),
        email_id: email.email_id.clone(),
    })
}

pub fn fetch_all_by_folder_path(folder_path: String) -> Result<Vec<WebEmailPreview>, Error> {
    let connection = &mut establish_connection();

    let all_emails = dsl_email
        .filter(schema_email::folder_path.eq(folder_path))
        .select(Email::as_select())
        .load::<Email>(connection)?;
    let mut web_emails = Vec::new();

    for email in all_emails {
        web_emails.push(map_email_to_web_email_preview(email));
    }

    web_emails.sort_by(|a, b| a.delivered_at.cmp(&b.delivered_at));

    Ok(web_emails)
}

pub fn save_full_email(web_email: &mut WebEmail) -> Result<(), Error> {
    let conn = &mut establish_connection();

    let db_email = Email::from(web_email.clone());
    diesel::insert_into(dsl_email)
        .values(&db_email)
        .execute(conn)?;

    let inserted_email_id = schema_email::table
        .select(schema_email::id)
        .order(schema_email::id.desc())
        .first::<Option<i32>>(conn)?;

    web_email.id = inserted_email_id;

    for web_attachment in &web_email.attachments {
        let db_attachment = Attachment {
            id: None,
            filename: web_attachment.filename.clone(),
            content_id: web_attachment.content_id.clone(),
            content: web_attachment.content.clone(),
            encoding: web_attachment.encoding.clone(),
            email_id: web_email.id,
        };
        diesel::insert_into(attachment::dsl::attachment)
            .values(&db_attachment)
            .execute(conn)?;
    }

    for recipient_address in &web_email.to {
        let db_recipient = Recipient {
            id: None,
            address: recipient_address.clone(),
            email_id: web_email.id,
        };
        diesel::insert_into(recipient::dsl::recipient)
            .values(&db_recipient)
            .execute(conn)?;
    }

    for sender_address in &web_email.from {
        let db_sender = Sender {
            id: None,
            address: sender_address.clone(),
            email_id: web_email.id,
        };
        diesel::insert_into(sender::table)
            .values(&db_sender)
            .execute(conn)?;
    }

    for html_body in &web_email.html_bodies {
        let db_body = Body {
            id: None,
            email_id: web_email.id,
            content: html_body.clone(),
            is_html: true,
        };
        diesel::insert_into(body::table)
            .values(&db_body)
            .execute(conn)?;
    }

    for text_body in &web_email.text_bodies {
        let db_body = Body {
            id: None,
            email_id: web_email.id,
            content: text_body.clone(),
            is_html: false,
        };
        diesel::insert_into(body::table)
            .values(&db_body)
            .execute(conn)?;
    }

    Ok(())
}

pub fn save_email(db_email: &Email) {
    let connection = &mut establish_connection();
    let result = diesel::insert_into(schema_email::table)
        .values(db_email)
        .execute(connection);
    match result {
        Ok(_) => (),
        Err(e) => {
            error!("Error saving email: {:?}", e);
        }
    }
}

pub fn save_web_email(web_email: &mut WebEmail) {
    match save_full_email(web_email) {
        Ok(_) => (),
        Err(e) => {
            panic!("Error saving email: {:?}", e);
        }
    }
}

pub fn save_web_emails(web_emails: &mut Vec<WebEmail>) {
    for web_email in web_emails {
        save_web_email(web_email)
    }
}

pub fn fetch_latest_email() -> Option<Email> {
    let connection = &mut establish_connection();
    let latest_email = dsl_email
        .select(Email::as_select())
        .order(schema_email::delivered_at.desc())
        .first(connection);
    match latest_email {
        Ok(e) => Some(e),
        Err(e) => {
            error!("Error fetching latest email: {:?}", e);
            None
        }
    }
}

pub fn email_already_exists(id: &String, delivered_at: &NaiveDateTime) -> Option<Email> {
    let connection = &mut establish_connection();
    let email = dsl_email
        .select(Email::as_select())
        .filter(schema_email::email_id.eq(id))
        .filter(schema_email::delivered_at.eq(delivered_at))
        .first(connection);
    match email {
        Ok(e) => Some(e),
        Err(_e) => None,
    }
}

pub fn map_email_to_web_email_preview(email: Email) -> WebEmailPreview {
    let connection = &mut establish_connection();

    let recipients = Recipient::belonging_to(&email)
        .select(Recipient::as_select())
        .load::<Recipient>(connection)
        .unwrap_or_else(|e| {
            error!("Error while fetching recipients: {:?}", e);
            vec![]
        });

    let senders = Sender::belonging_to(&email)
        .select(Sender::as_select())
        .load::<Sender>(connection)
        .unwrap_or_else(|e| {
            error!("Error while fetching senders: {:?}", e);
            vec![]
        });

    let body = Body::belonging_to(&email)
        .select(Body::as_select())
        .filter(body::is_html.eq(false))
        .first::<Body>(connection)
        .unwrap_or_else(|e| {
            error!("Error while fetching body: {:?}", e);
            Body {
                id: None,
                email_id: None,
                content: "".to_string(),
                is_html: false,
            }
        });

    let preview_body = slice_text(&body.content, 150);

    let subject = slice_text(&email.subject, 50);

    WebEmailPreview {
        id: email.id,
        subject,
        delivered_at: email.delivered_at.clone(),
        to: recipients.iter().map(|r| r.address.clone()).collect(),
        from: senders.iter().map(|s| s.address.clone()).collect(),
        preview_body,
    }
}

fn slice_text(text: &String, length: usize) -> String {
    let text_vec = text.chars().collect::<Vec<_>>();
    if text_vec.len() > length {
        format!(
            "{}...",
            text_vec[..length].iter().cloned().collect::<String>()
        )
    } else {
        text.clone()
    }
}
