use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter;

const HELP: &str = "Get some details about yourself!\n
Usage: tw me [OPTIONS]

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
    Get your user summary:
        tw me
    Get user summary from your alt home:
        tw me -p alt1
";

fn help() -> Result<(), TwitterError> {
    println!("{}", HELP);
    Ok(())
}

pub fn execute(base_args: &BaseArgs) -> Result<(), TwitterError> {
    if base_args.is_requesting_help() {
        return help();
    }
    let credentials = credentials::get(base_args)?;
    base_args.debug(&credentials);

    let me = twitter::Client::new(&credentials, base_args).me()?;

    me.display();

    Ok(())
}
