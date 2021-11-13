use super::credentials::Credentials;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode;

// TODO: add more fields: https://developer.twitter.com/en/docs/twitter-api/v1/tweets/post-and-engage/api-reference/post-statuses-update#example-response

/*
{
  "data": {
    "id": "1459618689611935745",
    "text": "testing"
  }
}
*/
#[derive(Deserialize, Debug)]
struct TwitterResponseData {
    id: String,
    text: String,
}

#[derive(Deserialize, Debug)]
struct TwitterResponse {
    data: TwitterResponseData,
}

pub struct Twitter {
    credentials: Credentials,
}

struct OauthParams {
    oauth_consumer_key: String,
    oauth_nonce: String,
    oauth_timestamp: String,
    oauth_token: String,
}

impl Twitter {
    pub fn new(credentials: Credentials) -> Twitter {
        Twitter { credentials }
    }

    pub fn post_v2(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        dbg!(format!("Posting message: {}", message));

        // Async is obviously the cooler way but I know nothing about Rust Futures and it felt out of scope for this stage. From the reqwest docs:
        // "For applications wishing to only make a few HTTP requests, the reqwest::blocking API may be more convenient."
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/index.html
        let client = reqwest::blocking::Client::new();

        // omitting ?include_entities=true for now to ensure I can get signing to work before adding extra params
        // let base_url = "http://localhost:8080/2/tweets";
        let base_url = "https://api.twitter.com/2/tweets";

        let params = OauthParams {
            oauth_consumer_key: self.credentials.api_key.to_owned(),
            oauth_nonce: self.nonce(),
            oauth_timestamp: self.timestamp(),
            oauth_token: self.credentials.access_token.to_owned(),
        };

        // Twitter takes their authorization seriously
        // TODO: I think there are cleaner ways to abstract this
        // https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature
        let encoded_request = format!(
            "{}&{}&{}",
            "POST",
            encode(&base_url),
            encode(&self.parameter_string(&params))
        );
        let hashed_request = self.hash(&self.signing_key(), &encoded_request);
        let oath_signature = base64::encode(&hashed_request);

        let authorization = self.authorization(&params, &oath_signature);
        dbg!(&authorization);

        let mut body = HashMap::new();
        body.insert("text", message);

        // returns Result<Response>
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/struct.Response.html
        let req = client
            .post(base_url)
            .header("Authorization", authorization)
            .json(&body);
        dbg!(&req);

        let res = req.send()?;
        dbg!(&res);

        // Possible to use match on the enum if desired
        // https://docs.rs/reqwest/0.11.6/reqwest/struct.StatusCode.html#impl-1
        if res.status().is_success() {
            println!("success!");

            let json: TwitterResponse = res.json()?;
            dbg!(json);
            /*
            json = TwitterResponse {
                data: TwitterResponseData {
                    id: "1459623297629532163",
                    text: "if you happen to be looking at my feed in real time then this is a test",
                },
            }
            */
        } else if res.status().is_server_error() {
            println!("server error!");
        } else if res.status().is_client_error() {
            println!("bad credentials");
        } else {
            println!("Something else happened. Status: {:?}", &res.status());
        }

        Ok(())
    }

    // This is obviously a very fake nonce but it should be fine I think
    fn nonce(&self) -> String {
        let timestamp = self.timestamp();
        // is there really no easier way to write a byte slice to hex string?
        self.hash(&timestamp, &timestamp)
            .iter()
            .fold(String::new(), |mut string, &byte| {
                string.push_str(&format!("{:02X}", byte));
                string
            })
    }

    fn timestamp(&self) -> String {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs().to_string(),
            // truly no idea what would cause this
            Err(err) => panic!("{}", err),
        }
    }

    fn hash(&self, key: &str, input: &str) -> [u8; 20] {
        /*
        This lib looks a little jank but it works and its tiny https://docs.rs/hmac-sha1/0.1.3/hmacsha1/
        */
        hmacsha1::hmac_sha1(key.as_bytes(), input.as_bytes())
    }

    fn signing_key(&self) -> String {
        format!(
            "{}&{}",
            encode(&self.credentials.api_key_secret),
            encode(&self.credentials.access_token_secret),
        )
    }

    // https://developer.twitter.com/en/docs/authentication/oauth-1-0a/authorizing-a-request
    fn authorization(&self, params: &OauthParams, signature: &str) -> String {
        format!(
            concat!(
                "OAuth ",
                "oauth_consumer_key=\"{}\",",
                "oauth_token=\"{}\",",
                "oauth_signature_method=\"HMAC-SHA1\",",
                "oauth_timestamp=\"{}\",",
                "oauth_nonce=\"{}\",",
                "oauth_version=\"1.0\",",
                "oauth_signature=\"{}\"",
            ),
            encode(&params.oauth_consumer_key),
            encode(&params.oauth_token),
            encode(&params.oauth_timestamp),
            encode(&params.oauth_nonce),
            encode(&signature),
        )
    }

    fn parameter_string(&self, params: &OauthParams) -> String {
        // I'm confident there is a better way to do this with Serialize or Deserialize but this works for now
        format!(
            concat!(
                "oauth_consumer_key={}&",
                "oauth_nonce={}&",
                "oauth_signature_method=HMAC-SHA1&",
                "oauth_timestamp={}&",
                "oauth_token={}&",
                "oauth_version=1.0",
            ),
            encode(&params.oauth_consumer_key),
            encode(&params.oauth_nonce),
            encode(&params.oauth_timestamp),
            encode(&params.oauth_token),
        )
    }
}
