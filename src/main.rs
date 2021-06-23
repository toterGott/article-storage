use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        log::info!("id: {}", message.update.id);
        log::info!("date: {}", message.update.date);
        log::info!("update.chat.id: {}", message.update.chat.id);

        message.answer_dice().await?;

        match message.update.text().map(ToOwned::to_owned) {
            None => {
                message.answer("Send me a text message.").await?;
            }
            Some(ans) => {
                log::info!("User's message: {}", ans);
                let mut response_message: String = "Message received: ".to_owned();
                response_message.push_str(&ans);
                message.answer(response_message).await?;
            }
        }

        respond(())
    })
        .await;
}
