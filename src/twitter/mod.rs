use super::credentials::Credentials;
use std::collections::HashMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// pub mod twitter;

#[derive(Deserialize, Debug)]
struct TwitterResponse {

}

pub struct Twitter {
    credentials: Credentials,
}

impl Twitter {
    pub fn new(credentials: Credentials) -> Twitter {
        Twitter { credentials }
    }

    pub fn post(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Posting message: {}", message);
        
        // Async is obviously the cooler way but I know nothing about Rust Futures and it felt out of scope for this stage. From the reqwest docs:
        // "For applications wishing to only make a few HTTP requests, the reqwest::blocking API may be more convenient."
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/index.html
        let client = reqwest::blocking::Client::new();

        // returns Result<Response>
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/struct.Response.html
        let res = client.post("https://api.twitter.com/1.1/statuses/update.json?include_entities=true")
            .header("Authorization", "Oauth")
            .form(&[("status", message)])
            .send()?;

        println!("{:#?}", &res);

        // Possible to use match on the enum if desired
        // https://docs.rs/reqwest/0.11.6/reqwest/struct.StatusCode.html#impl-1
        if res.status().is_success() {
            println!("success!");
            let json: TwitterResponse = res.json()?;
            println!("{:?}", json);
        } else if res.status().is_server_error() {
            println!("server error!");
        } else if res.status().is_client_error() {
            println!("bad credentials");
        } else {
            println!("Something else happened. Status: {:?}", &res.status());
        }

        
        

        Ok(())

    }
}
