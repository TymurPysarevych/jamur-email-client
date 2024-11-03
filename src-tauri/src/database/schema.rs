// @generated automatically by Diesel CLI.

diesel::table! {
    access_token (keychain_user) {
        token -> Text,
        keychain_user -> Text,
    }
}

diesel::table! {
    attachment (id) {
        id -> Nullable<Integer>,
        filename -> Text,
        content_id -> Text,
        content -> Binary,
        encoding -> Text,
        email_id -> Nullable<Integer>,
    }
}

diesel::table! {
    body (id) {
        id -> Nullable<Integer>,
        content -> Text,
        is_html -> Bool,
        email_id -> Nullable<Integer>,
    }
}

diesel::table! {
    email (id) {
        id -> Nullable<Integer>,
        email_id -> Text,
        delivered_at -> Timestamp,
        subject -> Text,
        folder_path -> Text,
    }
}

diesel::table! {
    keychain_entry (id) {
        id -> Text,
        key -> Text,
    }
}

diesel::table! {
    recipient (id) {
        id -> Nullable<Integer>,
        address -> Text,
        email_id -> Nullable<Integer>,
    }
}

diesel::table! {
    sender (id) {
        id -> Nullable<Integer>,
        address -> Text,
        email_id -> Nullable<Integer>,
    }
}

diesel::table! {
    simple_mail_credentials (keychain_id) {
        keychain_id -> Text,
        username -> Text,
        imap_host -> Text,
        smtp_host -> Text,
        imap_port -> Integer,
        smtp_port -> Integer,
    }
}

diesel::joinable!(attachment -> email (email_id));
diesel::joinable!(body -> email (email_id));
diesel::joinable!(recipient -> email (email_id));
diesel::joinable!(sender -> email (email_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_token,
    attachment,
    body,
    email,
    keychain_entry,
    recipient,
    sender,
    simple_mail_credentials,
);
