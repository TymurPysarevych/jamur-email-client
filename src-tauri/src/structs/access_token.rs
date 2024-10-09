use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Deserialize, Insertable, Selectable, Debug, Eq, Hash, PartialEq, AsChangeset)]
#[diesel(table_name = crate::database::schema::access_token)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AccessToken {
    pub token: String,
    pub keychain_user: String,
}
