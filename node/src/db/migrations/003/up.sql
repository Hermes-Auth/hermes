create table apps(
    id uuid default gen_random_uuid() primary key,
    name text not null,
    owner text not null,
    active boolean not null,
    default_codes_ttl text default "5" not null
)

create table codes(
    code text not null,
    ttl text not null,
    generated_at text not null,
    generated_for text not null,
    app text not null,
    used boolean not null
)
