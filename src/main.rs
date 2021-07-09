mod dialog;
mod parser;
mod db_manager;

use teloxide::prelude::*;
use std::fs;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();
    let schema = fs::read_to_string("schema.sql")
        .expect("Something went wrong reading the file schema.sql");

    let connection= db_manager::get_connection();
    connection.execute(schema).unwrap();

    teloxide::repl(bot, |message| async move {
        // log::info!("date: {}", message.update.date);
        // log::info!("update.chat.id: {}", message.update.chat.id);
        // log::info!("kind: {:?}", message.update.kind);

        match message.update.text().map(ToOwned::to_owned) {
            None => {
                message.answer("Send me a text message.").await?;
            }
            Some(ans) => {
                log::info!("User's message: {}", ans);
                dialog::handle_message(message, &ans).await;
            }
        }

        respond(())
    })
        .await;
}
