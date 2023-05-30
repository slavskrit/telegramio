use teloxide::{
    prelude::*,
    types::InputFile,
    types::{InputMedia, InputMediaPhoto, ParseMode},
    utils::command::BotCommands,
};
extern crate reqwest;

mod reddit;
use reddit::reddit_top_records;
mod anime;
use anime::anime;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return top reddit posts with links and descriptions")]
    Reddit,
    #[command(description = "return top reddit posts as a gallery")]
    GReddit,
    #[command(description = "NSFW anime")]
    NsfwAnime,
    #[command(description = "SFW anime")]
    SfwAnime,
}
#[cfg_attr(feature = "async", tokio::main)]
async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::GReddit => {
            let messages_with_text = reddit_top_records().await.unwrap_or_default();
            let messages_len = messages_with_text.len();
            for message in messages_with_text {
                let caption = message.caption;
                let image = InputFile::url(message.image_url);
                bot.send_photo(msg.chat.id, image)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .caption(caption)
                    .disable_notification(true)
                    .await?;
            }
            bot.send_message(msg.chat.id, format!("{messages_len} posts!"))
                .await?
        }
        Command::Reddit => {
            let messages_with_text = reddit_top_records().await.unwrap_or_default();
            let messages_len = messages_with_text.len();
            let input_medias: Vec<InputMedia> = messages_with_text
                .iter()
                .map(|message| {
                    InputMedia::Photo(InputMediaPhoto {
                        media: InputFile::url(message.image_url.clone()),
                        parse_mode: Some(ParseMode::Html),
                        caption: Some(message.caption.clone()),
                        caption_entities: None,
                        has_spoiler: false,
                    })
                })
                .collect();
            for chunk in input_medias.chunks(10) {
                bot.send_media_group(msg.chat.id, chunk.to_vec()).await?;
            }
            bot.send_message(msg.chat.id, format!("{messages_len} posts!"))
                .await?
        }
        Command::nsfw_anime => {
            let image_url = anime("https://api.waifu.pics/nsfw/waifu").await;
            bot.send_photo(msg.chat.id, InputFile::url(image_url.clone()))
                .parse_mode(teloxide::types::ParseMode::Html)
                .caption(image_url.to_string())
                .disable_notification(true)
                .await?
        }
        Command::sfw_anime => {
            let image_url = anime("https://api.waifu.pics/sfw/waifu").await;
            bot.send_photo(msg.chat.id, InputFile::url(image_url.clone()))
                .parse_mode(teloxide::types::ParseMode::Html)
                .caption(image_url.to_string())
                .disable_notification(true)
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
