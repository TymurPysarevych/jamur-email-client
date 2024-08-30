// @generated automatically by Diesel CLI.

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

diesel::joinable!(attachment -> email (email_id));
diesel::joinable!(recipient -> email (email_id));
diesel::joinable!(sender -> email (email_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachment,
    email,
    recipient,
    sender,
);
