use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter;

const HELP: &str = "Post a tweet!\n
Usage: tw post message [...replies] [OPTIONS]

Including replies will post a thread

Options:
    -r, --reply-id <id>
        The ID of the tweet to reply to.
    -p, --profile <name>
        The name of the profile to use.
        Must correspond to an entry in your credentials file (~/.twitter_credentials.toml by default).
    -c, --credentials <name>
        The file name or path to use for the credentials file.
        Default: ~/.twitter_credentials.toml
    --debug
        Print debug messages.

Examples:
    Post a single tweet:
        tw post \"I'll tell you what's up\"
    Reply to an somebody's tweet:
        tw post \"Emojis work too 🤩\" --reply-id 12345
    Post a thread:
        tw post \"I took out my wool sweater today and it made me want to THREAD\" \"#sweaterweather\" \"#unnecessarythreading\"
    Post with an alt account:
        tw post \"Hey y'all @ericydauenhauer is real\" --profile alt1
";

struct Args {
    messages: Vec<String>,
    in_reply_to_tweet_id: Option<String>,
}

fn parse(args: &BaseArgs) -> Result<Args, TwitterError> {
    let messages = args.positional[1..].to_vec();
    if messages.len() == 0 {
        return Err(TwitterError::MissingArgument("message".to_string()));
    }
    let in_reply_to_tweet_id = args.get_option("reply-id", "r");
    Ok(Args {
        messages,
        in_reply_to_tweet_id,
    })
}

fn help() -> Result<(), TwitterError> {
    println!("{}", HELP);
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
    let handle = String::from(&credentials.handle);

    let client = twitter::Client::new(credentials, base_args);
    let mut response = client.post_v2(&args.messages[0], &args.in_reply_to_tweet_id)?;
    let first_tweet_id = String::from(&response.id);
    println!(
        "Posted tweet {} - https://twitter.com/{}/status/{}",
        response.id, handle, response.id
    );

    for message in args.messages[1..].iter() {
        response = client.post_v2(message, &Some(response.id))?;
        println!(
            "Posted tweet {} - https://twitter.com/{}/status/{}",
            response.id, handle, response.id
        );
    }

    if args.messages.len() > 1 {
        println!(
            "Thread posted {} - https://twitter.com/{}/status/{}",
            first_tweet_id, handle, first_tweet_id
        );
    }

    Ok(())
}
