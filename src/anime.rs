use std::collections::HashMap;

use reqwest::{header::USER_AGENT, Client};
use url::Url;

pub async fn anime(url: &str) -> Url {
    let c = Client::new();
    let a = c.get(url).header(USER_AGENT, "dashboard/0.1").send().await;
    match a {
        Ok(aa) => {
            let b = aa.json::<HashMap<String, String>>().await;
            let waifu_url = b.unwrap().get("url").unwrap().to_owned();
            let c = waifu_url.as_str();
            Url::parse(c).unwrap()
        }
        Err(_) => todo!(),
    }
}
