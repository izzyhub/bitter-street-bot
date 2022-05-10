use std::io::BufReader;
use rss::{Channel, Item};

fn find_new_items(reader: BufReader, cache_path: Path) -> Result<Vec<Item>> {
    let mut new_items = vec![];

    let channel = Channel::read_from(&reader[..])?;

    if !cache_path.exists() {
        // Path doesn't exist. Just take the most recent
        // n to avoid spamming everything.

        let item = &remote_channel.items.first().expect("No items in feed");

        let title = item.title().expect("New item has no title");
        let link = item.link().expect("New item has no link");
    }
    else {

    }


    Ok(new_items)
}
