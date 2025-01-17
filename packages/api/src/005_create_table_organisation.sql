create table if not exists organisation (
    id blob not null,

    display_name text not null,

    created_at integer not null,
    updated_at integer not null,

    primary key (id)
) strict;
