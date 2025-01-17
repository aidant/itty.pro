create table if not exists permission (
    id blob not null,

    name text not null unique,

    created_at integer not null,
    updated_at integer not null,

    primary key (id)
) strict;
