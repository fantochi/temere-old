CREATE TYPE chat_status AS ENUM ('open', 'closed', 'waiting');

CREATE TABLE chats (
    id SERIAL PRIMARY KEY,
    message_counter INT NOT NULL DEFAULT 0 CHECK (message_counter > 0),
    status chat_status NOT NULL DEFAULT 'waiting',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);