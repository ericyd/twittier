use super::super::args::BaseArgs;
use super::super::credentials::{Credentials, CredentialsFile};
use super::super::error::TwitterError;
use std::fs;
use std::path::PathBuf;

struct Args {
    credentials_file: String,
}

fn parse(args: &BaseArgs) -> Args {
    let credentials_file = args.get(
        "credentials",
        "c",
        String::from(".twitter_credentials.toml"),
    );
    Args { credentials_file }
}

fn help() -> Result<(), TwitterError> {
    println!(
        "Initialize your credentials file!\n
    Usage: tw init [OPTIONS]

    Options:
        -c, --credentials <name>
            The file name or path to use for the credentials file.
            Default: ~/.twitter_credentials.toml

    Examples:
        Read 10 tweets from your feed (default):
            tw feed
        Read 20 tweets from your feed:
            tw feed 20
        Read 1 tweet from your alt feed:
            tw feed 1 -p alt1
"
    );
    Ok(())
}

fn home_dir() -> PathBuf {
    home::home_dir()
        .expect("Cannot get your home directory! Please pass the path to your .twitter_credentials.toml manually using -c or --credentials")
}

fn write_empty_credentials(path: &PathBuf) -> Result<(), TwitterError> {
    let credentials = Credentials {
        api_key: "".to_string(),
        api_key_secret: "".to_string(),
        access_token: "".to_string(),
        access_token_secret: "".to_string(),
        handle: "".to_string(),
    };
    let credentials_file = CredentialsFile {
        default: credentials,
    };
    let contents = toml::to_string(&credentials_file)?;
    match fs::write(path, contents) {
        Ok(_) => {
            println!(
              "✅ Credentials file succesfully initialized. Please open {:?} and fill in the values",
              &path
          );
            Ok(())
        }
        Err(e) => {
            println!("Could not initialize credentials file!");
            println!("Please ensure the credentials file path {:?} is a valid relative or absolute path name.", &path);
            Err(TwitterError::Io(e))
        }
    }
}

pub fn execute(base_args: &BaseArgs) -> Result<(), TwitterError> {
    if base_args.is_requesting_help() {
        return help();
    }
    let args = parse(&base_args);
    let mut path = PathBuf::from(home_dir());
    path.push(args.credentials_file);

    match fs::canonicalize(&path) {
        Ok(_) => match fs::read_to_string(&path) {
            Ok(contents) if contents != "" => {
                println!(
                    "🤨 Credentials file {:?} already exists and is non-empty!",
                    &path
                );
                Ok(())
            }
            Ok(_) => write_empty_credentials(&path),
            Err(e) => {
                println!("Could not initialize credentials file!");
                println!("Please ensure the credentials file path {:?} is a valid relative or absolute path name.", &path);
                Err(TwitterError::Io(e))
            }
        },
        Err(_) => write_empty_credentials(&path),
    }
}
