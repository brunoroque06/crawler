use crate::{atom::Atom, source::Source};

pub struct Reddit;

impl Source for Reddit {
    fn key(&self) -> String {
        "reddit".to_string()
    }

    fn parse(&self, body: &str) -> Result<Vec<crate::source::SourceItem>, String> {
        Atom.parse(body)
    }

    fn url(&self, sub: &str) -> Result<String, String> {
        Ok(format!("https://www.reddit.com/r/{sub}/.rss"))
    }
}
