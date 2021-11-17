use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter::{Twitter, TwitterCreateResponseData};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;

struct Args {
    count: i32,
}

fn parse(args: &BaseArgs) -> Args {
    Args {
        // TODO: this should just be positional
        count: args.get("count", "n", 1),
    }
}

fn help() -> Result<(), TwitterError> {
    println!("TODO: document");
    Ok(())
}

pub fn execute(base_args: &BaseArgs) -> Result<(), TwitterError> {
    if base_args.is_nth_argument_help(1) {
        return help();
    }
    let args = parse(&base_args);
    let credentials = credentials::get(base_args)?;
    dbg!(&credentials);

    let feed = Twitter::new(credentials).feed(args.count)?;

    for item in feed {
        item.display();
    }

    Ok(())
}
