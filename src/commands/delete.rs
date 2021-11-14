use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter::{Twitter, TwitterCreateResponseData};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;

struct Args {
    id: Option<String>,
}

fn parse(args: &BaseArgs) -> Args {
    Args {
        id: args.get_position::<String>(1),
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

    match args.id {
        // TODO: find latest tweet and delete
        // Some(id) if *id == "last" => {
        //     Twitter::new(credentials).post_v2(&message).unwrap()
        //     Ok(())
        // }
        Some(id) if id != "" => {
            let response = Twitter::new(credentials).delete_v2(&id)?;
            if response.deleted == true {
                Ok(())
            } else {
                Err(TwitterError::Api(format!(
                    "Error deleting tweet ID: {}",
                    id
                )))
            }
        }
        _ => Err(TwitterError::MissingArgument("id".to_string())),
    }
}
