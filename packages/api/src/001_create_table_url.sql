create table if not exists url (
    id blob not null,

    key text not null unique,
    url text not null,

    created_at integer not null,
    updated_at integer not null,

    primary key (id)
) strict;
create index if not exists url_key on url (key);
