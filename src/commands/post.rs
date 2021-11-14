use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter::{Twitter, TwitterCreateResponseData};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;

struct Args {
    message: Option<String>,
}

fn parse(args: &BaseArgs) -> Args {
    Args {
        message: args.get_position::<String>(1),
    }
}

fn help() -> Result<(), TwitterError> {
    println!("TODO: document");
    Ok(())
}

struct TwitterHistoryFile {
    tweets: Vec<TwitterCreateResponseData>,
}

// Not a good look to have this copoy/pasted from credentials.rs ¯\_(ツ)_/¯
fn home_dir() -> PathBuf {
    home::home_dir().expect("Cannot get your home directory!")
}

fn create_or_open_history() -> Result<File, std::io::Error> {
    let mut path = PathBuf::from(home_dir());
    path.push(".twitter_history.toml");
    path = fs::canonicalize(&path)?;
    OpenOptions::new().write(true).create(true).open(path)
}

fn append_response_to_history(response: TwitterCreateResponseData) -> Result<(), TwitterError> {
    let history = create_or_open_history()?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // match contents {

    //     let contents = toml::to_string(HistoryFile { tweets: vec![response] })?;
    //     fs::write(path, contents)?;
    // }

    Ok(())
}

pub fn execute(base_args: &BaseArgs) -> Result<(), TwitterError> {
    if base_args.is_nth_argument_help(1) {
        return help();
    }
    let args = parse(&base_args);
    let credentials = credentials::get(base_args)?;
    dbg!(&credentials);

    match args.message {
        Some(message) if &message != "" => {
            let response = Twitter::new(credentials).post_v2(&message)?;
            println!("Posted tweet id: {}", response.id);
            append_response_to_history(response)
        }
        _ => Err(TwitterError::MissingArgument("message".to_string())),
    }
}
