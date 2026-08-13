use crate::{
    args::Arg,
    http::get,
    source::{SourceItem, Sources},
    time::now,
};

#[derive(Debug)]
pub struct Feed {
    title: String,
    items: Result<Vec<SourceItem>, String>,
}

pub fn dispatch(srcs: &Sources, args: Vec<Arg>) -> Vec<Feed> {
    let urls = args.into_iter().filter_map(|a| match a {
        Arg::Uint { .. } => None,
        Arg::Url { key, title, url } => Some((key, title, url)),
    });

    urls.into_iter()
        .map(|(key, title, url)| {
            let items = (|| {
                let src = srcs
                    .get(&key)
                    .ok_or_else(|| format!("unknown source {key}"))?;
                let url = src.url(&url)?;
                let body = get(&url)?;
                src.parse(&body)
            })();

            Feed { title, items }
        })
        .collect()
}

pub fn deliver(feeds: Vec<Feed>) {
    let now = now(25);
    for f in feeds {
        println!("{}", f.title);
        match &f.items {
            Err(e) => {
                eprintln!("\t{}", e)
            }
            Ok(items) => {
                for i in items.iter().filter(|i| i.pub_date.0 > now).take(8) {
                    println!("\t{} - {}", i.title, i.url)
                }
            }
        }
        println!();
    }
}
