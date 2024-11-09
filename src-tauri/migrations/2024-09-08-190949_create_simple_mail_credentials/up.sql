-- Your SQL goes here
CREATE TABLE simple_mail_credentials
(
    keychain_id TEXT PRIMARY KEY NOT NULL,
    username    TEXT             NOT NULL,
    imap_host   TEXT             NOT NULL,
    smtp_host   TEXT             NOT NULL,
    imap_port   INTEGER          NOT NULL,
    smtp_port   INTEGER          NOT NULL
)
