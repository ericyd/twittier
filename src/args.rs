// AGAINST ALL ODDS, THIS WORKS.....
// Why though?

// ripgrep has a bunch of custom logic, I feel like this might be required
// https://github.com/BurntSushi/ripgrep/blob/af54069c51cc3656c9c343a7fb3c9360cfddf505/crates/core/args.rs#L228-L250

use super::error::TwitterError;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
struct ArgParser {
    map: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Command {
    Help,
    Version,
    Tweet,
    Login,
    Init,
}

// simple argument collector
impl ArgParser {
    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().collect();
        // Start at 1 to omit the executable name
        let mut i = 1;
        let mut position = 0;
        let mut map = HashMap::new();

        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("--") {
                let key = arg.trim_start_matches("--").to_string();
                let value = if i + 1 < args.len() { &args[i + 1] } else { "" };
                map.insert(key, value.to_string());
                i += 2;
            } else if arg.starts_with("-") {
                let key = arg.trim_start_matches("-").to_string();
                let value = if i + 1 < args.len() { &args[i + 1] } else { "" };
                map.insert(key, value.to_string());
                i += 2;
            } else {
                map.insert(String::from(position.to_string()), arg.to_string());
                i += 1;
                position += 1;
            }
        }

        ArgParser { map }
    }

    pub fn get<T: FromStr>(&self, long_name: &str, short_name: &str, default: T) -> T {
        match self.map.get(long_name) {
            Some(thing) => match thing.parse::<T>() {
                Ok(val) => val,
                Err(_err) => default,
            },
            None => match self.map.get(short_name) {
                Some(thing) => match thing.parse::<T>() {
                    Ok(val) => val,
                    Err(_err) => default,
                },
                None => default,
            },
        }
    }

    pub fn get_option<T: FromStr>(&self, long_name: &str, short_name: &str) -> Option<T> {
        match self.map.get(long_name) {
            Some(thing) => match thing.parse::<T>() {
                Ok(val) => Some(val),
                Err(_err) => None,
            },
            None => match self.map.get(short_name) {
                Some(thing) => match thing.parse::<T>() {
                    Ok(val) => Some(val),
                    Err(_err) => None,
                },
                None => None,
            },
        }
    }
}

impl Display for ArgParser {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let key_values = self
            .map
            .iter()
            .fold(Vec::<String>::new(), |vec, (k, v)| {
                [&vec[..], &vec![format!("{}: {}", k, v)]].concat()
            })
            .join(", ");
        write!(f, "ArgParser <{}>", key_values)
    }
}

// This struct represents a deserialized set of all possible arguments accepted by the program.
// I don't really think this is the absolute *best* way to do this but it has some advantages and it might be alright.
#[derive(Debug)]
pub struct Args {
    pub command: Command,
    pub credentials_file: String,
    pub message: Option<String>,
    pub profile: Option<String>,
}

impl Args {
    // TODO: Does this really need to be a result? What possible errors could we encounter?
    pub fn parse() -> Result<Self, TwitterError> {
        let args = ArgParser::new();
        dbg!(&args);
        let command = command(&args);

        let credentials_file = args.get(
            "credentials",
            "c",
            String::from(".twitter_credentials.toml"),
        );

        let message = args.get_option("message", "m");

        let profile = args.get_option("profile", "p");

        Ok(Args {
            command,
            credentials_file,
            message,
            profile,
        })
    }
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Args <command: {:?}, credentials_file: {}, message: {}>",
            &self.command,
            &self.credentials_file,
            self.message.as_ref().unwrap_or(&"None".to_string())
        )
    }
}

fn command(args: &ArgParser) -> Command {
    let first_positional_arg_is_help = args.map.get("0") == Some(&String::from("help"));
    let requested_help_with_no_positional_arg = args.map.get("0").is_none()
        && (args.map.get("help").is_some() || args.map.get("h").is_some());
    if first_positional_arg_is_help || requested_help_with_no_positional_arg {
        return Command::Help;
    }

    match args.map.get("0") {
        Some(thing) => match thing.as_str() {
            "post" => Command::Tweet,
            "tweet" => Command::Tweet,
            "init" => Command::Init,
            "help" => Command::Help,
            "version" => Command::Version,
            "login" => Command::Login,
            _ => {
                println!("Unknown command: {}", thing);
                Command::Help
            }
        },
        None => {
            println!("No command specified");
            Command::Help
        }
    }
}
