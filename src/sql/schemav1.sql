CREATE TABLE users (
    id TEXT DEFAULT gen_random_uuid()::text PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    api_key TEXT DEFAULT gen_random_uuid()::text NOT NULL
);

CREATE TABLE services (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    key VARCHAR(255) NOT NULL,
    code_ttl INTEGER DEFAULT 5 NOT NULL,
    created_by TEXT NOT NULL
);

CREATE TABLE logs (
    id TEXT DEFAULT gen_random_uuid()::text PRIMARY KEY,
    service TEXT NOT NULL,
    event VARCHAR(255) NOT NULL,
    data TEXT
);
