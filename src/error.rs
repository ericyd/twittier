use std::io::Error as IoError;
use std::{error::Error, fmt};
use toml::de::Error as TomlError;

// Allow the use of "{:?}" format specifier
#[derive(Debug)]
pub enum TwitterError {
    Io(IoError),
    Parse(TomlError),
    MissingArgument(String),
}

// Allow the use of "{}" format specifier
impl fmt::Display for TwitterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TwitterError::Io(ref cause) => write!(f, "I/O error: {}", cause),
            TwitterError::Parse(ref cause) => write!(f, "Error parsing file: {}", cause),
            TwitterError::MissingArgument(ref arg) => write!(f, "Missing argument: {}", arg),
        }
    }
}

// Allow this type to be treated like an error
impl Error for TwitterError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            TwitterError::Io(ref cause) => Some(cause),
            TwitterError::Parse(ref cause) => Some(cause),
            TwitterError::MissingArgument(_) => None,
        }
    }
}

// Support converting system errors into our custom error.
// This trait is used in `try!`.
impl From<IoError> for TwitterError {
    fn from(cause: IoError) -> TwitterError {
        TwitterError::Io(cause)
    }
}

impl From<TomlError> for TwitterError {
    fn from(cause: TomlError) -> TwitterError {
        TwitterError::Parse(cause)
    }
}

impl From<String> for TwitterError {
    fn from(message: String) -> TwitterError {
        TwitterError::MissingArgument(message)
    }
}
