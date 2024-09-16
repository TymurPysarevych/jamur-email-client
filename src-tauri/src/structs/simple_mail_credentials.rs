use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Deserialize, Insertable, Selectable, Debug, Eq, Hash, PartialEq)]
#[diesel(table_name = crate::database::schema::simple_mail_credentials)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SimpleMailCredentials {
    pub username: String,
    pub keychain_key: String,
    pub imap_host: String,
    pub imap_port: i32,
    pub smtp_host: String,
}
