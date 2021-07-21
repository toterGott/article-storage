alter table article_link
    add content text;
alter table article_link
    add minutes_to_read integer;

pragma user_version = 2;
