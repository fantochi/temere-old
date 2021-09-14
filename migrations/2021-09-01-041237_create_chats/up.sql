CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE chat_status AS ENUM ('open', 'closed');

CREATE TABLE chats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    lobby_id UUID NOT NULL,
        CONSTRAINT fk_lobby
            FOREIGN KEY(lobby_id) 
	            REFERENCES lobbys(id),
    message_counter INT NOT NULL DEFAULT 0 CHECK (message_counter >= 0),
    status VARCHAR NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);