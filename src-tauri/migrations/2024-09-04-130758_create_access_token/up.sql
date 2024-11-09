-- Your SQL goes here
CREATE TABLE access_token
(
    token         TEXT             NOT NULL,
    keychain_user TEXT PRIMARY KEY NOT NULL
);
