use std::str::FromStr;

use crate::{
    http::get,
    source::{Item, Source},
    time::hours_ago,
};

#[derive(Debug, PartialEq)]
pub struct Spec {
    pub title: String,
    pub url: String,
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

pub fn dispatch(specs: Vec<Specs>) -> Vec<Feed> {
    specs
        .into_iter()
        .flat_map(|Specs(specs, src)| {
            specs.into_iter().map(|s| {
                let items = (|| {
                    let url = src.url(&s.url)?;
                    let body = get(&url)?;
                    src.parse(&body)
                })();
                Feed {
                    title: s.title,
                    items,
                }
            })
        })
        .collect()
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

pub fn deliver(feeds: Vec<Feed>, cutoff: u8, last: u8, output: Output) {
    let cutoff = hours_ago(i64::from(cutoff));
    for f in feeds {
        println!("{}", f.title);
        match &f.items {
            Err(e) => {
                eprintln!("\t{}", e)
            }
            Ok(items) => {
                for i in items
                    .iter()
                    .filter(|i| i.pub_date.0 > cutoff)
                    .take(usize::from(last))
                {
                    match output {
                        Output::Plain => println!("\t{} {} {}", i.pub_date, i.title, i.url),
                        Output::Tty => println!(
                            "\t\x1b]8;;{}\x1b\\{} {}\x1b]8;;\x1b\\",
                            i.url, i.pub_date, i.title
                        ),
                    }
                    // println!("- [{} {}]({})\n", i.pub_date, i.title, i.url);
                }
            }
        }
        println!();
    }
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
                title: "title".to_string(),
                url: "url".to_string()
            })
        );
    }
}
