use crate::{atom::Atom, source::Source};

pub struct Reddit;

impl Source for Reddit {
    fn parse(&self, body: &str) -> Result<Vec<crate::source::Item>, String> {
        Atom.parse(body)
    }

    fn url(&self, sub: &str) -> Result<String, String> {
        Ok(format!("https://www.reddit.com/r/{sub}/.rss"))
    }
}
