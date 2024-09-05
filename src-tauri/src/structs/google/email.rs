use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Email {
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

#[derive(Deserialize, Debug)]
struct Payload {
    pub mimeType: String,
    pub body: Body,
    pub partId: String,
    pub filename: String,
    pub headers: Vec<Header>,
    pub parts: Option<Vec<Part>>,
}

#[derive(Deserialize, Debug)]
struct Body {
    pub size: u32,
    pub data: Option<String>,
    pub attachmentId: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
struct Part {
    pub mimeType: String,
    pub body: Body,
    pub partId: String,
    pub filename: String,
    pub headers: Vec<Header>,
    pub parts: Option<Vec<Part>>,
}
