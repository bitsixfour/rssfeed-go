use std::error::Error;
use rss::Channel;
use reqwest::Client;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let chnl: Channel = rss().await?;
    // for channel in chnl.items() {
       // println!("{}", channel.title().unwrap_or("untitled"));
       // }
    rss().await?;
    Ok(())
}


async fn rss() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
        .build()?;

    let content = client
        .get("https://rateyourmusic.com/~wnguyen7/data/rss")
        .send()
        .await?
        .text()
        .await?;

    // let channel = Channel::read_from(&content[..])?;
    println!("{}", content);
    Ok(())
}
