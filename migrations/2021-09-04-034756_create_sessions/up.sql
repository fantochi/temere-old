CREATE TABLE "sessions" (
    -- id as fingerprint
    id VARCHAR(255) PRIMARY KEY,
    "address" INET NOT NULL,
    last_connection TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);