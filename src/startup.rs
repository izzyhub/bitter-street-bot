use crate::configuration::read_configuration;
use crate::feed;
use crate::status::format_item;
use std::path::Path;
use elefren::Mastodon;
use elefren::MastodonClient;
use elefren::status_builder::NewStatus;
use std::io::{BufWriter, BufReader};
use std::fs::File;
use rss::Channel;
/*
use elefren::MastodonClient;
*/
use thiserror::Error;
use anyhow::Result;

#[derive(Error, Debug)]
enum StartupError {
    #[error("Failed to download feed: {0}")]
    NetworkError(reqwest::Error),
    #[error("Failed to parse feed: {0}")]
    ParseError(rss::Error),
}

pub async fn run(configuration_path: &Path) -> Result<()> {
    let feed_config = read_configuration(configuration_path)?;
    let mastodon = Mastodon::from(feed_config.authentication);

    for feed_config in feed_config.feeds {
        let feed_url = reqwest::Url::parse(&feed_config.url)?;

        let content = reqwest::get(feed_url).await.map_err(StartupError::NetworkError)?;
        let content = content.bytes().await?;
        let new_feed = Channel::read_from(&content[..]).map_err(StartupError::ParseError)?;

        let cache_path = Path::new(&feed_config.cache_file);
        println!("cache_path: {}", feed_config.cache_file);

        let items;
        if cache_path.exists() {
            println!("Reading cached file");
            let cached_file = File::open(cache_path)?;
            let reader = BufReader::new(cached_file);
            let cached_feed = Channel::read_from(reader).map_err(StartupError::ParseError)?;
            items = feed::uncached_items(&new_feed, &cached_feed)?;
        }
        else {
            items = new_feed.items.iter().take(feed_config.post_n_items_when_new).collect()
        }
        println!("Found {} items for {}", items.len(), feed_config.name);

        //let items = feed::find_new_items(feed_url, cache_path, feed_config.post_n_items_when_new).await?;

        let new_statuses: Result<Vec<NewStatus>> = items.iter().map(|item| format_item(item, &feed_config.fields_in_status, feed_config.language, feed_config.visibility)).collect();
        let new_statuses = new_statuses?;
        for status in new_statuses {
            mastodon.new_status(status).expect("failed to create new status");
        }

        println!("Writing cached file");
        let cached_file = File::create(cache_path)?;
        println!("Opened file at: {}", feed_config.cache_file);
        let cached_writer = BufWriter::new(cached_file);
        println!("Created writer");
        new_feed.write_to(cached_writer)?;
    }

    Ok(())
}
