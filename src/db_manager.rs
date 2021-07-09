use sqlite;
use sqlite::{Connection};
use sqlite::State;

pub async fn save_link(message: &str, user_id: i64) {
    save_user(user_id).await;
    execute_query(&format!("INSERT OR IGNORE INTO article_link (link) VALUES ('{}');", message)).await;
    let connection = get_connection();

    let mut statement = connection
        .prepare(&format!("SELECT  id FROM article_link WHERE link =  '{}'", message))
        .unwrap();
    match statement.bind_by_name("id", int::class) {
        Ok(x) => x,
        Err(error) => log::info!("ERROR: {}", error),
    };
    while let State::Row = statement.next().unwrap() {
        println!("id = {}", statement.read::<String>(0).unwrap());
    }

    log::info!("Link saved");
}

async fn save_user(user_id: i64) {
    execute_query(&format!("INSERT OR IGNORE INTO bot_user (user_id) VALUES ('{}')", user_id)).await;
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
