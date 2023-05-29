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
    let mut messages_with_text = Vec::new();
    for e in elem {
        let caption_text = e.text().unwrap_or("");
        let caption = caption_text.clone();
        let image_url = e.attribute("url");
        match image_url {
            Some(image_url_str) => {
                messages_with_text.push(MessageWithText {
                    caption: String::from(caption),
                    image_url: Url::parse(image_url_str).unwrap(),
                });
            }
            None => log::info!("No picture -> ignoring {caption}"),
        }
    }
    let len_success_message = messages_with_text.len();
    log::info!("Messages with images: {len_success_message}");
    Ok(messages_with_text)
    // bot.send_message(msg.chat.id, "DONE").await?;
}
