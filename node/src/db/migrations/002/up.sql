create table users(
    id text default gen_random_uuid()::text primary key,
    email text not null unique,
    api_key text not null unique
)

create table apps(
    id uuid default gen_random_uuid() primary key,
    name text not null,
    owner text not null,
    active boolean not null
)
