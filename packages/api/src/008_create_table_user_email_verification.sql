create table if not exists user_email_verification (
    id blob not null,
    user_id blob not null,

    token blob not null unique,

    created_at integer not null,
    updated_at integer not null,

    primary key (id),
    foreign key (user_id) references user(id) on delete cascade on update cascade
) strict;
