use std::collections::HashMap;

use crate::{
    args::{ArgDef, ArgType},
    time::DateTimeUtc,
};

pub trait Source {
    fn key(&self) -> String;

    fn parse(&self, body: &str) -> Result<Vec<SourceItem>, String>;

    fn url(&self, url: &str) -> Result<String, String> {
        Ok(url.to_string())
    }
}

#[derive(Debug)]
pub struct SourceItem {
    pub title: String,
    pub url: String,
    pub pub_date: DateTimeUtc,
}

pub struct Sources {
    sources: HashMap<String, Box<dyn Source>>,
}

impl Sources {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn add<S>(mut self, src: S) -> Self
    where
        S: Source + 'static,
    {
        self.sources.insert(src.key(), Box::new(src));
        self
    }

    pub fn args(&self) -> Vec<ArgDef> {
        self.sources
            .keys()
            .map(|k| ArgDef {
                key: k.clone(),
                typ: ArgType::Url,
            })
            .collect()
    }

    pub fn get(&self, key: &str) -> Option<&dyn Source> {
        self.sources.get(key).map(Box::as_ref)
    }
}
