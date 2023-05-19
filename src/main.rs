use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
extern crate reqwest;
use reqwest::{header::USER_AGENT, Client};
use roxmltree;

#[cfg(feature = "async")]
use tokio;

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
            let c = Client::new();
            let result = c
                .get("http://www.reddit.com/.rss")
                .header(USER_AGENT, "dashboard/0.1")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let doc = roxmltree::Document::parse(&result).unwrap();
            let elem = doc
                .descendants()
                .filter(|n| n.has_tag_name("title") || n.has_tag_name("thumbnail"))
                .skip(1);
            for e in elem {
                let message = format!(
                    "{:?} : {:?}",
                    e.text().unwrap_or(""),
                    e.attribute("url").unwrap_or("")
                );
                bot.send_message(msg.chat.id, message).await?;
            }
            bot.send_message(msg.chat.id, "Done!").await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;

    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     // msg.chat.
    //     let caption = msg.caption().unwrap_or_default();
    //     let image_url_str = "https://i.imgur.com/ENXEU8r.jpeg";
    //     bot.send_photo(
    //         msg.chat.id,
    //         InputFile::url(Url::parse(image_url_str).unwrap()),
    //     )
    //     .allow_sending_without_reply(true)
    //     .await?;
    //     bot.send_message(msg.chat.id, caption).await?;

    //     Ok(())
    // })
    // .await;
}
