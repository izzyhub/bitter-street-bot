use elefren::status_builder::Visibility;
use elefren::data::Data as MastodonAuth;
use toml;
use std::fs;
use std::fmt;
use std::path::Path;
use elefren::Language;

use serde::{Serialize, Deserialize};

use anyhow::Result;

#[derive(thiserror::Error, Debug)]
enum ConfigError {
    #[error("Error parsing configuration: {0}")]
    ParseError(#[from] toml::de::Error ),
}

#[derive(Deserialize)]
pub struct Config {
    pub authentication: MastodonAuth,
    pub feeds: Vec<FeedConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusField {
    Title,
    Link,
    Description,
    Author,
    Categories,
    Content,
    Guid,
    PubDate,
}

impl fmt::Display for StatusField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            StatusField::Title => "Title",
            StatusField::Link => "Link",
            StatusField::Description => "Description",
            StatusField::Author => "Author",
            StatusField::Categories => "Categories",
            StatusField::Content => "Content",
            StatusField::Guid => "Guid",
            StatusField::PubDate => "PubDate",
        };

        write!(f, "{}", value)
    }
}

#[derive(Deserialize)]
pub struct FeedConfig {
    pub name: String,
    pub url: String,
    pub cache_file: String,
    pub post_n_items_when_new: usize,
    pub fields_in_status: Vec<StatusField>,
    pub language: Language,
    pub visibility: Visibility,
}

pub fn read_configuration(configuration_path: &Path) -> Result<Config> {
    let config = fs::read_to_string(configuration_path)?;
    toml::from_str(&config).map_err(|e| ConfigError::ParseError(e).into() )
}
