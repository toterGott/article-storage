use teloxide::prelude::*;
use crate::parser;
use crate::db_manager;

pub async fn handle_message(message: UpdateWithCx<AutoSend<Bot>,
    Message>, message_str: &str) {
    if parser::is_link(message_str) {
        db_manager::save_user(message.update.chat.id).await;
        db_manager::save_link(message_str).await;
        let article_id = db_manager::get_article_id(article_link).await;
        // TODO save article read status

        message.answer("Link has been successfully saved").await.unwrap();
    } else if message_str.starts_with("/") {
        message.answer("It's a command").await.unwrap();
    } else {
        let mut response_message: String = "Message received: ".to_owned();
        response_message.push_str(message_str);
        message.answer(response_message).await.unwrap();
    }
}