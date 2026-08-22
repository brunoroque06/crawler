use argh::FromArgs;

use crate::mux::{Output, Spec};

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
    #[argh(option, default = "25")]
    pub cutoff: u8,

    /// last items
    #[argh(option, default = "8")]
    pub last: u8,

    /// output format
    #[argh(option, default = "Output::Tty")]
    pub output: Output,
}

pub fn parse_args() -> Args {
    argh::from_env()
}
