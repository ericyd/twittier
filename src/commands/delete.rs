use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter;

const HELP: &str = "Delete a tweet!\n
Usage: tw delete tweet_id [OPTIONS]

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
    Delete a single tweet:
        tw delete 12345666
    Delete a tweet from an alt account:
        tw delete 12345666 --profile alt1
";

struct Args {
    id: String,
}

fn parse(args: &BaseArgs) -> Result<Args, TwitterError> {
    match args.get_position::<String>(1) {
        // TODO: find latest tweet and delete
        // Some(id) if *id == "last" => {
        //     Client::new(credentials).post_v2(&message).unwrap()
        //     Ok(())
        // }
        Some(id) if id != "" => Ok(Args { id }),
        _ => Err(TwitterError::MissingArgument("id".to_string())),
    }
}

fn help() -> Result<(), TwitterError> {
    println!("{}", HELP);
    Ok(())
}

pub fn execute(base_args: &BaseArgs) -> Result<(), TwitterError> {
    if base_args.is_requesting_help() {
        return help();
    }
    let args = parse(&base_args)?;
    let credentials = credentials::get(base_args)?;
    base_args.debug(&credentials);

    let response = twitter::Client::new(credentials, base_args).delete_v2(&args.id)?;
    if response.deleted == true {
        println!("Deleted tweet id: {}", args.id);
        Ok(())
    } else {
        Err(TwitterError::Api(format!(
            "Error deleting tweet ID: {}",
            args.id
        )))
    }
}
