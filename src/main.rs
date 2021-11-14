use std::process;

mod args;
use args::Args;
mod credentials;
mod error;
mod tweet;
mod twitter;

#[derive(Debug)]
enum Command {
    Help,
    Version,
    Tweet,
    Delete,
    Login,
    Init,
}

fn main() {
    // Pattern lifted wholesale from ripgrep ¯\_(ツ)_/¯
    // https://github.com/BurntSushi/ripgrep/blob/e6cac8b119d0d50646b3ba1aaf53e648c779901a/crates/core/main.rs#L48-L74
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main(args: Args) -> Result<(), error::TwitterError> {
    dbg!(&args);
    match command(&args) {
        Command::Tweet => tweet::post(&args),
        Command::Delete => tweet::delete(&args),
        Command::Version => {
            println!("Twitter CLI 🐤 v0.1.0");
            Ok(())
        }
        Command::Login => {
            let credentials = credentials::get(&args)?;
            dbg!(credentials);
            Ok(())
        }
        Command::Init => credentials::init(&args),
        Command::Help => print_help(),
    }
}

fn command(args: &Args) -> Command {
    if args.is_nth_argument_help(0) {
        return Command::Help;
    }

    match args.map.get("0") {
        Some(thing) => match thing.as_str() {
            "post" => Command::Tweet,
            "p" => Command::Tweet,
            "tweet" => Command::Tweet,
            "delete" => Command::Delete,
            "init" => Command::Init,
            "help" => Command::Help,
            "version" => Command::Version,
            "login" => Command::Login,
            _ => {
                println!("Unknown command: {}", thing);
                Command::Help
            }
        },
        None => {
            println!("No command specified");
            Command::Help
        }
    }
}

fn print_help() -> Result<(), error::TwitterError> {
    println!(
        "Usage: twitter [command] [options]

Commands:
    post [message]
    tweet [message]
    delete [id]
"
    );

    Ok(())
}
