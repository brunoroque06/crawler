use serde::Deserialize;

use crate::{
    source::{Item, Source},
    time::DateTimeUtc,
    xml::from_str,
};

pub struct Atom;

impl Source for Atom {
    fn parse(&self, body: &str) -> Result<Vec<Item>, String> {
        from_str::<AtomFeed>(body).map(|r| {
            r.entries
                .into_iter()
                .map(|i| Item {
                    title: i.title,
                    url: i.link.href,
                    pub_date: i.updated,
                })
                .collect()
        })
    }
}

#[derive(Debug, Deserialize)]
struct AtomFeed {
    #[serde(rename = "entry", default)]
    entries: Vec<AtomEntry>,
}

#[derive(Debug, Deserialize)]
struct AtomEntry {
    title: String,
    link: AtomLink,
    updated: DateTimeUtc,
}

#[derive(Debug, Deserialize)]
struct AtomLink {
    #[serde(rename = "@href")]
    href: String,
}
