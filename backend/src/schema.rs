// @generated automatically by Diesel CLI.

diesel::table! {
    chat_room_invites (id) {
        id -> Int4,
        user_id -> Int4,
        chat_room_id -> Int4,
        recipient_email -> Varchar,
        accepted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    chat_rooms (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        chat_room_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int4,
        user_id -> Int4,
        message_id -> Int4,
        is_read -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_chat_rooms (user_id, chat_room_id) {
        user_id -> Int4,
        chat_room_id -> Int4,
    }
}

diesel::table! {
    user_status (user_id) {
        user_id -> Int4,
        is_online -> Bool,
        last_seen_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(chat_room_invites -> chat_rooms (chat_room_id));
diesel::joinable!(chat_room_invites -> users (user_id));
diesel::joinable!(chat_rooms -> users (created_by));
diesel::joinable!(messages -> chat_rooms (chat_room_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(notifications -> messages (message_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(user_chat_rooms -> chat_rooms (chat_room_id));
diesel::joinable!(user_chat_rooms -> users (user_id));
diesel::joinable!(user_status -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat_room_invites,
    chat_rooms,
    messages,
    notifications,
    user_chat_rooms,
    user_status,
    users,
);
