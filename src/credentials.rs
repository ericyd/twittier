use super::args::Args;
use super::error::TwitterError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub api_key: String,
    pub api_key_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

pub fn get(args: &Args) -> Result<Credentials, TwitterError> {
    let home_dir = home::home_dir().expect("Cannot get your home directory! Please pass the path to your .twitter_credentials.toml manually using -c or --credentials");
    let mut path = PathBuf::from(home_dir);
    path.push(&args.credentials_file);

    path = fs::canonicalize(&path)?;
    let contents = fs::read_to_string(&path)?;
    Ok(toml::from_str::<Credentials>(&contents)?)
}

// TODO: write file if it doesn't exist or it is empty
pub fn init(args: &Args) -> Result<(), TwitterError> {
    Ok(())
    // let home_dir = home::home_dir().expect("Cannot get your home directory! Please pass the path to your .twitter_credentials.toml manually using -c or --credentials");
    // let mut path = PathBuf::from(home_dir);
    // path.push(&args.credentials_file);

    // path = fs::canonicalize(&path)?;
    // let contents = fs::read_to_string(&path)?;
    // Ok(toml::from_str::<Credentials>(&contents)?)
}
