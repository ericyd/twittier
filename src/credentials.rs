use super::args::BaseArgs;
use super::error::TwitterError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct Credentials {
    pub api_key: String,
    pub api_key_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

impl From<&Value> for Credentials {
    fn from(value: &Value) -> Credentials {
        match value {
            Value::Table(fields) => Credentials {
                api_key: fields["api_key"].as_str().unwrap_or("").to_string(),
                api_key_secret: fields["api_key_secret"].as_str().unwrap_or("").to_string(),
                access_token: fields["access_token"].as_str().unwrap_or("").to_string(),
                access_token_secret: fields["access_token_secret"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            },
            _ => {
                panic!("Credentials file not formatted correctly! Try using `tw init`")
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CredentialsFile {
    pub default: Credentials,
}

fn home_dir() -> PathBuf {
    home::home_dir()
        .expect("Cannot get your home directory! Please pass the path to your .twitter_credentials.toml manually using -c or --credentials")
}

fn is_any_empty(credentials: &Credentials) -> bool {
    credentials.api_key.is_empty()
        || credentials.api_key_secret.is_empty()
        || credentials.access_token.is_empty()
        || credentials.access_token_secret.is_empty()
}

pub fn get(base_args: &BaseArgs) -> Result<Credentials, TwitterError> {
    let credentials_file = base_args.get(
        "credentials",
        "c",
        String::from(".twitter_credentials.toml"),
    );
    let mut path = PathBuf::from(home_dir());
    path.push(&credentials_file);

    path = fs::canonicalize(&path)?;
    let contents = fs::read_to_string(&path)?;

    match base_args.get_option::<String>("profile", "p") {
        Some(profile) => {
            let credentials: Value = toml::from_str(&contents)?;
            let profile_credentials: Credentials = credentials
                .get(&profile)
                .ok_or(TwitterError::ProfileNotFound(profile.to_string()))?
                .into();
            if is_any_empty(&profile_credentials) {
                panic!("Profile {} has empty fields! Please ensure all values are present and non-empty", profile);
            }
            Ok(profile_credentials)
        }
        None => {
            let credentials: CredentialsFile = toml::from_str(&contents)?;
            Ok(credentials.default)
        }
    }
}

fn write_empty_credentials(path: &PathBuf) -> Result<(), TwitterError> {
    let credentials = Credentials {
        api_key: "".to_string(),
        api_key_secret: "".to_string(),
        access_token: "".to_string(),
        access_token_secret: "".to_string(),
    };
    let credentials_file = CredentialsFile {
        default: credentials,
    };
    let contents = toml::to_string(&credentials_file)?;
    fs::write(path, contents)?;
    println!(
        "✅ Credentials file succesfully initialized. Please open {:?} and fill in the values",
        &path
    );
    Ok(())
}

pub fn init(base_args: &BaseArgs) -> Result<(), TwitterError> {
    let credentials_file = base_args.get(
        "credentials",
        "c",
        String::from(".twitter_credentials.toml"),
    );
    let mut path = PathBuf::from(home_dir());
    path.push(&credentials_file);

    match fs::canonicalize(&path) {
        Ok(_) => {
            let contents = fs::read_to_string(&path)?;
            if contents != "" {
                println!(
                    "🤨 Credentials file ({:?}) already exists and is non-empty!",
                    &path
                );
                Ok(())
            } else {
                write_empty_credentials(&path)
            }
        }
        Err(_) => write_empty_credentials(&path),
    }
}
