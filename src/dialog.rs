use std::fs;
use std::io::Read;

use extrablatt::Extrablatt;
use futures::StreamExt;
use teloxide::net::Download;
use teloxide::prelude::*;
use teloxide::prelude::*;
use teloxide::types::{Document, InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use teloxide::types::File as TgFile;
use tokio::fs::File;

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

async fn handle_link_silent(message: &UpdateWithCx<AutoSend<Bot>, Message>, message_str: &str) {
    let user_id = message.update.chat.id;
    db_manager::save_user(user_id).await;

    db_manager::save_link(message_str).await;
    let article_id = db_manager::get_article_id(message_str).await;

    db_manager::init_read_status(user_id, article_id).await;
    db_manager::set_unread_status(user_id, article_id).await;
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
        "/mark_oldest_as_read" => { read_oldest_command(message).await; }
        "/stats" => { get_stats(message).await; }
        _ => { message.answer("Unknown command. Please use /help").await.unwrap(); }
    };
}

async fn get_stats(message: UpdateWithCx<AutoSend<Bot>, Message>) {
    let read = db_manager::get_article_counter_where_status(message.update.chat.id, true).await;
    let unread = db_manager::get_article_counter_where_status(message.update.chat.id, false).await;
    message.answer(
        format!("Read articles: {}\nUnread articles: {}\n", read, unread)
    ).await.unwrap();
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
}

async fn read_oldest_command(message: UpdateWithCx<AutoSend<Bot>, Message>) {
    match db_manager::get_oldest_article(message.update.chat.id).await {
        Some(x) => x,
        None => {
            message.answer("You have no article to mark as read \
            in your storage\n¯\\_(ツ)_/¯ ").await.unwrap();
            return;
        }
    };

    db_manager::mark_oldest_as_read(message.update.chat.id).await;
    let unread = db_manager::get_article_counter_where_status(message.update.chat.id, false).await;
    message.answer(&format!("Oldest link has been marked as read.\n\
    There is {} unread articles left in the storage.", unread)).await.unwrap();
}

pub async fn handle_file(message: &UpdateWithCx<AutoSend<Bot>, Message>, file: &Document) {
    message.answer("Downloading file...").await.unwrap();
    log::info!("File id {}", file.file_id);
    log::info!("File size {}", file.file_size.unwrap());
    if file.file_size.unwrap() > 20 * 1024 * 1024
    {
        message.answer("Sorry, file is too large. Max size is 20MB.").await.unwrap();
    }
    let downloader_bot = Bot::from_env();
    let file_content = downloader_bot
        .get_file(&file.file_id).send().await.unwrap();
    log::info!("File path: {}", file_content.file_path);
    let mut output_file = File::create(
        &format!("./{}", file_content.file_path)).await.unwrap();
    let TgFile { file_path, .. } = downloader_bot.get_file(&file.file_id).send().await.unwrap();
    downloader_bot.download_file(&file_path, &mut output_file).await.unwrap();
    let mut file_content = match fs::read_to_string(&file_path) {
        Ok(x) => x,
        Err(e) => {
            log::info!("Read error {}", e);
            return;
        }
    };
    let split_pattern = String::from("<h1>Read Archive</h1>");
    let split = file_content.split(&split_pattern);
    let split_vec = split.collect::<Vec<&str>>();
    message.answer("Parsing links...").await.unwrap();
    let links = parser::parse_links(&split_vec[0]).await;
    log::info!("Links: {:?}", &links);
    for link in &links {
        let link_to_save = link.as_str();
        handle_link_silent(message, link.as_str()).await;
        db_manager::save_link(&link_to_save).await;
        log::info!("Saving {}", &link_to_save)
    }
    let read = db_manager::get_article_counter_where_status(message.update.chat.id, true).await;
    let unread = db_manager::get_article_counter_where_status(message.update.chat.id, false).await;
    message.answer(
        format!(
            "Links has been successfully saved.\n\
            Added {} new links.\n\
            Now you have {} unread and {} read articles.",
            links.len(), unread, read)
    ).await.unwrap();
}
