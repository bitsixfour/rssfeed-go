use std::error::Error;
use rss::Channel;



#[tokio::main]

async fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = reqwest::get("https://rateyourmusic.com/~wnguyen7/data/rss")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
