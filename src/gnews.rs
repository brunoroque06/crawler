use crate::{
    http::build_url,
    rss::Rss,
    source::{Item, Source},
};

pub struct Gnews;

impl Source for Gnews {
    fn url(&self, url: &str) -> Result<String, String> {
        build_url(
            "https://news.google.com/rss/search",
            &[("hl", "en-US"), ("gl", "US"), ("ceid", "US:en"), ("q", url)],
        )
    }

    fn parse(&self, body: &str) -> Result<Vec<Item>, String> {
        Rss.parse(body)
    }
}
