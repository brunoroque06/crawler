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

use crate::args::parse_args;
use crate::atom::Atom;
use crate::gnews::Gnews;
use crate::mux::{Specs, deliver, dispatch};
use crate::reddit::Reddit;
use crate::rss::Rss;

fn main() -> Result<(), String> {
    let args = parse_args();

    let specs = vec![
        Specs(args.atom, &Atom),
        Specs(args.gnews, &Gnews),
        Specs(args.reddit, &Reddit),
        Specs(args.rss, &Rss),
    ];

    let feeds = dispatch(specs);

    deliver(feeds, args.cutoff, args.last, args.output);

    Ok(())
}
