use serde::Deserialize;

use crate::{
    source::{Item, Source},
    time::DateTimeUtc,
    xml::from_str,
};

pub struct Rss;

impl Source for Rss {
    fn parse(&self, body: &str) -> Result<Vec<Item>, String> {
        from_str::<RssFeed>(body).map(|r| {
            r.channel
                .items
                .into_iter()
                .map(|i| Item {
                    title: i.title,
                    url: i.link,
                    pub_date: i.pub_date,
                })
                .collect()
        })
    }
}

#[derive(Debug, Deserialize)]
struct RssFeed {
    channel: RssChannel,
}

#[derive(Debug, Deserialize)]
struct RssChannel {
    #[serde(rename = "item", default)]
    items: Vec<RssItem>,
}

#[derive(Debug, Deserialize)]
struct RssItem {
    title: String,
    link: String,
    #[serde(rename = "pubDate")]
    pub_date: DateTimeUtc,
}
