create table url_analytics (
    id blob not null,
    url_id blob not null,

    req_client_ip text,
    req_user_agent text,

    created_at integer not null,
    updated_at integer not null,

    primary key (id),
    foreign key (url_id) references url(id) on delete cascade on update cascade
) strict
