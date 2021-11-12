use super::args::Args;
use super::credentials::get_credentials;
use super::error::TwitterError;

use super::twitter::Twitter;

pub fn tweet(args: &Args) -> Result<(), TwitterError> {
    let credentials = get_credentials(args)?;
    println!("{}", credentials.api_key);
    println!("{}", credentials.access_token);
    println!("{}", credentials.access_token_secret);

    match &args.message {
        Some(message) if message != "" => {
            if let Err(err) = Twitter::new(credentials).post(&message) {
                eprintln!("{}", err);
                return Err(TwitterError::MissingArgument(
                    "who the fuck knows".to_string(),
                ));
            }
            Ok(())
        }
        // TODO: is to_string() necessary? Is there a way to use &str in MissingArgument perhaps?
        _ => Err(TwitterError::MissingArgument("message".to_string())),
    }
}
