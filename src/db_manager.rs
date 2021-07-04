use sqlite;
use sqlite::{Connection};

pub async fn save_link(message: &str, user_id: i64) {
    save_user(user_id).await;
    execute_query(&format!("INSERT INTO article_link (link) VALUES ('{}');", message)).await;
    log::info!("Link saved");
}

async fn save_user(user_id: i64) {
    execute_query(&format!("INSERT INTO bot_user (user_id) VALUES ('{}')", user_id)).await;
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
