PRAGMA foreign_keys = ON;

create table if not exists bot_user
(
    user_id           bigint not null
        constraint user_pk
            primary key,
    notification_time bigint
);

create unique index if not exists user_user_id_uindex
    on bot_user (user_id);

create table if not exists article_link
(
    id   integer not null
        constraint article_link_pk
            primary key autoincrement,
    link text    not null
);

create unique index if not exists article_link_id_uindex
    on article_link (id);

create unique index if not exists article_link_link_uindex
    on article_link (link);

create table if not exists read_status
(
    user_id           bigint
        references bot_user
            on delete cascade,
    link_id           bigint
        references article_link
            on delete cascade,
    status            boolean   default false not null,
    added_timestamp   timestamp default current_timestamp not null,
    updated_timestamp timestamp default current_timestamp not null,
    unique (user_id, link_id)
);
