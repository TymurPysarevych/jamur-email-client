use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebFolders {
    pub folders: Vec<Folder>,
    pub delimiter: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub folder_name: String,
    pub children: Box<Vec<Self>>,
    pub full_path: String,
    pub parent: Option<String>,
}

#[derive(
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    Deserialize,
    Serialize,
    Debug,
    Clone,
    PartialEq
)]
#[diesel(table_name = crate::database::schema::email)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Email {
    pub id: Option<i32>,
    pub email_id: String,
    pub delivered_at: NaiveDateTime,
    pub subject: String,
    pub folder_path: String,
}

impl From<WebEmail> for Email {
    fn from(w: WebEmail) -> Self {
        Self {
            id: w.id,
            email_id: w.email_id,
            delivered_at: w.delivered_at,
            subject: w.subject,
            folder_path: w.folder_path,
        }
    }
}

#[derive(
    Queryable,
    Insertable,
    Identifiable,
    Associations,
    Selectable,
    Deserialize,
    Serialize,
    Debug,
    Clone,
    PartialEq
)]
#[diesel(table_name = crate::database::schema::body)]
#[diesel(belongs_to(Email, foreign_key = email_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Body {
    pub id: Option<i32>,
    pub email_id: Option<i32>,
    pub content: String,
    pub is_html: bool,
}

#[derive(
    PartialEq,
    Clone,
    Queryable,
    Insertable,
    Selectable,
    Identifiable,
    Associations,
    Hash,
    Eq,
    Deserialize,
    Serialize,
    Debug
)]
#[diesel(table_name = crate::database::schema::attachment)]
#[diesel(belongs_to(Email, foreign_key = email_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Attachment {
    pub id: Option<i32>,
    pub filename: String,
    pub content_id: String,
    pub content: Vec<u8>,
    pub encoding: String,
    pub email_id: Option<i32>,
}

#[derive(PartialEq, Clone, Queryable, Insertable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::database::schema::recipient)]
#[diesel(belongs_to(Email, foreign_key = email_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Recipient {
    pub id: Option<i32>,
    pub address: String,
    pub email_id: Option<i32>,
}

#[derive(Clone, Queryable, Insertable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::database::schema::sender)]
#[diesel(belongs_to(Email, foreign_key = email_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Sender {
    pub id: Option<i32>,
    pub address: String,
    pub email_id: Option<i32>,
}


/**
*
* Web structs
*
*/
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebEmail {
    pub id: Option<i32>,
    pub email_id: String,
    pub delivered_at: NaiveDateTime,
    pub from: Vec<String>,
    pub to: Vec<String>,
    pub subject: String,
    pub folder_path: String,
    pub html_bodies: Vec<String>,
    pub text_bodies: Vec<String>,
    pub attachments: Vec<Attachment>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebEmailPreview {
    pub id: Option<i32>,
    pub delivered_at: NaiveDateTime,
    pub from: Vec<String>,
    pub to: Vec<String>,
    pub subject: String,
    pub preview_body: String,
}

impl From<Email> for WebEmail {
    fn from(o: Email) -> Self {
        Self {
            id: o.id,
            email_id: o.email_id,
            delivered_at: o.delivered_at,
            from: vec![],
            to: vec![],
            subject: o.subject,
            folder_path: o.folder_path,
            html_bodies: vec![],
            text_bodies: vec![],
            attachments: vec![],
        }
    }
}
