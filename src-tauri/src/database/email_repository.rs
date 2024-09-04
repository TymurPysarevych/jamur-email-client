use crate::database::db_init::establish_connection;
use crate::database::schema::email as schema_email;
use crate::database::schema::email::dsl::email;
use crate::structs::imap_email::{Attachment, Email};
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use log::error;

pub fn fetch_all() -> Vec<Email> {
    let connection = &mut establish_connection();
    let all_emails = email
        .select(Email::as_select())
        .load(connection)
        .expect("Error loading posts");
    println!("Displaying {} emails", all_emails.len());

    let attachments = Attachment::belonging_to(&all_emails)
        .select(Attachment::as_select())
        .load(connection)
        .expect("Error loading attachments");

    all_emails
}

pub fn fetch_latest_email() -> Option<Email> {
    let connection = &mut establish_connection();
    let latest_email = email
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
