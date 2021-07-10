use teloxide::prelude::*;

use crate::db_manager;
use crate::parser;

pub async fn handle_message(message: UpdateWithCx<AutoSend<Bot>,
    Message>, message_str: &str) {
    if parser::is_link(message_str) {
        handle_link(message, message_str).await
    } else if message_str.starts_with("/") {
        handle_command(message, message_str).await
    } else {
        message.answer("Please send me article link or \
        use commands.\n\
        /help - for command list ").await.unwrap();
    }
}

async fn handle_link(message: UpdateWithCx<AutoSend<Bot>, Message>, message_str: &str) {
    let user_id = message.update.chat.id;
    db_manager::save_user(user_id).await;

    db_manager::save_link(message_str).await;
    let article_id = db_manager::get_article_id(message_str).await;

    db_manager::init_read_status(user_id, article_id).await;
    db_manager::set_unread_status(user_id, article_id).await;

    message.answer("Link has been successfully saved").await.unwrap();
}

async fn handle_command(message: UpdateWithCx<AutoSend<Bot>, Message>, message_str: &str) {
    match message_str {
        "/help" => { help_command(message).await; }
        "/get" => { get_command(message).await; }
        "/read_last" => { read_last_command(message).await; }
        _ => { message.answer("Unknown command. Please use /help").await.unwrap(); }
    };
}

async fn help_command(message: UpdateWithCx<AutoSend<Bot>, Message>) {
    message.answer("\
            /help - show this message\n\
            /get - get oldest article\n\
            /read_last - mark oldest article as read").await.unwrap();
}

async fn get_command(message: UpdateWithCx<AutoSend<Bot>, Message>) {
    let link = match db_manager::get_oldest_article(message.update.chat.id).await {
        Some(x) => x,
        None => String::from("You have no article to read in your storage\n¯\\_(ツ)_/¯ "),
    };

    message.answer(link).await.unwrap();
}

async fn read_last_command(message: UpdateWithCx<AutoSend<Bot>, Message>) {
    let link = match db_manager::get_oldest_article(message.update.chat.id).await {
        Some(x) => x,
        None => {
            message.answer("You have no article to mark as read \
            in your storage\n¯\\_(ツ)_/¯ ").await.unwrap();
            return
        }
    };

    db_manager::mark_oldest_as_read(message.update.chat.id).await;
    message.answer(&format!("Link has been marked as read: {}", &link)).await.unwrap();
}
