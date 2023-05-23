create table users(
    id text default gen_random_uuid()::text primary key,
    email text not null unique
)

create table api_keys(
    id uuid  not null default gen_random_uuid() primary key,
    key text not null unique,
    owner text not null,
    active boolean not null
)

create table apps(
    id uuid default gen_random_uuid() primary key,
    name text not null,
    owner text not null,
    active boolean not null
)
