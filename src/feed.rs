use rss::{Channel, Item};

use thiserror::Error;
use anyhow::Result;

use std::collections::HashMap;

#[derive(Error, Debug)]
enum FeedError {
    #[error("Feed missing Id")]
    MissingId,
}

pub fn uncached_items<'a>(config_feed: &'a Channel, cached_feed: &Channel) -> Result<Vec<&'a Item>> {
    let mut cached_ids = HashMap::new();

    for item in &cached_feed.items {
        let guid = item.guid().ok_or_else(|| FeedError::MissingId)?.value();

        cached_ids.insert(guid.to_string(), item);
    }
    let mut new_items = vec![];
    for item in &config_feed.items {
        let id = item.guid().ok_or_else(|| FeedError::MissingId)?.value();
        if !cached_ids.contains_key(id) {
          new_items.push(item);
        }
    }

    Ok(new_items)
}

#[cfg(test)]
mod tests {
    use rss::{Item, ChannelBuilder, ItemBuilder, GuidBuilder};
    use super::uncached_items;

    fn generate_item_sequence(sequence: String) -> Item {
        ItemBuilder::default()
            .title(Some(format!("Item {sequence}")))
            .author(Some(format!("Author")))
            .link(Some(format!("https://www.example.com/item/{sequence}")))
            .description(Some(format!("Lorem Ipsum {sequence}")))
            .guid(Some(GuidBuilder::default().
                  value(sequence)
                  .build()
                  ))
            .build()
    }

    #[test]
    fn finds_one_new_item() {
        let item1 = generate_item_sequence('1'.to_string());
        let item2 = generate_item_sequence('2'.to_string());

        let channel1 = 
            ChannelBuilder::default()
            .title("Channel")
            .items(vec![item1.clone()])
            .build();

        let channel2 = 
            ChannelBuilder::default()
            .title("Channel")
            .items(vec![item2, item1])
            .build();


        let new_items = uncached_items(&channel2, &channel1).expect("Uncached items returned error");
        assert_eq!(new_items.len(), 1);
    }

    #[test]
    fn finds_two_new_items() {
        let item1 = generate_item_sequence('1'.to_string());
        let item2 = generate_item_sequence('2'.to_string());
        let item3 = generate_item_sequence('3'.to_string());
        let item4 = generate_item_sequence('4'.to_string());

        let channel1 = 
            ChannelBuilder::default()
            .title("Channel")
            .items(vec![item2.clone(), item1.clone()])
            .build();

        let channel2 = 
            ChannelBuilder::default()
            .title("Channel")
            .items(vec![item4, item3, item2, item1])
            .build();


        let new_items = uncached_items(&channel2, &channel1).expect("Uncached items returned error");
        println!("new_items: {:?}", new_items);
        assert_eq!(new_items.len(), 2);
    }

    #[test]
    fn finds_no_new_items() {
        let item1 = generate_item_sequence('1'.to_string());
        let item2 = generate_item_sequence('2'.to_string());
        let item3 = generate_item_sequence('3'.to_string());
        let item4 = generate_item_sequence('4'.to_string());

        let channel1 = 
            ChannelBuilder::default()
            .title("Channel")
            .items(vec![item4.clone(), item3.clone(), item2.clone(), item1.clone()])
            .build();

        let channel2 = 
            ChannelBuilder::default()
            .title("Channel")
            .items(vec![item4, item3, item2, item1])
            .build();


        let new_items = uncached_items(&channel2, &channel1).expect("Uncached items returned error");
        assert_eq!(new_items.len(), 0);
    }
}
