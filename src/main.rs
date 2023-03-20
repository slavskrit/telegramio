use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    let mut _server = "localhost".to_string();

    match std::env::var("RENDER_EXTERNAL_HOSTNAME").ok() {
        Some(proxy) => _server = proxy,
        None => log::info!("No RENDER_EXTERNAL_HOSTNAME env found"),
    }

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_message(msg.chat.id, "On server $").await?;
        Ok(())
    })
    .await;
}
