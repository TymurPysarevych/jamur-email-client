use crate::commands::google::oauth::KEYRING_SERVICE_GMAIL_REFRESH_TOKEN;
use crate::database::db_init::establish_connection;
use crate::database::schema::keychain_entry as schema_keychain_entry;
use crate::database::schema::keychain_entry::dsl::keychain_entry;
use crate::structs::keychain_entry::KeychainEntry;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

pub fn fetch_all() -> Vec<KeychainEntry> {
    let connection = &mut establish_connection();
    let query_result = keychain_entry
        .select(KeychainEntry::as_select())
        .load(connection);
    match query_result {
        Ok(vec) => vec,
        Err(e) => {
            panic!("Error loading keychain entries: {:?}", e);
        }
    }
}

pub fn count_all() -> i64 {
    let connection = &mut establish_connection();
    let query_result = keychain_entry.count().get_result(connection);
    match query_result {
        Ok(count) => count,
        Err(e) => {
            panic!("Error counting keychain entries: {:?}", e);
        }
    }
}

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

pub fn fetch_keychain_entry_google_for_user(user: &str) -> Option<KeychainEntry> {
    let connection = &mut establish_connection();
    let query_result = keychain_entry
        .select(KeychainEntry::as_select())
        .filter(
            schema_keychain_entry::key
                .eq(KEYRING_SERVICE_GMAIL_REFRESH_TOKEN)
                .and(schema_keychain_entry::user.eq(user)),
        )
        .first(connection);

    if query_result.is_ok() {
        return Some(query_result.unwrap());
    }
    None
}

pub fn save_keychain_entry_google(entry: &KeychainEntry) -> Option<KeychainEntry> {
    let optional_keychain_entry = fetch_keychain_entry_google_for_user(&entry.user);
    if optional_keychain_entry.is_some() {
        return optional_keychain_entry;
    }

    let connection = &mut establish_connection();
    let query_result = diesel::insert_into(keychain_entry)
        .values(entry)
        .execute(connection);

    match query_result {
        Ok(_) => (),
        Err(e) => {
            panic!("Error inserting keychain entry: {:?}", e);
        }
    }

    fetch_keychain_entry_google_for_user(&entry.user)
}
