use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GEmail {
    pub internal_date: String,
    pub history_id: String,
    pub id: String,
    pub snippet: String,
    pub size_estimate: u32,
    pub thread_id: String,
    pub label_ids: Vec<String>,
    pub payload: Payload,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailLight {
    pub id: String,
    pub thread_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailLightResponse {
    pub messages: Vec<EmailLight>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub mime_type: String,
    pub body: Body,
    pub part_id: String,
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
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub size: u32,
    pub data: Option<String>,
    pub attachment_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    pub mime_type: String,
    pub body: Body,
    pub part_id: String,
    pub filename: String,
    pub headers: Vec<Header>,
    pub parts: Option<Vec<Part>>,
}
