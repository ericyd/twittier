use std::process;

mod args;
mod credentials;
mod error;
mod twitter;
mod commands;

use args::BaseArgs;

#[derive(Debug)]
enum Command {
    Help,
    Version,
    Tweet,
    Delete,
    Init,
}

fn main() {
    // Pattern lifted wholesale from ripgrep ¯\_(ツ)_/¯
    // https://github.com/BurntSushi/ripgrep/blob/e6cac8b119d0d50646b3ba1aaf53e648c779901a/crates/core/main.rs#L48-L74
    if let Err(err) = BaseArgs::parse().and_then(try_main) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main(args: BaseArgs) -> Result<(), error::TwitterError> {
    dbg!(&args);
    match command(&args) {
        Command::Tweet => commands::post(&args),
        Command::Delete => commands::delete(&args),
        Command::Version => {
            println!("Twitter CLI 🐤 v0.1.0");
            Ok(())
        }
        Command::Init => credentials::init(&args),
        Command::Help => print_help(),
    }
}

fn command(args: &BaseArgs) -> Command {
    if args.is_nth_argument_help(0) {
        return Command::Help;
    }

    match args.get_position::<String>(0) {
        Some(thing) => match thing.as_str() {
            "post" => Command::Tweet,
            "p" => Command::Tweet,
            "tweet" => Command::Tweet,
            "delete" => Command::Delete,
            "init" => Command::Init,
            "help" => Command::Help,
            "version" => Command::Version,
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
