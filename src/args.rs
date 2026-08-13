use std::{collections::HashMap, env::args, iter::from_fn};

pub struct ArgDef {
    pub key: String,
    pub typ: ArgType,
}

pub enum ArgType {
    Uint,
    Url,
}

#[derive(Debug, PartialEq)]
pub enum Arg {
    Uint {
        key: String,
        value: u16,
    },
    Url {
        key: String,
        title: String,
        url: String,
    },
}

impl TryFrom<(&ArgDef, String)> for Arg {
    type Error = String;

    fn try_from((def, str): (&ArgDef, String)) -> Result<Self, Self::Error> {
        let def_name = def.key.clone();
        match def.typ {
            ArgType::Uint => str
                .parse::<u16>()
                .map_err(|_| format!("cannot parse {} as uint", str))
                .map(|v| Arg::Uint {
                    key: def_name,
                    value: v,
                }),
            ArgType::Url => {
                let (title, url) = str
                    .split_once('=')
                    .ok_or_else(|| format!("expected title=url, but got {}", str))?;

                Ok(Arg::Url {
                    key: def_name,
                    title: title.to_string(),
                    url: url.to_string(),
                })
            }
        }
    }
}

pub fn parse_args(defs: &[ArgDef]) -> Result<Vec<Arg>, String> {
    parse_args_from(defs, args().skip(1))
}

fn parse_args_from(
    defs: &[ArgDef],
    args: impl Iterator<Item = String>,
) -> Result<Vec<Arg>, String> {
    let def_by_name: HashMap<&str, &ArgDef> = defs.iter().map(|d| (d.key.as_str(), d)).collect();

    pairs(args)
        .map(|p| {
            let (key, value) = p?;

            let name = key
                .strip_prefix("--")
                .ok_or_else(|| format!("flag {} must start with --", key))?;

            let def = def_by_name
                .get(name)
                .ok_or_else(|| format!("unknown flag {name}"))?;

            Arg::try_from((*def, value))
        })
        .collect()
}

fn pairs(
    mut args: impl Iterator<Item = String>,
) -> impl Iterator<Item = Result<(String, String), String>> {
    from_fn(move || {
        let key = args.next()?;
        Some(match args.next() {
            Some(value) => Ok((key, value)),
            None => Err(format!("missing value for {key}")),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Vec<Arg>, String> {
        parse_args_from(
            &[
                ArgDef {
                    key: "rss".to_string(),
                    typ: ArgType::Url,
                },
                ArgDef {
                    key: "back".to_string(),
                    typ: ArgType::Uint,
                },
            ],
            args.iter().map(|v| v.to_string()),
        )
    }

    #[test]
    fn needs_pairs() {
        assert_eq!(
            parse(&["--rss"]),
            Err("missing value for --rss".to_string())
        );
    }

    #[test]
    fn key_needs_dashes() {
        assert_eq!(
            parse(&["rss", "blog"]),
            Err("flag rss must start with --".to_string())
        );
    }

    #[test]
    fn unknown_flag() {
        assert_eq!(
            parse(&["--lie", "blog"]),
            Err("unknown flag lie".to_string())
        );
    }

    #[test]
    fn no_int() {
        assert_eq!(
            parse(&["--back", "not-an-int"]),
            Err("cannot parse not-an-int as uint".to_string())
        );
    }

    #[test]
    fn uint() {
        assert_eq!(
            parse(&["--back", "8"]),
            Ok(vec![Arg::Uint {
                key: "back".to_string(),
                value: 8
            }])
        );
    }

    #[test]
    fn url_needs_eq() {
        assert_eq!(
            parse(&["--rss", "no eq"]),
            Err("expected title=url, but got no eq".to_string())
        );
    }

    #[test]
    fn url() {
        assert_eq!(
            parse(&["--rss", "title=url"]),
            Ok(vec![Arg::Url {
                key: "rss".to_string(),
                title: "title".to_string(),
                url: "url".to_string()
            }])
        );
    }
}
