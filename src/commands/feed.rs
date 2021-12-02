use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter;

const HELP: &str = "Read your feed!\n
Usage: tw feed [count] [OPTIONS]

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
    Read 10 tweets from your feed (default):
        tw feed
    Read 20 tweets from your feed:
        tw feed 20
    Read 1 tweet from your alt feed:
        tw feed 1 -p alt1
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
    let credentials = credentials::get(base_args)?;
    base_args.debug(&credentials);

    let feed = twitter::Client::new(credentials, base_args).feed(args.count)?;

    for item in feed {
        item.display();
    }

    Ok(())
}
