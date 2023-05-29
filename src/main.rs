use teloxide::{
    prelude::*,
    types::{InputFile, MessageEntity, MessageEntityKind},
    utils::command::BotCommands,
};
extern crate reqwest;

mod reddit;
use reddit::reddit_top_records;
use url::Url;

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
            let messages_with_text_result = reddit_top_records().await;
            match messages_with_text_result {
                Ok(messages_with_text) => {
                    for message in messages_with_text {
                        let caption = message.caption;
                        let image_str = message.image_url.as_str();
                        let image = InputFile::url(Url::parse(message.image_url.as_str()).unwrap());
                        bot.send_photo(msg.chat.id, image)
                            .parse_mode(teloxide::types::ParseMode::Html)
                            .caption(format!("\n{caption}\n{caption}\n3"))
                            .caption_entities(vec![MessageEntity {
                                kind: MessageEntityKind::TextLink {
                                    url: reqwest::Url::parse("https://example.com").unwrap(),
                                },
                                offset: 1,
                                length: 2,
                            }])
                            .disable_notification(true)
                            .await?;
                        break;
                    }
                }
                Err(_) => log::info!("Error lol..."),
            }
            // for message in a {
            //     dbg!(message);
            //     // Some(image_url_str) => {
            //     //     bot.send_photo(
            //     //         msg.chat.id,
            //     //         InputFile::url(Url::parse(image_url_str).unwrap()),
            //     //     )
            //     //     .parse_mode(teloxide::types::ParseMode::Html)
            //     //     .caption(format!("{caption}{image_url_str}"))
            //     //     .disable_notification(true)
            //     //     .await?;
            // }
            bot.send_message(msg.chat.id, "Done!").await?
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
