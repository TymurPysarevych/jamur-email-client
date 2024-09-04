use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Email {
    internalDate: String,
    historyId: String,
    id: String,
    snippet: String,
    sizeEstimate: u32,
    threadId: String,
    labelIds: Vec<String>,
    payload: Payload,
}

#[derive(Deserialize, Debug)]
struct Payload {
    mimeType: String,
    body: Body,
    partId: String,
    filename: String,
    headers: Vec<Header>,
    parts: Option<Vec<Part>>,
}

#[derive(Deserialize, Debug)]
struct Body {
    size: u32,
    data: Option<String>,
    attachmentId: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Header {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct Part {
    mimeType: String,
    body: Body,
    partId: String,
    filename: String,
    headers: Vec<Header>,
    parts: Option<Vec<Part>>,
}
