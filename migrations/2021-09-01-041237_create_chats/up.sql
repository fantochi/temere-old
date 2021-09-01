CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE chats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    lobby_id UUID NOT NULL,
        CONSTRAINT fk_lobby
            FOREIGN KEY(lobby_id) 
	            REFERENCES lobbys(id),
    message_counter INT NOT NULL DEFAULT 0 CHECK (message_counter > 0),
    status VARCHAR(255) NOT NULL DEFAULT 'waiting',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);