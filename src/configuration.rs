
use elefren::data::Data as MastodonAuth;
use toml;
use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub authentication: MastodonAuth,
    pub feeds: Vec<FeedConfig>,
}

#[derive(Deserialize)]
pub struct FeedConfig {
    url: String,
    cache_file: String,
    post_n_items_when_new: i32,
}

pub fn read_configuration(configuration_path: Path) -> Result<FeedConfig> {
    let config = fs::read_to_string(configuration_path)?;
    toml::from_str(&config)
}
