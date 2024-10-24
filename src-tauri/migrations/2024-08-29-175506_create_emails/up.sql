CREATE TABLE "email"
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    email_id     TEXT      NOT NULL,
    delivered_at TIMESTAMP NOT NULL,
    subject      TEXT      NOT NULL
);

CREATE TABLE "sender"
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    address  TEXT NOT NULL,
    email_id INTEGER,
    FOREIGN KEY (email_id) REFERENCES email (id)
);

CREATE TABLE "recipient"
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    address  TEXT NOT NULL,
    email_id INTEGER,
    FOREIGN KEY (email_id) REFERENCES email (id)
);

CREATE TABLE "attachment"
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    filename   TEXT NOT NULL,
    content_id TEXT NOT NULL,
    content    BLOB NOT NULL,
    encoding   TEXT NOT NULL,
    email_id   INTEGER,
    FOREIGN KEY (email_id) REFERENCES email (id)
);

CREATE TABLE "body"
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    content  TEXT    NOT NULL,
    is_html  BOOLEAN NOT NULL,
    email_id INTEGER,
    FOREIGN KEY (email_id) REFERENCES email (id)
);
