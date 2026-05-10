use std::error::Error;
use rss::Channel;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let chnl: Channel = rss().await?;
    for channel in chnl.items() {
        println!("{}", channel.title().unwrap_or("untitled"));
    }
    Ok(())
}


async fn rss() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://rateyourmusic.com/~wnguyen7/data/rss")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
