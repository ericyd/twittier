use super::args::Args;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::Value;

#[derive(Deserialize)]
pub struct Credentials {
    pub access_token: String,
    pub access_token_secret: String,
}

pub fn get_credentials(args: &Args) -> Result<Credentials, String> {
    let home_dir = home::home_dir().expect("Cannot get your home directory! Please pass the path to your .twitterrc manually using <TODO: Make arg for this>");
    let mut path = PathBuf::from(home_dir);
    path.push(&args.credentials_file);

    // TODO: there is probably a more rusty way to do this
    match fs::canonicalize(&path) {
        Ok(path) => match fs::read_to_string(&path) {
            Ok(contents) => match toml::from_str::<Credentials>(&contents) {
                Ok(value) => Ok(value),
                Err(e) => Err(format!("TOML error: {} of {:?}", e, &path)),
            },
            Err(e) => Err(format!("Error reading file: {}", e)),
        },
        Err(e) => Err(format!("Error canonicalizing path: {:?}", e)),
    }
}
