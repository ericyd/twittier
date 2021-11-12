use super::credentials::Credentials;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode;

// TODO: add more fields: https://developer.twitter.com/en/docs/twitter-api/v1/tweets/post-and-engage/api-reference/post-statuses-update#example-response
#[derive(Deserialize, Debug)]
struct TwitterResponse {
    id: i64,
}

pub struct Twitter {
    credentials: Credentials,
}

struct OauthParams {
    base_url: String,
    oauth_consumer_key: String,
    oauth_nonce: String,
    oauth_signature_method: String,
    oauth_timestamp: String,
    oauth_token: String,
    oauth_version: String,
}

impl Twitter {
    pub fn new(credentials: Credentials) -> Twitter {
        Twitter { credentials }
    }

    // https://developer.twitter.com/en/docs/twitter-api/v1/tweets/post-and-engage/api-reference/post-statuses-update
    pub fn post(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        dbg!(format!("Posting message: {}", message));

        // Async is obviously the cooler way but I know nothing about Rust Futures and it felt out of scope for this stage. From the reqwest docs:
        // "For applications wishing to only make a few HTTP requests, the reqwest::blocking API may be more convenient."
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/index.html
        let client = reqwest::blocking::Client::new();

        // omitting ?include_entities=true for now to ensure I can get signing to work before adding extra params
        let base_url = "https://api.twitter.com/1.1/statuses/update.json";
        let authorization = self.build_auth_header(base_url, message);

        dbg!(&authorization);

        // returns Result<Response>
        // https://docs.rs/reqwest/0.11.6/reqwest/blocking/struct.Response.html
        let req = client
        .post(base_url)
        .header("Authorization", authorization)
        .form(&[("status", encode(message))]);
        dbg!(&req);

        let res = req.send()?;

        dbg!(&res);

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

    // https://developer.twitter.com/en/docs/authentication/oauth-1-0a/authorizing-a-request
    fn build_auth_header(&self, base_url: &str, message: &str) -> String {
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_millis().to_string(),
            // truly no idea what would cause this
            Err(err) => panic!("{}", err),
        };

        use crypto::digest::Digest;
        use crypto::whirlpool::Whirlpool;

        let mut hasher = Whirlpool::new();
        hasher.input_str(&timestamp);
        let nonce = hasher.result_str();

        let params = OauthParams {
            base_url: base_url.to_string(),
            oauth_consumer_key: self.credentials.api_key.to_owned(),
            oauth_nonce: self.credentials.api_key.to_owned(), //nonce,
            oauth_signature_method: "HMAC-SHA1".to_string(),
            oauth_timestamp: "1636753845094".to_string(), //timestamp,
            oauth_token: self.credentials.access_token.to_owned(),
            oauth_version: "1.0".to_string(),
        };

        let signature = self.build_signature(&params, message);

        format!(
            concat!(
                "OAuth ",
                "oauth_consumer_key=\"{}\", ",
                "oauth_nonce=\"{}\", ",
                "oauth_signature=\"{}\", ",
                "oauth_signature_method=\"HMAC-SHA1\", ",
                "oauth_timestamp=\"{}\", ",
                "oauth_token=\"{}\", ",
                "oauth_version=\"1.0\"",
            ),

            // concat!(
            //     "OAuth ",
            //     "oauth_consumer_key={}, ",
            //     "oauth_nonce={}, ",
            //     "oauth_signature={}, ",
            //     "oauth_signature_method=HMAC-SHA1, ",
            //     "oauth_timestamp={}, ",
            //     "oauth_token={}, ",
            //     "oauth_version=1.0",
            // ),
            encode(&params.oauth_consumer_key),
            encode(&params.oauth_nonce),
            encode(&signature),
            encode(&params.oauth_timestamp),
            encode(&params.oauth_token),
        )
    }

    // This is what we're getting
    // POST&https%3A%2F%2Fapi.twitter.com%2F1.1%2Fstatuses%2Fupdate.json&oauth_consumer_key%3DPism1Wur3vtYZoOsqDncDPnno%26oauth_nonce%3D6e47803b1e2cdcb806692cda64b7cbb0f9ed977d882bbbcb9e783326cc9a2fb57d3527f0059baee7ca99cdf3f6f9204d2f35343a4cfdcd4961ff31e4eb6ebe87%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1636675856629%26oauth_token%3D4879236142-wRTMco8S75VlgKUmcn5FEXDA6pEHO4nvfngaZOU%26oauth_version%3D1.0%26status%3Dok
    // and this is what they want
    // POST&https%3A%2F%2Fapi.twitter.com%2F1.1%2Fstatuses%2Fupdate.json&oauth_consumer_key%3Dxvz1evFS4wEEPTGEFPHBog%26oauth_nonce%3DkYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1318622958%26oauth_token%3D370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb%26oauth_version%3D1.0%26status%3DHello%2520Ladies%2520%252B%2520Gentlemen%252C%2520a%2520signed%2520OAuth%2520request%2521
    // Looks like the main difference is &

    // https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature
    /*
    These values need to be encoded into a single string, which will be used later on. The process to build the string is very specific:

    Percent encode every key and value that will be signed.
    Sort the list of parameters alphabetically [1] by encoded key [2].
    For each key/value pair:
    Append the encoded key to the output string.
    Append the ‘=’ character to the output string.
    Append the encoded value to the output string.
    If there are more key/value pairs remaining, append a ‘&’ character to the output string.


    then...

    To encode the HTTP method, base URL, and parameter string into a single string:

    Convert the HTTP Method to uppercase and set the output string equal to this value.
    Append the ‘&’ character to the output string.
    Percent encode the URL and append it to the output string.
    Append the ‘&’ character to the output string.
    Percent encode the parameter string and append it to the output string.
    */
    fn build_signature(&self, params: &OauthParams, message: &str) -> String {
        let signing_key = format!(
            "{}&{}",
            encode(&self.credentials.api_key_secret),
            encode(&self.credentials.access_token_secret),
        );

        // I'm confident there is a better way to do this with Serialize or Deserialize but this works for now
        let parameter_string = format!(
            concat!(
                "oauth_consumer_key={}&",
                "oauth_nonce={}&",
                "oauth_signature_method=HMAC-SHA1&",
                "oauth_timestamp={}&",
                "oauth_token={}&",
                "oauth_version=1.0&",
                "status={}"
            ),
            encode(&params.oauth_consumer_key),
            encode(&params.oauth_nonce),
            encode(&params.oauth_timestamp),
            encode(&params.oauth_token),
            encode(message),
        );
        let encoded = format!(
            "POST&{}&{}",
            encode(&params.base_url),
            encode(&parameter_string)
        );

        dbg!(&encoded);

        // https://stackoverflow.com/questions/54619582/hmac-sha1-in-rust
        // TODO: can this be more of a one-liner?

        let mut mac = Hmac::new(Sha1::new(), signing_key.as_bytes());
        mac.input(encoded.as_bytes());
        let result = mac.result();
        let code = result.code();

        // let code = hmac_sha1_compact::HMAC::mac(encoded.as_bytes(), signing_key.as_bytes());

        // let code = hmacsha1::hmac_sha1(signing_key.as_bytes(), encoded.as_bytes());



        // let signature = base64::encode_config(&code, base64::URL_SAFE);
        let signature = base64::encode(&code);

        dbg!(&signature);

        signature

        // something like this.... needs base 64 too maybe??
        // hmacsha1::hmac_sha1(signing_key.as_bytes(), params.base_url.as_bytes()).to_string()
    }
}
