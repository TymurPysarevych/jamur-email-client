// @generated automatically by Diesel CLI.

diesel::table! {
    access_token (keychain_user) {
        token -> Text,
        keychain_user -> Text,
    }
}

diesel::table! {
    attachment (id) {
        id -> Integer,
        filename -> Text,
        content_id -> Text,
        content -> Binary,
        encoding -> Text,
        email_id -> Integer,
    }
}

diesel::table! {
    email (id) {
        id -> Integer,
        email_id -> Text,
        delivered_at -> Timestamp,
        subject -> Text,
    }
}

diesel::table! {
    keychain_entry (user) {
        key -> Text,
        user -> Text,
    }
}

diesel::table! {
    recipient (id) {
        id -> Integer,
        address -> Text,
        email_id -> Integer,
    }
}

diesel::table! {
    sender (id) {
        id -> Integer,
        address -> Text,
        email_id -> Integer,
    }
}

diesel::table! {
    simple_mail_credentials (id) {
        id -> Nullable<Integer>,
        username -> Text,
        keychain_key -> Text,
        imap_host -> Text,
        smtp_host -> Text,
        imap_port -> Integer,
        smtp_port -> Integer,
    }
}

diesel::joinable!(attachment -> email (email_id));
diesel::joinable!(recipient -> email (email_id));
diesel::joinable!(sender -> email (email_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_token,
    attachment,
    email,
    keychain_entry,
    recipient,
    sender,
    simple_mail_credentials,
);
