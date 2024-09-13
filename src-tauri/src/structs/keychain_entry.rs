use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Deserialize, Insertable, Selectable, Debug, Eq, Hash, PartialEq)]
#[diesel(table_name = crate::database::schema::keychain_entry)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct KeychainEntry {
    pub key: String,
    pub user: String,
}