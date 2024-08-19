export interface Email {
    id: string,
    delivered_at: string,
    to: Array<string>,
    from: Array<string>,
    subject: string,
    body: Array<string>,
    attachments: Array<EmailAttachment>,
}

export interface EmailAttachment {
    filename: string,
    content: Array<number>,
    encoding: string,
}