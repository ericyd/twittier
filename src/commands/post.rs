use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter;

struct Args {
    message: String,
    in_reply_to_tweet_id: Option<String>
}

fn parse(args: &BaseArgs) -> Result<Args, TwitterError> {
    let message = match args.get_position::<String>(1) {
        Some(message) if &message != "" => message,
        _ => return Err(TwitterError::MissingArgument("message".to_string())),
    };
    let in_reply_to_tweet_id = args.get_option("reply-id", "r");
    Ok(Args { message, in_reply_to_tweet_id })
}

fn help() -> Result<(), TwitterError> {
    println!("TODO: document");
    println!("post");
    Ok(())
}

/*
Future plans: Append tweets to a file.

use super::super::twitter::TwitterCreateResponseData;
struct TwitterHistoryFile {
    tweets: Vec<TwitterCreateResponseData>,
}

// Not a good look to have this copoy/pasted from credentials.rs ¯\_(ツ)_/¯
fn home_dir() -> PathBuf {
    home::home_dir().expect("Cannot get your home directory!")
}

fn create_or_open_history() -> Result<File, std::io::Error> {
    let mut path = PathBuf::from(home_dir());
    path.push(".twitter_history.toml");
    path = fs::canonicalize(&path)?;
    OpenOptions::new().write(true).create(true).open(path)
}

fn append_response_to_history(response: TwitterCreateResponseData) -> Result<(), TwitterError> {
    let history = create_or_open_history()?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // match contents {

    //     let contents = toml::to_string(HistoryFile { tweets: vec![response] })?;
    //     fs::write(path, contents)?;
    // }

    Ok(())
}
*/

pub fn execute(base_args: &BaseArgs) -> Result<(), TwitterError> {
    if base_args.is_requesting_help() {
        return help();
    }
    let args = parse(&base_args)?;
    let credentials = credentials::get(base_args)?;
    base_args.debug(&credentials);

    let response = twitter::Client::new(credentials, base_args).post_v2(&args.message, &args.in_reply_to_tweet_id)?;
    println!("Posted tweet id: {}", response.id);
    Ok(())
}
