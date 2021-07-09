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
    if id == -1 { log::error!("Article id can't be fetched")}
    return id
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
