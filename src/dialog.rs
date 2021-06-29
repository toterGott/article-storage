use teloxide::prelude::*;
use crate::parser;
use crate::db_manager;

pub async fn handle_message(message: UpdateWithCx<AutoSend<Bot>,
    Message>, message_str: &str) {
    if parser::is_link(message_str) {
        db_manager::save_link(message_str, message.update.chat.id).await;
        message.answer("It is a link").await.unwrap();
        return
    }

    let mut response_message: String = "Message received: ".to_owned();
    response_message.push_str(message_str);
    message.answer(response_message).await.unwrap();
}