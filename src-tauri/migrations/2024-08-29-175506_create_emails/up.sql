CREATE TABLE "email"
(
    id           INTEGER PRIMARY KEY NOT NULL,
    email_id     TEXT                NOT NULL,
    delivered_at TIMESTAMP           NOT NULL,
    subject      TEXT                NOT NULL
);

CREATE TABLE "sender"
(
    id       INTEGER PRIMARY KEY NOT NULL,
    address  TEXT                NOT NULL,
    email_id INTEGER             NOT NULL,
    FOREIGN KEY (email_id) REFERENCES email (id)
);

CREATE TABLE "recipient"
(
    id       INTEGER PRIMARY KEY NOT NULL,
    address  TEXT                NOT NULL,
    email_id INTEGER             NOT NULL,
    FOREIGN KEY (email_id) REFERENCES email (id)
);

CREATE TABLE "attachment"
(
    id         INTEGER PRIMARY KEY NOT NULL,
    filename   TEXT                NOT NULL,
    content_id TEXT                NOT NULL,
    content    BLOB                NOT NULL,
    encoding   TEXT                NOT NULL,
    email_id   INTEGER             NOT NULL,
    FOREIGN KEY (email_id) REFERENCES email (id)
);
