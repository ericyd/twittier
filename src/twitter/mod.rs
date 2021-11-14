use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode;
use super::credentials::Credentials;
use super::error::TwitterError;

#[derive(Deserialize, Serialize, Debug)]
pub struct TwitterCreateResponseData {
    pub id: String,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct TwitterDeleteResponseData {
    pub deleted: bool
}

#[derive(Deserialize, Debug)]
pub struct TwitterResponse<T> {
    data: T,
}

pub struct Twitter {
    credentials: Credentials,
    client: reqwest::blocking::Client,
}

struct OauthParams {
    oauth_consumer_key: String,
    oauth_nonce: String,
    oauth_timestamp: String,
    oauth_token: String,
}

impl Twitter {
    pub fn new(credentials: Credentials) -> Twitter {
        // Async is obviously the cooler way but I know nothing about Rust Futures and it felt out of scope for this stage. From the reqwest docs:
        // "For applications wishing to only make a few HTTP requests, the reqwest::blocking API may be more convenient."
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/index.html
        let client = reqwest::blocking::Client::new();
        Twitter { credentials, client }
    }

    // https://developer.twitter.com/en/docs/twitter-api/tweets/manage-tweets/api-reference/post-tweets
    pub fn post_v2(&self, message: &str) -> Result<TwitterCreateResponseData, TwitterError> {
        dbg!(format!("Posting message: {}", message));

        let base_url = "https://api.twitter.com/2/tweets";
        let authorization = self.build_authorization("POST", &base_url);

        let mut body = HashMap::new();
        body.insert("text", message);

        // returns Result<Response>
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/struct.Response.html
        let req = self.client
            .post(base_url)
            .header("Authorization", authorization)
            .json(&body);
        dbg!(&req);

        let res = req.send()?;
        dbg!(&res);

        // Possible to use match on the enum if desired
        // https://docs.rs/reqwest/0.11.6/reqwest/struct.StatusCode.html#impl-1
        if res.status().is_success() {
            let json: TwitterResponse<TwitterCreateResponseData> = res.json()?;
            dbg!(&json);
            Ok(json.data)
        } else if res.status().is_server_error() {
            Err(TwitterError::Api(format!("Server error: {}", &res.status())))
        } else if res.status().is_client_error() {
            Err(TwitterError::Api(format!("Client error: {}", &res.status())))
        } else {
            Err(TwitterError::Api(format!("Unknown error: {}", &res.status())))
        }
    }

    // https://developer.twitter.com/en/docs/twitter-api/tweets/manage-tweets/api-reference/delete-tweets-id
    pub fn delete_v2(&self, id: &str) -> Result<TwitterDeleteResponseData, TwitterError> {
        dbg!(format!("Deleting id: {}", id));

        let base_url = format!("https://api.twitter.com/2/tweets/{}", id);
        let authorization = self.build_authorization("DELETE", &base_url);

        // returns Result<Response>
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/struct.Response.html
        let req = self.client
            .delete(&base_url)
            .header("Authorization", authorization);
        dbg!(&req);

        let res = req.send()?;
        dbg!(&res);

        // Possible to use match on the enum if desired
        // https://docs.rs/reqwest/0.11.6/reqwest/struct.StatusCode.html#impl-1
        if res.status().is_success() {
            let json: TwitterResponse<TwitterDeleteResponseData> = res.json()?;
            dbg!(&json);
            Ok(json.data)
        } else if res.status().is_server_error() {
            Err(TwitterError::Api(format!("Server error: {}", &res.status())))
        } else if res.status().is_client_error() {
            Err(TwitterError::Api(format!("Client error: {}", &res.status())))
        } else {
            Err(TwitterError::Api(format!("Unknown error: {}", &res.status())))
        }
    }

    fn build_authorization(&self, method: &str, base_url: &str) -> String {
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
            method,
            encode(base_url),
            encode(&self.parameter_string(&params))
        );
        let hashed_request = self.hash(&self.signing_key(), &encoded_request);
        let oath_signature = base64::encode(&hashed_request);

        let authorization = self.authorization(&params, &oath_signature);
        dbg!(&authorization);
        authorization
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
        // This lib looks a little jank but it works and its tiny https://docs.rs/hmac-sha1/0.1.3/hmacsha1/
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
