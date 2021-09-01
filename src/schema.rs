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
    lobbys (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        nsfw -> Bool,
        enabled -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
        created_at -> Timestamptz,
    }
}

joinable!(chats -> lobbys (lobby_id));

allow_tables_to_appear_in_same_query!(
    blocks,
    chats,
    lobbys,
    users,
);
