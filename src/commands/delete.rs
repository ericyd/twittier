use super::super::args::BaseArgs;
use super::super::credentials;
use super::super::error::TwitterError;
use super::super::twitter;

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
    println!("TODO: document");
    println!("delete");
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
        Ok(())
    } else {
        Err(TwitterError::Api(format!(
            "Error deleting tweet ID: {}",
            args.id
        )))
    }
}
