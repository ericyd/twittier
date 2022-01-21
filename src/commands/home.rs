use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter;

const HELP: &str = "See your most recent tweets!\n
Usage: tw home [count] [OPTIONS]

Arguments
    count (default: 10):
        integer between 5 and 100.

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
    Read your last 10 tweets (default):
        tw home
    Read your last 20 tweets:
        tw home 20
    Read your last 5 tweets from your alt profile:
        tw home 5 -p alt1
";

struct Args {
    count: i32,
}

fn parse(args: &BaseArgs) -> Args {
    let count = match args.get_position::<String>(1) {
        Some(count) => count.parse::<i32>().unwrap(),
        None => 10,
    };
    Args { count }
}

fn help() -> Result<(), TwitterError> {
    println!("{}", HELP);
    Ok(())
}

pub fn execute(base_args: &BaseArgs) -> Result<(), TwitterError> {
    if base_args.is_requesting_help() {
        return help();
    }
    let args = parse(&base_args);
    if args.count < 5 || args.count > 100 {
        return Err(TwitterError::Invalid(
            "Count must be between 5 and 100".to_string(),
        ));
    }
    let credentials = credentials::get(base_args)?;
    base_args.debug(&credentials);

    let me = twitter::Client::new(&credentials, base_args).me()?;
    let home = twitter::Client::new(&credentials, base_args).home_v2(&me.id, args.count)?;

    for item in home {
        item.display();
    }

    Ok(())
}
