use std::process;

mod args;
use args::Args;

mod credentials;
use credentials::get_credentials;

mod tweet;
use tweet::tweet;

fn main() {
    // Pattern lifted wholesale from ripgrep ¯\_(ツ)_/¯
    // https://github.com/BurntSushi/ripgrep/blob/e6cac8b119d0d50646b3ba1aaf53e648c779901a/crates/core/main.rs#L48-L74
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main(args: Args) -> Result<(), String> {
    use args::Command::*;

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
            let credentials = get_credentials(&args);
            match credentials {
                Ok(credentials) => {
                    println!("{}", credentials.access_token);
                    println!("{}", credentials.access_token_secret);
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        Help => {
            println!("Help");
            Ok(())
        }
    }
}
