use super::args::Args;
use super::credentials;
use super::error::TwitterError;
use super::twitter::{Twitter, TwitterCreateResponseData};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;

struct TwitterHistoryFile {
    tweets: Vec<TwitterCreateResponseData>
}

// Not a good look to have this copoy/pasted from credentials.rs ¯\_(ツ)_/¯
fn home_dir() -> PathBuf {
    home::home_dir()
        .expect("Cannot get your home directory!")
}

fn create_or_open_history() -> Result<File, std::io::Error> {
    let mut path = PathBuf::from(home_dir());
    path.push(".twitter_history.toml");
    path = fs::canonicalize(&path)?;
    OpenOptions::new().write(true).create(true).open(path)
}

fn append_response_to_history(response: TwitterCreateResponseData) -> Result<(), TwitterError> {
    let mut history = create_or_open_history()?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // match contents {

    //     let contents = toml::to_string(HistoryFile { tweets: vec![response] })?;
    //     fs::write(path, contents)?;
    // }

    Ok(())
}

pub fn post(args: &Args) -> Result<(), TwitterError> {
    let credentials = credentials::get(args)?;
    dbg!(&credentials);

    match &args.message {
        Some(message) if message != "" => {
            let response = Twitter::new(credentials).post_v2(&message)?;
            append_response_to_history(response)
        }
        _ => Err(TwitterError::MissingArgument("message".to_string())),
    }
}

pub fn delete(args: &Args) -> Result<(), TwitterError> {
    let credentials = credentials::get(args)?;
    dbg!(&credentials);

    match &args.raw.get("1") {
        // TODO: find latest tweet and delete
        // Some(id) if *id == "last" => {
        //     Twitter::new(credentials).post_v2(&message).unwrap()
        //     Ok(())
        // }
        Some(id) if *id != "" => {
            let response = Twitter::new(credentials).delete_v2(id)?;
            if response.deleted == true {
                Ok(())
            } else {
                Err(TwitterError::Api(format!("Error deleting tweet ID: {}", id)))
            }
        }
        // TODO: update help message when `last` is implemented
        _ => panic!("No `id` argument supplied. Please specify an id. Example: `tw delete 123456`"),
    }
}