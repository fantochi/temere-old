CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE lobbys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "name" VARCHAR NOT NULL,
    "description" TEXT NOT NULL,
    "nsfw" BOOLEAN NOT NULL DEFAULT false,
    "enabled" BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);