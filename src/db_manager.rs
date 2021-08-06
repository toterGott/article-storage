use std::fs;

use sqlite;
use sqlite::{Connection, State};

pub async fn save_link(article_link: &str) {
    execute_query(
        &format!(
            "INSERT OR IGNORE INTO article_link (link) VALUES ('{}');",
            article_link)
    ).await;
}

pub async fn get_article_id(article_link: &str) -> i64 {
    let connection = get_connection();
    let mut statement = connection
        .prepare(
            &format!(
                "SELECT id FROM article_link WHERE link =  '{}'",
                article_link)
        ).unwrap();

    // TODO refactor this mess below
    let mut id: i64 = -1;
    while let State::Row = statement.next().unwrap() {
        id = statement.read::<i64>(0).unwrap();
    }
    if id == -1 { log::error!("Article id can't be fetched") }
    return id;
}

pub async fn get_subscription_status(user_id: i64) -> bool {
    let connection = get_connection();
    let mut statement = connection
        .prepare(
            &format!(
                "SELECT update_subscription FROM bot_user WHERE user_id = {};",
                user_id)
        ).unwrap();

    let mut status: bool = false;
    while let State::Row = statement.next().unwrap() {
        let res = statement.read::<i64>(0).unwrap();

        if res == 1 {
            status = true
        } else {
            status = false
        }
    }
    return status;
}

pub async fn save_user(user_id: i64) {
    execute_query(
        &format!(
            "INSERT OR IGNORE INTO bot_user (user_id) VALUES ('{}')",
            user_id)
    ).await;
}

pub fn get_connection() -> Connection {
    match sqlite::open("article-storage.db") {
        Ok(connection) => { return connection; }
        Err(error) => {
            log::error!("Unable to open SQLite connection: {}", error);
            panic!();
        }
    };
}

async fn execute_query(query: &str) {
    match get_connection().execute(query) {
        Ok(_) => { return; }
        Err(sql_error) => {
            log::info!("SQL error: {}", sql_error);
        }
    };
}

pub async fn init_read_status(user_id: i64, article_id: i64) {
    execute_query(
        &format!(
            "INSERT INTO read_status (user_id, link_id) VALUES ('{}', '{}')",
            user_id,
            article_id)
    ).await;
}

pub async fn set_unread_status(user_id: i64, article_id: i64) {
    execute_query(
        &format!(
            "UPDATE read_status SET status = false, updated_timestamp = current_timestamp where user_id = {} and link_id = {}",
            user_id,
            article_id)
    ).await;
}

pub async fn get_oldest_article(user_id: i64) -> Option<String> {
    let connection = get_connection();
    let mut statement = connection
        .prepare(
            &format!(
                "select al.link
                from read_status
                    join article_link al on read_status.link_id = al.id
                where user_id = {} and status = false
                order by added_timestamp
                limit 1;",
                user_id)
        ).unwrap();

    let mut link: String = String::new();
    while let State::Row = statement.next().unwrap() {
        link = statement.read::<String>(0).unwrap();
    }
    if link == "" {
        log::error!("Article can't be fetched");
        return Option::None;
    }
    return Option::Some(link);
}

pub async fn mark_oldest_as_read(user_id: i64) {
    execute_query(
        &format!(
            "update read_status
            set status = true,
            updated_timestamp = current_timestamp
            where user_id = {}
            and link_id = (select link_id from read_status where user_id = {} and status = false limit 1);",
            user_id, user_id)
    ).await;
}

pub fn init_schema() {
    let mut db_version = get_db_version();
    log::info!("db version: {}", db_version);
    let connection = get_connection();
    loop {
        db_version += 1;
        let path = &format!("db_migration/migration_{}.sql", db_version);
        let migration_sql = match fs::read_to_string(path) {
            Ok(x) => x,
            Err(e) => {
                log::info!("Schema is up to date");
                break;
            }
        };
        log::info!("executing script: {}", migration_sql);
        connection.execute(migration_sql).unwrap();
    }
}

pub async fn switch_changelog_notification(user_id: i64) {
    execute_query(
        &format!(
            "update bot_user \
            set update_subscription = not update_subscription \
            where user_id = {};",
            user_id)
    ).await;
}

pub async fn get_subscribed_users() -> Vec<i64> {
    let connection = get_connection();
    let mut statement = connection
        .prepare(
            &format!(
                "SELECT user_id \
                FROM bot_user \
                WHERE update_subscription = true")
        ).unwrap();

    let mut res: Vec<i64> = vec![];
    while let State::Row = statement.next().unwrap() {
        res.push(statement.read::<i64>(0).unwrap());
    }
    return res;
}

pub fn get_db_version() -> i64 {
    let connection = get_connection();
    let mut statement = connection
        .prepare("pragma user_version").unwrap();

    let mut id: i64 = -1;
    while let State::Row = statement.next().unwrap() {
        id = statement.read::<i64>(0).unwrap();
    }
    if id == -1 { log::error!("Version can't be fetch") }
    return id;
}

pub async fn get_article_counter_where_status(user_id: i64, status: bool) -> i64 {
    let connection = get_connection();
    let mut statement = connection
        .prepare(
            &format!(
                "select count() \
                from read_status \
                where user_id = {} and status = {};",
                user_id, status)
        ).unwrap();

    let mut id: i64 = -1;
    while let State::Row = statement.next().unwrap() {
        id = statement.read::<i64>(0).unwrap();
    }
    if id == -1 { log::error!("Can't count articles") }
    return id;
}