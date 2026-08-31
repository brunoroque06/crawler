use crate::time::DateTimeUtc;

pub trait Source {
    fn parse(&self, body: &str) -> Result<Vec<Item>, String>;

    fn url(&self, url: &str) -> Result<String, String> {
        Ok(url.to_owned())
    }
}

#[derive(Debug)]
pub struct Item {
    pub title: String,
    pub url: String,
    pub pub_date: DateTimeUtc,
}
