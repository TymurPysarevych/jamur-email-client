use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::database::schema::keychain_entry)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct KeychainEntry {
    pub key: String,
    pub user: String,
}
