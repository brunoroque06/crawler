use std::{
    fmt::{Error, Write},
    str::FromStr,
};

use crate::{
    http::Client,
    source::{Item, Source},
    time::DateTimeUtc,
};

#[derive(Debug, PartialEq)]
pub struct Spec {
    title: String,
    url: String,
}

impl FromStr for Spec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (title, url) = s
            .split_once('=')
            .filter(|(t, u)| !t.trim().is_empty() && !u.trim().is_empty())
            .ok_or_else(|| format!("expected title=url, but got {s}"))?;

        Ok(Self {
            title: title.into(),
            url: url.into(),
        })
    }
}

pub struct Specs<'a>(pub Vec<Spec>, pub &'a dyn Source);

#[derive(Debug)]
pub struct Feed {
    title: String,
    items: Result<Vec<Item>, String>,
}

pub fn dispatch(specs: Vec<Specs>) -> Result<Vec<Feed>, String> {
    let client = Client::new()?;
    Ok(specs
        .into_iter()
        .flat_map(|Specs(specs, src)| {
            specs.into_iter().map(|s| {
                let items = (|| {
                    let url = src.url(&s.url)?;
                    let body = client.get(&url)?;
                    src.parse(&body)
                })();
                Feed {
                    title: s.title,
                    items,
                }
            })
        })
        .collect())
}

#[derive(Debug)]
pub enum Output {
    Plain,
    Tty,
}

impl FromStr for Output {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(Output::Plain),
            "tty" => Ok(Output::Tty),
            _ => Err(format!("expected plain or tty, got {s}")),
        }
    }
}

pub fn compose(feeds: &[Feed], cutoff: u8, last: u8, output: Output) -> Result<String, Error> {
    let cutoff = DateTimeUtc::hours_ago(i64::from(cutoff));
    let mut out = String::new();

    for f in feeds {
        writeln!(out, "{}", f.title)?;
        let items = match &f.items {
            Ok(items) => items,
            Err(e) => {
                writeln!(out, "\t{}\n", e)?;
                continue;
            }
        };
        for i in items
            .iter()
            .filter(|i| i.pub_date > cutoff)
            .take(usize::from(last))
        {
            match output {
                Output::Plain => writeln!(out, "\t{} {} {}", i.pub_date, i.title, i.url)?,
                Output::Tty => writeln!(
                    out,
                    "\t\x1b]8;;{}\x1b\\{} {}\x1b]8;;\x1b\\",
                    i.url, i.pub_date, i.title
                )?,
            }
        }
        writeln!(out)?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec_err(val: &str) -> Result<Spec, String> {
        Err(format!("expected title=url, but got {}", val))
    }

    #[test]
    fn spec_needs_eq() {
        assert_eq!(Spec::from_str("no_eq"), spec_err("no_eq"));
    }

    #[test]
    fn spec_no_empty() {
        assert_eq!(Spec::from_str("title="), spec_err("title="));
        assert_eq!(Spec::from_str("title= "), spec_err("title= "));
        assert_eq!(Spec::from_str("=url"), spec_err("=url"));
        assert_eq!(Spec::from_str(" =url"), spec_err(" =url"));
    }

    #[test]
    fn spec() {
        assert_eq!(
            Spec::from_str("title=url"),
            Ok(Spec {
                title: "title".to_owned(),
                url: "url".to_owned()
            })
        );
    }
}
