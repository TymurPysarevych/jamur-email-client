use crate::commands::google::oauth::KEYRING_SERVICE_GMAIL_REFRESH_TOKEN;
use crate::database::db_init::establish_connection;
use crate::database::schema::keychain_entry as schema_keychain_entry;
use crate::database::schema::keychain_entry::dsl::keychain_entry;
use crate::structs::keychain_entry::KeychainEntry;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

pub fn fetch_keychain_entry_google() -> Vec<KeychainEntry> {
    let connection = &mut establish_connection();
    let query_result = keychain_entry
        .select(KeychainEntry::as_select())
        .filter(schema_keychain_entry::key.eq(KEYRING_SERVICE_GMAIL_REFRESH_TOKEN))
        .load(connection);
    match query_result {
        Ok(vec) => vec,
        Err(e) => {
            panic!("Error loading keychain entry: {:?}", e);
        }
    }
}
