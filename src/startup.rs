use configuration::read_configuration;
use std::path::Path;
/*
use elefren::status_builder::{NewStatus, StatusBuilder, Visibility};
use elefren::MastodonClient;
use elefren::Mastodon;
use elefren::Language;
*/



fn run(configuration_path: Path) -> Result<()> {
    let feed_config = read_configuration(configuration_path)?;

    /*
    let mut statuses: Vec<NewStatus>;
    for feed_config in config.feeds {
        let content = reqwest::get(feed_config.url).await.expect("Failed to get feed");
        let content = content.bytes().await.expect("Failed to get content bytes");

        let cache_path = Path::new(&feed_config.cache_file);
    }


    let content = format!("New Pale Chapter!\n{}\n{}", title, link);

    let status = StatusBuilder::new()
        .status(content)
        .visibility(Visibility::Public)
        .language(Language::Eng)
        .build().expect("Failed to build status");

    let config_path = Path::new("augur_config.toml");
    let auth_data = toml::from_file(config_path).expect("Failed to get auth data from environment");
    let mastodon = Mastodon::from(auth_data);
    mastodon.new_status(status).expect("failed to create new status");
    */

    Ok(())
}
