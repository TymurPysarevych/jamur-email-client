use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Insertable, Selectable, Debug, Eq, Hash, PartialEq, Clone, AsChangeset)]
#[diesel(table_name = crate::database::schema::simple_mail_credentials)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct SimpleMailCredentials {
    pub username: String,
    pub keychain_id: String,
    pub imap_host: String,
    pub imap_port: i32,
    pub smtp_host: String,
    pub smtp_port: i32,
}

#[derive(Deserialize,Serialize, Debug, Eq, Hash, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebSimpleMailCredentials {
    pub config: SimpleMailCredentials,
    pub password: String,
}
