use teloxide::{prelude::*, types::InputFile};
use url::Url;

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
        let caption = msg.caption().unwrap_or_default();
        let image_url_str = "https://i.imgur.com/DrblMGp.jpeg";
        bot.send_photo(
            msg.chat.id,
            InputFile::url(Url::parse(image_url_str).unwrap()),
        )
        .allow_sending_without_reply(true)
        .await?;
        bot.send_message(msg.chat.id, caption).await?;

        Ok(())
    })
    .await;
}
