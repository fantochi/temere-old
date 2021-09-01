CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE chat_status AS ENUM ('open', 'closed', 'waiting');

CREATE TABLE chats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    message_counter INT NOT NULL DEFAULT 0 CHECK (message_counter > 0),
    status chat_status NOT NULL DEFAULT 'waiting',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);