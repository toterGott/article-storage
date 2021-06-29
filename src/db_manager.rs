use sqlite;

pub async fn save_link(message: &str, user_id: i64) {
    save_user(user_id).await;
    let connection = sqlite::open("articles-storage").unwrap();
    connection
        .execute(
            format!
            ("INSERT INTO article_link (link) VALUES ('{}');", message),
        ).unwrap();
}

async fn save_user(user_id: i64) {
    let connection = sqlite::open("articles-storage").unwrap();
    connection
        .execute(
            format!
            ("INSERT INTO bot_user (user_id) VALUES ('{}') on conflict ignore;", user_id),
        ).unwrap();
}