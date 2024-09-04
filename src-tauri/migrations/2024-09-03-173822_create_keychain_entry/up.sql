-- Your SQL goes here
CREATE TABLE "keychain_entry"
(
    "key"  TEXT NOT NULL,
    "user" TEXT NOT NULL,
    PRIMARY KEY ("key", "user")
);