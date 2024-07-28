use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct Email {
    pub(crate) delivered_at: String,
    pub(crate) to: String,
    pub(crate) subject: String,
    pub(crate) body: String,
}
