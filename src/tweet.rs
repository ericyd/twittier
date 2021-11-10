use super::args::Args;
use super::credentials::get_credentials;

// https://developer.twitter.com/en/docs/twitter-api/v1/tweets/post-and-engage/api-reference/post-statuses-update
// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/authorizing-a-request
// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature
// https://stackoverflow.com/questions/54619582/hmac-sha1-in-rust
pub fn tweet(args: &Args) -> Result<(), String> {
    match get_credentials(args) {
        Ok(credentials) => {
            println!("{}", credentials.access_token);
            println!("{}", credentials.access_token_secret);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
