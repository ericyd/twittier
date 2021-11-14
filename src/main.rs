use std::process;

mod args;
use args::Args;
mod credentials;
mod error;
mod tweet;
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

    dbg!(&args);
    match args.command {
        Tweet => tweet::post(&args),
        Version => Ok(()),
        Login => {
            let credentials = credentials::get(&args)?;
            dbg!(credentials);
            Ok(())
        }
        Init => credentials::init(&args),
        Help => Ok(()),
    }
}
