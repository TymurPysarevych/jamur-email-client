use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebEmail {
    pub id: String,
    pub delivered_at: String,
    pub from: Vec<String>,
    pub to: Vec<String>,
    pub subject: String,
    pub bodies: Vec<String>,
    pub attachments: Vec<WebAttachment>,
}

#[derive(Eq, Hash, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebAttachment {
    pub filename: String,
    pub content_id: String,
    pub content: Vec<u8>,
    pub encoding: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::database::schema::email)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Email {
    pub id: i32,
    pub email_id: String,
    pub delivered_at: String,
    pub subject: String,
}

#[derive(PartialEq, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::database::schema::attachment)]
#[diesel(belongs_to(Email, foreign_key = email_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Attachment {
    pub id: i32,
    pub filename: String,
    pub content_id: String,
    pub content: Vec<u8>,
    pub encoding: String,
    pub email_id: i32,
}

#[derive(PartialEq, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::database::schema::recipient)]
#[diesel(belongs_to(Email, foreign_key = email_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Recipient {
    pub id: i32,
    pub address: String,
    pub email_id: i32,
}

#[derive(Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::database::schema::sender)]
#[diesel(belongs_to(Email, foreign_key = email_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Sender {
    pub id: i32,
    pub address: String,
    pub email_id: i32,
}
