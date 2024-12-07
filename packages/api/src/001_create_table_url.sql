create table url (
    id blob not null,

    key text not null,
    url text not null,

    created_at integer not null,
    updated_at integer not null,

    primary key (id)
) strict;
create index url_key on url (key);
