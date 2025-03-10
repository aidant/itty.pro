create table if not exists user (
    id blob not null,

    display_name text not null,
    email text not null unique,
    email_verified integer not null,
    password text not null,

    created_at integer not null,
    updated_at integer not null,

    primary key (id)
) strict;
