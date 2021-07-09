use teloxide::prelude::*;

use crate::db_manager;
use crate::parser;

pub async fn handle_message(message: UpdateWithCx<AutoSend<Bot>,
    Message>, message_str: &str) {
    if parser::is_link(message_str) {
        let user_id = message.update.chat.id;
        db_manager::save_user(user_id).await;
        db_manager::save_link(message_str).await;
        let article_id = db_manager::get_article_id(message_str).await;
        db_manager::set_read_status(user_id, article_id).await;
        message.answer("Link has been successfully saved").await.unwrap();
    } else if message_str.starts_with("/") {
        match message_str {
            "/help" =>
                message.answer("\
            /help - show this message\n\
            /get - get oldest article\n\
            /stat - show read and unread article count (TODO)\n\
            /read_last - mark oldest article as read").await.unwrap()
            ,
            "/get" => {
                let link = match db_manager::get_oldest_article(message.update.chat.id).await {
                    Some(x) => x,
                    None => String::from("You have no article to read in your storage\n\
                    ¯\\_(ツ)_/¯ "),
                };
                message.answer(link).await.unwrap()
            }
            "/read_last" => {
                let link = match db_manager::get_oldest_article(message.update.chat.id).await {
                    Some(x) => x,
                    None => {
                        let answer = String::from("You have no article to mark as read read in your storage\n\
                    ¯\\_(ツ)_/¯ ");
                        message.answer(answer).await.unwrap();
                        return;
                    }
                };
                db_manager::mark_oldest_as_read(message.update.chat.id).await;
                message.answer(
                    &format!("Link has been marked as read: {}", &link)
                ).await.unwrap()
            }
            _ => { message.answer("Unknown command").await.unwrap() }
        };
    } else {
        let response_message: String = "Please send me article link or \
        use commands.\n\
        /help - for command list ".to_owned();
        message.answer(response_message).await.unwrap();
    }
}