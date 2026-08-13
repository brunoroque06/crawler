mod args;
mod atom;
mod gnews;
mod http;
mod mux;
mod reddit;
mod rss;
mod source;
mod time;
mod xml;

use crate::args::{ArgDef, ArgType, parse_args};
use crate::atom::Atom;
use crate::gnews::Gnews;
use crate::mux::{deliver, dispatch};
use crate::reddit::Reddit;
use crate::rss::Rss;
use crate::source::Sources;

fn main() -> Result<(), String> {
    let sources = Sources::new().add(Atom).add(Gnews).add(Reddit).add(Rss);

    let flags = sources
        .args()
        .into_iter()
        .chain([
            ArgDef {
                key: "cutoff".to_string(),
                typ: ArgType::Uint,
            },
            ArgDef {
                key: "last".to_string(),
                typ: ArgType::Uint,
            },
        ])
        .collect::<Vec<_>>();

    let args = parse_args(&flags)?;

    let feeds = dispatch(&sources, args);

    deliver(feeds);

    Ok(())
}
