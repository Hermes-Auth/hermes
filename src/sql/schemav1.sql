CREATE TABLE users (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    email VARCHAR(255) NOT NULL
);

CREATE TABLE services (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    key VARCHAR(255) NOT NULL,
    code_ttl INTEGER DEFAULT 5 NOT NULL,
    created_by UUID NOT NULL
);

CREATE TABLE logs (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    service UUID NOT NULL,
    event VARCHAR(255) NOT NULL,
    data TEXT
);
