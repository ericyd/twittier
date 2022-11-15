// Why write a custom argument parser?
// Let me answer your question with a question:
// Why is a muffin delicious?
// Why is coal dirty?
// Why is a cat hungry?

use super::error::TwitterError;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct BaseArgs {
    named: HashMap<String, String>,
    pub positional: Vec<String>,
    pub flags: HashMap<String, bool>,
    is_debug: bool,
}

// simple argument collector
impl BaseArgs {
    pub fn parse() -> Result<Self, TwitterError> {
        let args: Vec<String> = std::env::args().collect();
        // Start at 1 to omit the executable name
        let mut i = 1;
        let mut named = HashMap::new();
        let mut flags = HashMap::new();
        let mut positional = Vec::new();

        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("-") {
                let key = if arg.starts_with("--") {
                    arg.trim_start_matches("--").to_string()
                } else {
                    arg.trim_start_matches("-").to_string()
                };

                // If this argument is last in the list, or the next one is also a named arg, do not use next arg as value
                // ... it might be smarter to just have a list of accepted flags, i.e. debug, help, version
                let is_flag_arg = i + 1 >= args.len() || args[i + 1].starts_with("-");
                if is_flag_arg {
                    flags.insert(key, true);
                    i += 1;
                } else {
                    named.insert(key, args[i + 1].to_string());
                    i += 2;
                };
            } else {
                positional.push(arg.to_string());
                i += 1;
            }
        }

        // I'm absolutely SURE this is bad practice but I don't care
        let is_debug = flags.get("debug").unwrap_or(&false).clone();

        Ok(Self {
            named,
            positional,
            flags,
            is_debug,
        })
    }

    pub fn get<T: FromStr>(&self, long_name: &str, short_name: &str, default: T) -> T {
        match self.named.get(long_name) {
            Some(thing) => match thing.parse::<T>() {
                Ok(val) => val,
                Err(_err) => default,
            },
            None => match self.named.get(short_name) {
                Some(thing) => match thing.parse::<T>() {
                    Ok(val) => val,
                    Err(_err) => default,
                },
                None => default,
            },
        }
    }

    pub fn get_option<T: FromStr>(&self, long_name: &str, short_name: &str) -> Option<T> {
        match self.named.get(long_name) {
            Some(thing) => match thing.parse::<T>() {
                Ok(val) => Some(val),
                Err(_err) => None,
            },
            None => match self.named.get(short_name) {
                Some(thing) => match thing.parse::<T>() {
                    Ok(val) => Some(val),
                    Err(_err) => None,
                },
                None => None,
            },
        }
    }

    pub fn get_position<T: FromStr>(&self, position: usize) -> Option<T> {
        match self.positional.get(position) {
            Some(thing) => match thing.parse::<T>() {
                Ok(val) => Some(val),
                Err(_err) => None,
            },
            None => None,
        }
    }

    pub fn get_flag(&self, long_name: &str, short_name: &str) -> bool {
        match self.flags.get(long_name) {
            Some(result) => *result,
            None => match self.flags.get(short_name) {
                Some(result) => *result,
                None => false,
            },
        }
    }

    pub fn is_requesting_help(&self) -> bool {
        let last_positional_arg_is_help = match self.positional.last() {
            Some(arg) => arg == "help" || arg == "h",
            None => false,
        };
        let is_help_flag_set = self.flags.get("help").is_some()
            || self.flags.get("h").is_some()
            || self.named.get("help").is_some()
            || self.named.get("h").is_some();
        last_positional_arg_is_help || is_help_flag_set
    }

    pub fn debug<T: std::fmt::Debug>(&self, thing: &T) {
        if self.is_debug {
            dbg!(thing);
        }
    }
}

impl Display for BaseArgs {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let named = self
            .named
            .iter()
            .fold(Vec::<String>::new(), |vec, (k, v)| {
                [&vec[..], &vec![format!("{}: {}", k, v)]].concat()
            })
            .join(", ");
        let positional = self
            .positional
            .iter()
            .fold(Vec::<String>::new(), |vec, string| {
                [&vec[..], &vec![format!("{}", string)]].concat()
            })
            .join(", ");
        write!(f, "BaseArgs <[{}] {}>", positional, named)
    }
}
