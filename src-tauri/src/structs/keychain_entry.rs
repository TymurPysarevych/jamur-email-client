use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Insertable,
    Selectable,
    Debug,
    Eq,
    Hash,
    PartialEq,
    AsChangeset,
)]
#[diesel(table_name = crate::database::schema::keychain_entry)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct KeychainEntry {
    pub key: String,
    pub id: String,
}
