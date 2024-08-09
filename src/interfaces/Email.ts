export interface Email {
    id: string,
    delivered_at: string,
    to: Array<string>,
    from: Array<string>,
    subject: string,
    body: Array<string>,
}
