table! {
    chats (id) {
        id -> Int4,
        message_counter -> Int4,
        status -> Chat_status,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    drops (id) {
        id -> Int4,
        user_id -> Varchar,
        blocked_id -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    chats,
    drops,
    users,
);
