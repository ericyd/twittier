use super::args::Args;
use super::credentials;
use super::error::TwitterError;
use super::twitter::Twitter;

pub fn post(args: &Args) -> Result<(), TwitterError> {
    let credentials = credentials::get(args)?;
    dbg!(&credentials);

    match &args.message {
        Some(message) if message != "" => {
            if let Err(err) = Twitter::new(credentials).post_v2(&message) {
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
