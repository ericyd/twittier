// AGAINST ALL ODDS, THIS WORKS.....
// Why though?

// ripgrep has a bunch of custom logic, I feel like this might be required
// https://github.com/BurntSushi/ripgrep/blob/af54069c51cc3656c9c343a7fb3c9360cfddf505/crates/core/args.rs#L228-L250

use super::error::TwitterError;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct BaseArgs(HashMap<String, String>);

// simple argument collector
impl BaseArgs {
    pub fn parse() -> Result<Self, TwitterError> {
        let args: Vec<String> = std::env::args().collect();
        // Start at 1 to omit the executable name
        let mut i = 1;
        let mut position = 0;
        let mut map = HashMap::new();

        // TODO: handle flag arguments
        //      - if the next argument is a flag, or if it's the last argument, then assume it's a flag and set value to true
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

        Ok(Self(map))
    }

    pub fn get<T: FromStr>(&self, long_name: &str, short_name: &str, default: T) -> T {
        match self.0.get(long_name) {
            Some(thing) => match thing.parse::<T>() {
                Ok(val) => val,
                Err(_err) => default,
            },
            None => match self.0.get(short_name) {
                Some(thing) => match thing.parse::<T>() {
                    Ok(val) => val,
                    Err(_err) => default,
                },
                None => default,
            },
        }
    }

    pub fn get_option<T: FromStr>(&self, long_name: &str, short_name: &str) -> Option<T> {
        match self.0.get(long_name) {
            Some(thing) => match thing.parse::<T>() {
                Ok(val) => Some(val),
                Err(_err) => None,
            },
            None => match self.0.get(short_name) {
                Some(thing) => match thing.parse::<T>() {
                    Ok(val) => Some(val),
                    Err(_err) => None,
                },
                None => None,
            },
        }
    }

    pub fn get_position<T: FromStr>(&self, position: i32) -> Option<T> {
        match self.0.get(&position.to_string()) {
            Some(thing) => match thing.parse::<T>() {
                Ok(val) => Some(val),
                Err(_err) => None,
            },
            None => None,
        }
    }

    pub fn is_nth_argument_help(&self, n: i32) -> bool {
        let first_positional_arg_is_help =
            self.0.get(&n.to_string()) == Some(&String::from("help"));
        let requested_help_with_no_positional_arg = self.0.get(&n.to_string()).is_none()
            && (self.0.get("help").is_some() || self.0.get("h").is_some());
        first_positional_arg_is_help || requested_help_with_no_positional_arg
    }
}

impl Display for BaseArgs {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let key_values = self
            .0
            .iter()
            .fold(Vec::<String>::new(), |vec, (k, v)| {
                [&vec[..], &vec![format!("{}: {}", k, v)]].concat()
            })
            .join(", ");
        write!(f, "BaseArgs <{}>", key_values)
    }
}
