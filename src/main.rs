use std::fs;

use teloxide::net::Download;
use teloxide::prelude::*;
use teloxide::types::File as TgFile;
use tokio::fs::File;

mod dialog;
mod parser;
mod db_manager;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    db_manager::init_schema();

    let bot = Bot::from_env().auto_send();
    let changelog = fs::read_to_string("changelog_message.txt")
        .expect("Something went wrong reading the changelog");
    for sub in db_manager::get_subscribed_users().await {
        bot.send_message(sub, &changelog).send().await;
    }

    teloxide::repl(bot, |message| async move {
        // TODO refactor handling
        match message.update.document() {
            Some(file) => {
                message.answer("Downloading file...").await?;
                log::info!("File id {}", file.file_id);
                log::info!("File size {}", file.file_size.unwrap());
                let downloader_bot = Bot::from_env();
                let file_content = downloader_bot
                    .get_file(&file.file_id).send().await?;
                log::info!("File path: {}", file_content.file_path);
                let mut output_file = File::create(
                    &format!("./{}", file_content.file_path)).await.unwrap();
                let TgFile { file_path, .. } = downloader_bot.get_file(&file.file_id).send().await?;
                downloader_bot
                    .download_file(&file_path, &mut output_file).await.unwrap();
                message.answer("File has been downloaded").await?;
            }
            None => match message.update.text().map(ToOwned::to_owned) {
                None => {
                    message.answer("Send me a text message.").await?;
                }
                Some(text) => {
                    log::info!("User's message: {}", text);
                    dialog::handle_message(message, &text).await;
                }
            },
        };
        respond(())
    }).await;
}
