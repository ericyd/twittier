use std::process;

mod args;
mod commands;
mod credentials;
mod error;
mod twitter;

use args::BaseArgs;

#[derive(Debug)]
enum Command {
    Help,
    Version,
    Tweet,
    Delete,
    Feed,
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
    args.debug(&args);
    match command(&args) {
        Command::Tweet => commands::post(&args),
        Command::Delete => commands::delete(&args),
        Command::Feed => commands::feed(&args),
        Command::Version => {
            print_banner();
            println!("🐤 v1.0.0");
            Ok(())
        }
        Command::Init => {
            print_banner();
            commands::init(&args)
        }
        Command::Help => print_help(),
    }
}

fn command(args: &BaseArgs) -> Command {
    match args.get_position::<String>(0) {
        Some(thing) => match thing.as_str() {
            "post" => Command::Tweet,
            "p" => Command::Tweet,
            "tweet" => Command::Tweet,
            "delete" => Command::Delete,
            "feed" => Command::Feed,
            "init" => Command::Init,
            "help" => Command::Help,
            "version" => Command::Version,
            _ => {
                println!("Unknown command: {}", thing);
                Command::Help
            }
        },
        None => match args.get_flag("version", "v") {
            true => Command::Version,
            false => {
                if !args.is_requesting_help() {
                    println!("No command specified");
                }
                Command::Help
            }
        },
    }
}

fn print_help() -> Result<(), error::TwitterError> {
    print_banner();
    println!(
        "Usage: tw [command] [options]

Commands:
    post [message]
    tweet [message]
    delete [id]

For help with specific commands, run:
    tw [command] --help
"
    );

    Ok(())
}

// Thanks to WireMock for the idea 😛
// https://github.com/wiremock/wiremock/blob/a8f8f40999eafecea03a83a86ff5ac14daeab1a5/src/main/java/com/github/tomakehurst/wiremock/standalone/WireMockServerRunner.java#L36-L43
fn print_banner() {
    println!(
        " /██████████ /██       /██ /██ /██████████ /██████████ /██ /████████ /███████▙
|___  ██    | ██  /▟▙ | ██| ██|___  ██ __/|___  ██ __/| ██| ██_____/| ██____/▐█
    | ██    | ██ /▟██▙| ██| ██    | ██        | ██    | ██| ██      | ██    |▐█
    | ██    | ██/▟█▘ ▜▙ ██| ██    | ██        | ██    | ██| ████████| ████████▘
    | ██    | ████▘_  ▜███| ██    | ██        | ██    | ██| ██_____/| ██  ▜█▙
    | ██    | ███▘/ \\  ▜██| ██    | ██        | ██    | ██| ██      | ██ \\ ▜█▙
    | ██    | ██▘/   \\  ▜█| ██    | ██        | ██    | ██| ████████| ██  \\ ▜█▙
    |__/    |__/      \\__/|__/    |__/        |__/    |__/|________/|__/   \\__/"
    );
}
