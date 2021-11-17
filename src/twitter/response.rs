use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TwitterCreateResponseData {
    pub id: String,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct TwitterDeleteResponseData {
    pub deleted: bool,
}

#[derive(Deserialize, Debug)]
pub struct TwitterResponse<T> {
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub struct TwitterUser {
    id: u64,
    id_str: String,
    name: String,        // display name
    screen_name: String, // handle
    verified: bool,
}

// This has basically everything that TwitterFeedItem has, but I don't want to deal with making a recursive structure work
#[derive(Deserialize, Debug)]
pub struct TwitterStatus {
    id_str: String, // identical to id, but in String formaat
    text: String,
    user: TwitterUser,
}

#[derive(Deserialize, Debug)]
pub struct TwitterFeedItem {
    created_at: String, // format: "Wed Oct 10 20:19:24 +0000 2018",
    id: u64,
    id_str: String, // identical to id, but in String formaat
    text: String,
    user: TwitterUser,
    retweet_count: i32,
    favorite_count: i32,
    favorited: bool,
    retweeted: bool,
    in_reply_to_status_id: Option<u64>,
    in_reply_to_status_id_str: Option<String>,
    in_reply_to_user_id: Option<u64>,
    in_reply_to_user_id_str: Option<String>,
    in_reply_to_screen_name: Option<String>,
    retweeted_status: Option<TwitterStatus>,
}

impl TwitterFeedItem {
    pub fn display(&self) {
        println!("---------------------------------\n");
        println!("{}, @{}", self.user.name, self.user.screen_name);
        match self.retweeted_status {
            Some(ref retweeted_status) => println!(
                "Retweeted from: {}, @{}",
                retweeted_status.user.name, retweeted_status.user.screen_name
            ),
            None => (),
        };

        // Future optimization: This doesn't come sequentially in the feed,
        // so a cool future enhancement would be to organize this data such that
        // if a tweet is in reply to another tweet,
        // go and fetch it (or find it in the vec) and print them near each other
        match self.in_reply_to_screen_name {
            Some(ref in_reply_to_screen_name) => println!(
                "Replied to: {} - https://twitter.com/{}/status/{}",
                in_reply_to_screen_name,
                in_reply_to_screen_name,
                self.in_reply_to_status_id_str
                    .as_ref()
                    .unwrap_or(&"".to_string())
            ),
            None => (),
        };
        println!("");

        // Actual tweet text is in the re-tweet.
        // TODO: not sure about quoted retweets actually 🤔
        match self.retweeted_status {
            Some(ref retweeted_status) => println!("{}", retweeted_status.text),
            None => println!("{}", self.text),
        };

        // Get those stats
        println!("");
        println!(
            "{}{} Retweets      {}{} Likes",
            self.retweet_count,
            if self.retweeted { "✅" } else { "" },
            self.favorite_count,
            if self.favorited { "✅" } else { "" }
        );

        println!(
            "https://twitter.com/{}/status/{}\n{}\n",
            self.user.screen_name, self.id_str, self.created_at
        );
    }
}

pub type TwitterFeed = Vec<TwitterFeedItem>;
