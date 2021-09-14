CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE records (
    id SERIAL PRIMARY KEY,
    chat_id UUID NOT NULL,
        CONSTRAINT fk_chat
            FOREIGN KEY(chat_id) 
	            REFERENCES chats(id),
    session_id VARCHAR NOT NULL,
        CONSTRAINT fk_session
            FOREIGN KEY(session_id) 
	            REFERENCES sessions(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);