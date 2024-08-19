use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct Email {
    pub(crate) id: String,
    pub(crate) delivered_at: String,
    pub(crate) from: Vec<String>,
    pub(crate) to: Vec<String>,
    pub(crate) subject: String,
    pub(crate) body: Vec<String>,
    pub(crate) attachments: Vec<Attachment>,
}

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct Attachment {
    pub(crate) filename: String,
    pub(crate) content: Vec<u8>, // base64 encoded
    pub(crate) encoding: String,
}