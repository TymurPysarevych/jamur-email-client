export interface Email {
  id: string;
  deliveredAt: string;
  to: Array<string>;
  from: Array<string>;
  subject: string;
  htmlBodies: Array<string>;
  textBodies: Array<string>;
  attachments: Array<EmailAttachment>;
}

export interface EmailAttachment {
  filename: string;
  contentId: string;
  content: Array<number>;
  encoding: string;
}
