use std::io::Read;

use soup::Soup;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

use crate::db_manager;
use crate::parser;

pub async fn handle_message(message: UpdateWithCx<AutoSend<Bot>, Message>, message_str: &str) {
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
        "/switch_changelog_notification" => {
            switch_changelog_notification(message).await;
        }
        "/get" => { get_command(message).await; }
        "/mark_last_as_read" => { read_last_command(message).await; }
        _ => { message.answer("Unknown command. Please use /help").await.unwrap(); }
    };
}

async fn switch_changelog_notification(message: UpdateWithCx<AutoSend<Bot>, Message>) {
    db_manager::switch_changelog_notification(message.update.chat.id).await;
    let status = db_manager::get_subscription_status(message.update.chat.id).await;
    if status {
        message.answer("Now you subscribed to update notifications").await.unwrap();
    } else {
        message.answer("You has been unsubscribed from update notifications").await.unwrap();
    }
}

async fn get_command(message: UpdateWithCx<AutoSend<Bot>, Message>) {
    let link = match db_manager::get_oldest_article(message.update.chat.id).await {
        Some(x) => x,
        None => String::from("You have no article to read in your storage\n¯\\_(ツ)_/¯ "),
    };

    message.answer(&link).send().await;
    log::info!("Downloading...");
    let body = reqwest::get(&link)
        .await.unwrap()
        .text()
        .await.unwrap();
    let soup = Soup::new(&body);
    let text = soup.text();
    log::info!("Body: {}", text);
}

async fn read_last_command(message: UpdateWithCx<AutoSend<Bot>, Message>) {
    match db_manager::get_oldest_article(message.update.chat.id).await {
        Some(x) => x,
        None => {
            message.answer("You have no article to mark as read \
            in your storage\n¯\\_(ツ)_/¯ ").await.unwrap();
            return;
        }
    };

    db_manager::mark_oldest_as_read(message.update.chat.id).await;
    message.answer(&format!("Last link has been marked as read")).await.unwrap();
}
