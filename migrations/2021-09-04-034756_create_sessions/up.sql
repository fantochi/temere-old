CREATE TABLE "sessions" (
    id VARCHAR PRIMARY KEY,
    "address" INET NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);