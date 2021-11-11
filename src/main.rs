use std::process;

mod args;
use args::Args;

mod credentials;
use credentials::get_credentials;

mod tweet;
use tweet::tweet;

mod error;

mod twitter;

fn main() {
    // Pattern lifted wholesale from ripgrep ¯\_(ツ)_/¯
    // https://github.com/BurntSushi/ripgrep/blob/e6cac8b119d0d50646b3ba1aaf53e648c779901a/crates/core/main.rs#L48-L74
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main(args: Args) -> Result<(), error::TwitterError> {
    use args::Command::*;

    println!("{:?}", &args);
    match args.command {
        Tweet => {
            println!("Tweet");
            tweet(&args)
        }
        Version => {
            println!("Version");
            Ok(())
        }
        Login => {
            println!("Login");
            let credentials = get_credentials(&args)?;
            println!("{}", credentials.access_token);
            println!("{}", credentials.access_token_secret);
            Ok(())
        }
        Help => {
            println!("Help");
            Ok(())
        }
    }
}
