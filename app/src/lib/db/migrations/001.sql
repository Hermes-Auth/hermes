create table users (
    id text default gen_random_uuid()::text,
    email text not null,
    api_key text default gen_random_uuid()::text
);

create table apps(
    id text default gen_random_uuid()::text,
    name text not null,
    owner text not null,
    default_ttl text not null default 5
);

create table codes (
    id uuid default gen_random_uuid(),
    code text not null,
    ttl text not null,
    used boolean default false
);
