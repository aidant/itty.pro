create table if not exists user (
    id blob not null,

    email text not null unique,
    display_name text not null,
    password text not null,

    created_at integer not null,
    updated_at integer not null,

    primary key (id)
) strict;
