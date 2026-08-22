use crate::{
    args::Spec,
    http::get,
    source::{Item, Source},
    time::now,
};

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

pub fn deliver(feeds: Vec<Feed>, cutoff: u8, last: u8) {
    let now = now(i64::from(cutoff));
    for f in feeds {
        println!("{}", f.title);
        match &f.items {
            Err(e) => {
                eprintln!("\t{}", e)
            }
            Ok(items) => {
                for i in items
                    .iter()
                    .filter(|i| i.pub_date.0 > now)
                    .take(usize::from(last))
                {
                    println!(
                        "\t\x1b]8;;{}\x1b\\{} {}\x1b]8;;\x1b\\",
                        i.url, i.pub_date, i.title
                    );
                    // println!("- [{} {}]({})\n", i.pub_date, i.title, i.url);
                }
            }
        }
        println!();
    }
}
