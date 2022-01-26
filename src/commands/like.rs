use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter;

const HELP: &str = "Like (or unlike) a tweet!\n
Usage: tw like tweet_id [OPTIONS]
       tw unlike tweet_id [OPTIONS]

Options:
    -p, --profile <name>
        The name of the profile to use.
        Must correspond to an entry in your credentials file (~/.twitter_credentials.toml by default).
    -c, --credentials <name>
        The file name or path to use for the credentials file.
        Default: ~/.twitter_credentials.toml
    --debug
        Print debug messages.

Examples:
    Like a single tweet:
        tw like 12345666
    Like a tweet from an alt account:
        tw like 12345666 --profile alt1
    Unlike a single tweet:
        tw unlike 12345666
";

struct Args {
    id: String,
}

fn parse(args: &BaseArgs) -> Result<Args, TwitterError> {
    match args.get_position::<String>(1) {
        Some(id) if id != "" => Ok(Args { id }),
        _ => Err(TwitterError::MissingArgument("id".to_string())),
    }
}

fn help() -> Result<(), TwitterError> {
    println!("{}", HELP);
    Ok(())
}

enum Action {
    Like,
    Unlike,
}

fn execute(base_args: &BaseArgs, like_or_unlike: Action) -> Result<(), TwitterError> {
    if base_args.is_requesting_help() {
        return help();
    }
    let args = parse(&base_args)?;
    let credentials = credentials::get(base_args)?;
    base_args.debug(&credentials);

    let me = twitter::Client::new(&credentials, base_args).me()?;
    let response = match like_or_unlike {
        Action::Like => twitter::Client::new(&credentials, base_args).like_v2(&me.id, &args.id)?,
        Action::Unlike => {
            twitter::Client::new(&credentials, base_args).unlike_v2(&me.id, &args.id)?
        }
    };
    if response.liked {
        println!("Liked tweet id: {}", args.id);
    } else {
        println!("Unliked tweet id: {}", args.id);
    }
    Ok(())
}

pub fn execute_like(base_args: &BaseArgs) -> Result<(), TwitterError> {
    execute(base_args, Action::Like)
}

pub fn execute_unlike(base_args: &BaseArgs) -> Result<(), TwitterError> {
    execute(base_args, Action::Unlike)
}
