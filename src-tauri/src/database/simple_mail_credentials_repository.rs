use crate::database::db_init::establish_connection;
use crate::database::schema::simple_mail_credentials as schema_simple_mail_credentials;
use crate::database::schema::simple_mail_credentials::dsl::simple_mail_credentials;
use crate::structs::simple_mail_credentials::SimpleMailCredentials;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

pub fn fetch_all() -> Vec<SimpleMailCredentials> {
    let connection = &mut establish_connection();
    let query_result = simple_mail_credentials
        .select(SimpleMailCredentials::as_select())
        .load(connection);
    match query_result {
        Ok(vec) => vec,
        Err(e) => {
            panic!("Error loading SimpleMailCredentials: {:?}", e);
        }
    }
}

pub fn fetch_by_keychain_id(id: &str) -> SimpleMailCredentials {
    let connection = &mut establish_connection();
    let query_result = simple_mail_credentials
        .select(SimpleMailCredentials::as_select())
        .filter(schema_simple_mail_credentials::keychain_id.eq(id))
        .first(connection);
    match query_result {
        Ok(entry) => entry,
        Err(e) => {
            panic!("Error loading SimpleMailCredentials: {:?}", e);
        }
    }
}

pub fn count_all() -> i64 {
    let connection = &mut establish_connection();
    let query_result = simple_mail_credentials.count().get_result(connection);
    match query_result {
        Ok(count) => count,
        Err(e) => {
            panic!("Error counting keychain entries: {:?}", e);
        }
    }
}

pub fn save(entry: &SimpleMailCredentials) {
    let connection = &mut establish_connection();
    let query_result = diesel::insert_into(simple_mail_credentials)
        .values(entry)
        .execute(connection);

    match query_result {
        Ok(_) => (),
        Err(e) => {
            panic!("Error inserting SimpleMailCredentials: {:?}", e);
        }
    }
}
