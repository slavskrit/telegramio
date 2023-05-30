extern crate reqwest;

use reqwest::{header::USER_AGENT, Client};
use roxmltree;
use url::Url;

#[derive(Debug)]
pub struct MessageWithText {
    pub caption: String,
    pub image_url: Url,
}

pub async fn reddit_top_records() -> Result<Vec<MessageWithText>, Vec<bool>> {
    let c = Client::new();
    let reddit_xml = c
        .get("http://www.reddit.com/.rss")
        .header(USER_AGENT, "dashboard/0.1")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let doc = roxmltree::Document::parse(&reddit_xml).unwrap();
    let entries = doc
        .descendants()
        .filter(|n| n.tag_name().name() == "entry")
        .skip(1);
    let mut messages_with_text = Vec::new();
    for e in entries {
        let mut thumbnail = "";
        let mut link = "";
        let mut title = "";
        for k in e.children() {
            match k.tag_name().name() {
                "title" => title = k.text().unwrap_or(""),
                "link" => link = k.attribute("href").unwrap_or_default(),
                "thumbnail" => thumbnail = k.attribute("url").unwrap_or_default(),
                _ => log::warn!("something is wrong!"),
            }
        }
        if !thumbnail.is_empty() {
            match Url::parse(thumbnail) {
                Ok(url) => messages_with_text.push(MessageWithText {
                    caption: String::from(format!("<a href='{link}'>{title}</a>")),
                    image_url: url,
                }),
                Err(_) => log::warn!("Url {thumbnail} is broken... proceed "),
            }
        }
    }
    let len_success_message = messages_with_text.len();
    log::info!("Messages with images: {len_success_message}");
    Ok(messages_with_text)
    // bot.send_message(msg.chat.id, "DONE").await?;
}
