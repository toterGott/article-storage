use teloxide::prelude::*;

mod dialog;
mod parser;
mod db_manager;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    db_manager::init_schema();

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        match message.update.text().map(ToOwned::to_owned) {
            None => {
                message.answer("Send me a text message.").await?;
            }
            Some(text) => {
                log::info!("User's message: {}", text);
                dialog::handle_message(message, &text).await;
            }
        }
        respond(())
    }).await;
}
