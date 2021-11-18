use super::super::args::BaseArgs;
use super::super::credentials::Credentials;
use super::super::error::TwitterError;
use super::TwitterCreateResponseData;
use super::TwitterDeleteResponseData;
use super::TwitterFeed;
use super::TwitterResponse;
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode;
use serde_json::json;

type ParameterList<'a> = &'a [(&'a str, String)];

pub struct Client<'c> {
    credentials: Credentials,
    client: reqwest::blocking::Client,
    args: &'c BaseArgs,
}

impl<'c> Client<'c> {
    pub fn new(credentials: Credentials, args: &'c BaseArgs) -> Self {
        // Async is obviously the cooler way but I know nothing about Rust Futures and it felt out of scope for this stage. From the reqwest docs:
        // "For applications wishing to only make a few HTTP requests, the reqwest::blocking API may be more convenient."
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/index.html
        let client = reqwest::blocking::Client::new();
        Self {
            credentials,
            client,
            args,
        }
    }

    // https://developer.twitter.com/en/docs/twitter-api/tweets/manage-tweets/api-reference/post-tweets
    pub fn post_v2(&self, message: &str, in_reply_to_tweet_id: &Option<String>) -> Result<TwitterCreateResponseData, TwitterError> {
        self.args.debug(&format!("Posting message: {}", message));

        let base_url = "https://api.twitter.com/2/tweets";
        let authorization = self.build_authorization("POST", &base_url, None);

        let body = match in_reply_to_tweet_id {
            Some(id) => json!({
                "text": message,
                "reply": {
                    "in_reply_to_tweet_id": id,
                }
            }),
            None => json!({
                "text": message,
            }),
        };
        self.args.debug(&body);

        // returns Result<Response>
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/struct.Response.html
        let req = self
            .client
            .post(base_url)
            .header("Authorization", authorization)
            .json(&body);
        self.args.debug(&req);

        let res = req.send()?;
        self.args.debug(&res);

        // Possible to use match on the enum if desired
        // https://docs.rs/reqwest/0.11.6/reqwest/struct.StatusCode.html#impl-1
        if res.status().is_success() {
            let json: TwitterResponse<TwitterCreateResponseData> = res.json()?;
            self.args.debug(&json);
            Ok(json.data)
        } else {
            Err(self.error(res))
        }
    }

    // https://developer.twitter.com/en/docs/twitter-api/tweets/manage-tweets/api-reference/delete-tweets-id
    pub fn delete_v2(&self, id: &str) -> Result<TwitterDeleteResponseData, TwitterError> {
        self.args.debug(&format!("Deleting id: {}", id));

        let base_url = format!("https://api.twitter.com/2/tweets/{}", id);
        let authorization = self.build_authorization("DELETE", &base_url, None);

        // returns Result<Response>
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/struct.Response.html
        let req = self
            .client
            .delete(&base_url)
            .header("Authorization", authorization);
        self.args.debug(&req);

        let res = req.send()?;
        self.args.debug(&res);

        if res.status().is_success() {
            let json: TwitterResponse<TwitterDeleteResponseData> = res.json()?;
            self.args.debug(&json);
            Ok(json.data)
        } else {
            Err(self.error(res))
        }
    }

    // https://developer.twitter.com/en/docs/twitter-api/v1/tweets/timelines/api-reference/get-statuses-home_timeline
    // https://developer.twitter.com/en/docs/twitter-api/v1/tweets/timelines/guides/working-with-timelines
    pub fn feed(&self, count: i32) -> Result<TwitterFeed, TwitterError> {
        self.args
            .debug(&format!("Fetching feed with count: {}", count));

        // UGH... to have query params, I will need to send them in too build_authorization separately from the base_url, then re-encode it here...
        // might need to use a custom struct for feed args, too keep this from getting too wild.
        let base_url = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let params = &[("count", count.to_string())];
        let authorization = self.build_authorization("GET", base_url, Some(params));

        // returns Result<Response>
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/struct.Response.html
        let req = self
            .client
            .get(&format!("{}?count={}", base_url, count))
            .header("Authorization", authorization);
        self.args.debug(&req);

        let res = req.send()?;
        self.args.debug(&res);

        if res.status().is_success() {
            // self.args.debug(res.text()?);
            // Ok(vec![])
            let json: TwitterFeed = res.json()?;
            Ok(json)
        } else {
            Err(self.error(res))
        }
    }

    // Twitter takes their authorization seriously
    // https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature
    fn build_authorization(
        &self,
        method: &str,
        base_url: &str,
        request_params: Option<ParameterList>,
    ) -> String {
        let parameters = &[
            ("oauth_consumer_key", self.credentials.api_key.to_owned()),
            ("oauth_nonce", self.nonce()),
            ("oauth_signature_method", "HMAC-SHA1".to_string()),
            ("oauth_timestamp", self.timestamp()),
            ("oauth_token", self.credentials.access_token.to_owned()),
            ("oauth_version", "1.0".to_string()),
        ];
        let encoded_request = format!(
            "{}&{}&{}",
            method,
            encode(base_url),
            encode(&self.parameter_string(parameters, request_params, "&", false))
        );
        let hashed_request = self.hash(&self.signing_key(), &encoded_request);
        let oath_signature = base64::encode(&hashed_request);

        let authorization = self.authorization_header(parameters, &oath_signature);
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
    fn authorization_header(&self, params: ParameterList, signature: &str) -> String {
        format!(
            "Oauth {}",
            self.parameter_string(
                params,
                Some(&[("oauth_signature", signature.to_string())]),
                ",",
                true
            )
        )
    }

    /// Takes a list of parameters and returns a string of them all joined by the given separator.
    /// All parameters are URL encoded.
    ///
    /// Example
    ///    self.parameter_string(&[("test", "data".to_string()), ("is", "fun".to_string())], "&", false)
    ///    => "test=data&is=fun"
    ///
    ///    self.parameter_string(&[("test", "data".to_string()), ("is", "fun".to_string())], ",", true)
    ///    => "test=\"data\",is=\"fun\""
    fn parameter_string(
        &self,
        oauth_params: ParameterList,
        request_params: Option<ParameterList>,
        join_str: &str,
        wrap_in_quotes: bool,
    ) -> String {
        let mut params = match request_params {
            Some(params) => [params, oauth_params].concat(),
            None => [oauth_params].concat(),
        };
        params.sort_by(|a, b| a.0.cmp(b.0));
        params
            .iter()
            .map(|(key, value)| {
                if wrap_in_quotes {
                    format!("{}=\"{}\"", encode(key), encode(value))
                } else {
                    format!("{}={}", encode(key), encode(value))
                }
            })
            .collect::<Vec<String>>()
            .join(join_str)
    }

    // Possible to use match on the enum if desired
    // https://docs.rs/reqwest/0.11.6/reqwest/struct.StatusCode.html#impl-1
    fn error(&self, res: reqwest::blocking::Response) -> TwitterError {
        if res.status().is_server_error() {
            TwitterError::Api(format!("Server error: {}", &res.status()))
        } else if res.status().is_client_error() {
            TwitterError::Api(format!("Client error: {}", &res.status()))
        } else {
            TwitterError::Api(format!("Unknown error: {}", &res.status()))
        }
    }
}
