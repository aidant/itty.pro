create table if not exists session (
    id text not null,

    data blob not null,

    created_at integer not null,
    updated_at integer not null,
    expires_at integer not null,

    primary key (id)
) strict;
