export interface GEmail {
    internalDate: string;
    historyId: string;
    id: string;
    snippet: string;
    sizeEstimate: number;
    threadId: string;
    labelIds: string[];
    payload: Payload;
}

export interface Payload {
    mimeType: string;
    body: Body;
    partId: string;
    filename: string;
    headers: Header[];
    parts?: Part[];
}

export interface Body {
    size: number;
    data?: string;
    attachmentId?: string;
}

export interface Header {
    name: string;
    value: string;
}

export interface Part {
    mimeType: string;
    body: Body;
    partId: string;
    filename: string;
    headers: Header[];
    parts?: Part[];
}
