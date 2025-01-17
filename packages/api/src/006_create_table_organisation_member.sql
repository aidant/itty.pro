create table if not exists organisation_access (
    id blob not null,

    organisation_id blob not null,
    permission_id blob not null,
    user_id blob not null,

    created_at integer not null,
    updated_at integer not null,

    primary key (id),
    foreign key (organisation_id) references organisation(id) on delete cascade on update cascade,
    foreign key (permission_id) references permission(id) on delete cascade on update cascade,
    foreign key (user_id) references user(id) on delete cascade on update cascade
) strict;
