use crate::database::db_init::establish_connection;
use crate::database::schema::access_token as schema_access_token;
use crate::database::schema::access_token::dsl::access_token;
use crate::structs::access_token::AccessToken;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

pub fn fetch_access_token_google(user: String) -> AccessToken {
    let connection = &mut establish_connection();
    match access_token
        .select(AccessToken::as_select())
        .filter(schema_access_token::keychain_user.eq(user))
        .first(connection)
    {
        Ok(a) => a,
        Err(e) => {
            panic!("Error fetching access token: {:?}", e);
        }
    }
}

pub fn save_access_token_google(token: &AccessToken) -> AccessToken {
    let connection = &mut establish_connection();
    let query_result = diesel::update(access_token)
        .filter(schema_access_token::keychain_user.eq(&token.keychain_user))
        .set(schema_access_token::token.eq(&token.token))
        .execute(connection);

    match query_result {
        Ok(_) => (),
        Err(e) => {
            panic!("Error inserting access token: {:?}", e);
        }
    };

    let user = token.keychain_user.clone();

    fetch_access_token_google(user)
}
