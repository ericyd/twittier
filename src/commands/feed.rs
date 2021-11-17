use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter;

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
    println!("TODO: document");
    Ok(())
}

pub fn execute(base_args: &BaseArgs) -> Result<(), TwitterError> {
    if base_args.is_nth_argument_help(1) {
        return help();
    }
    let args = parse(&base_args);
    let credentials = credentials::get(base_args)?;
    dbg!(&credentials);

    let feed = twitter::Client::new(credentials).feed(args.count)?;

    for item in feed {
        item.display();
    }

    Ok(())
}
