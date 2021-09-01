table! {
    blocks (id) {
        id -> Int4,
        user_id -> Varchar,
        blocked_id -> Varchar,
    }
}

table! {
    chats (id) {
        id -> Uuid,
        message_counter -> Int4,
        status -> Chat_status,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    blocks,
    chats,
    users,
);
