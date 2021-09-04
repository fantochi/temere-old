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
        lobby_id -> Uuid,
        message_counter -> Int4,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    history (id) {
        id -> Int4,
        chat_id -> Uuid,
        session_id -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    lobbys (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        nsfw -> Bool,
        enabled -> Bool,
    }
}

table! {
    sessions (id) {
        id -> Varchar,
        address -> Inet,
        last_connection -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Varchar,
        created_at -> Timestamptz,
    }
}

joinable!(chats -> lobbys (lobby_id));
joinable!(history -> chats (chat_id));
joinable!(history -> sessions (session_id));

allow_tables_to_appear_in_same_query!(
    blocks,
    chats,
    history,
    lobbys,
    sessions,
    users,
);
