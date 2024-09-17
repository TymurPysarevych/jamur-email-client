-- Your SQL goes here
CREATE TABLE simple_mail_credentials
(
    id           INTEGER PRIMARY KEY,
    username     TEXT    NOT NULL,
    keychain_id TEXT    NOT NULL,
    imap_host    TEXT    NOT NULL,
    smtp_host    TEXT    NOT NULL,
    imap_port    INTEGER NOT NULL,
    smtp_port    INTEGER NOT NULL
)
