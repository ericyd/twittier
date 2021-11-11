use super::args::Args;
use super::credentials::get_credentials;
use super::error::TwitterError;

use super::twitter::Twitter;

// https://developer.twitter.com/en/docs/twitter-api/v1/tweets/post-and-engage/api-reference/post-statuses-update
// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/authorizing-a-request
// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature
// https://stackoverflow.com/questions/54619582/hmac-sha1-in-rust
pub fn tweet(args: &Args) -> Result<(), TwitterError> {
    let credentials = get_credentials(args)?;
    println!("{}", credentials.access_token);
    println!("{}", credentials.access_token_secret);

    match &args.message {
        Some(message) if message != "" => {
            Twitter::new(credentials).post(&message);
            Ok(())
        }
        // TODO: is to_string() necessary? Is there a way to use &str in MissingArgument perhaps?
        _ => Err(TwitterError::MissingArgument("message".to_string())),
    }
}
