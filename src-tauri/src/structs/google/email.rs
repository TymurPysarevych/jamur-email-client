use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GEmail {
    pub internalDate: String,
    pub historyId: String,
    pub id: String,
    pub snippet: String,
    pub sizeEstimate: u32,
    pub threadId: String,
    pub labelIds: Vec<String>,
    pub payload: Payload,
}

#[derive(Deserialize, Debug)]
pub struct EmailLight {
    pub id: String,
    pub threadId: String,
}

#[derive(Deserialize, Debug)]
pub struct EmailLightResponse {
    pub messages: Vec<EmailLight>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    pub mimeType: String,
    pub body: Body,
    pub partId: String,
    pub filename: String,
    pub headers: Vec<Header>,
    pub parts: Option<Vec<Part>>,
}

impl Payload {
    pub fn set_decoded_parts(&mut self, decoded_parts: Option<Vec<Part>>) {
        self.parts = decoded_parts;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    pub size: u32,
    pub data: Option<String>,
    pub attachmentId: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Part {
    pub mimeType: String,
    pub body: Body,
    pub partId: String,
    pub filename: String,
    pub headers: Vec<Header>,
    pub parts: Option<Vec<Part>>,
}
