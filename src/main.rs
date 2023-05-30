use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
extern crate reqwest;

mod reddit;
use reddit::reddit_top_records;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return top reddit posts.")]
    Reddit,
}
#[cfg_attr(feature = "async", tokio::main)]
async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Reddit => {
            let messages_with_text = reddit_top_records().await.unwrap_or_default();
            let messages_len = messages_with_text.len();
            dbg!(&messages_with_text);
            for message in messages_with_text {
                let caption = message.caption;
                let image = InputFile::url(message.image_url);
                log::warn!("Caption... {caption}");
                bot.send_photo(msg.chat.id, image)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .caption(caption)
                    .disable_notification(true)
                    .await?;
            }
            bot.send_message(msg.chat.id, format!("Done with {messages_len} messages."))
                .await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting telegramio bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}
