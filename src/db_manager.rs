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

pub async fn save_user(user_id: i64) {
    execute_query(
        &format!(
            "INSERT OR IGNORE INTO bot_user (user_id) VALUES ('{}')",
            user_id)
    ).await;
}

pub fn get_connection() -> Connection {
    match sqlite::open("article-storage") {
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

pub async fn create_read_status(user_id: i64, article_id: i64) {
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

    // TODO this language makes hate myself
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
                set status = true, updated_timestamp = current_timestamp
                where added_timestamp = (select added_timestamp
                from read_status
                where user_id = {} and status = false
                order by added_timestamp
                limit 1);",
            user_id)
    ).await;
}