use argh::FromArgs;
use std::str::FromStr;

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
            .ok_or_else(|| format!("expected title=url, but got {s}"))?;

        Ok(Self {
            title: title.into(),
            url: url.into(),
        })
    }
}

fn default_cutoff() -> u8 {
    25
}

fn default_last() -> u8 {
    8
}

/// feed
#[derive(Debug, FromArgs)]
pub struct Args {
    /// title=url
    #[argh(option)]
    pub atom: Vec<Spec>,

    /// title=url
    #[argh(option)]
    pub gnews: Vec<Spec>,

    /// title=url
    #[argh(option)]
    pub reddit: Vec<Spec>,

    /// title=url
    #[argh(option)]
    pub rss: Vec<Spec>,

    /// in hours
    #[argh(option, default = "default_cutoff()")]
    pub cutoff: u8,

    /// last items
    #[argh(option, default = "default_last()")]
    pub last: u8,
}

pub fn parse_args() -> Args {
    argh::from_env()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_needs_eq() {
        assert_eq!(
            Spec::from_str("no_eq"),
            Err("expected title=url, but got no_eq".to_string())
        );
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
