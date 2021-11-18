<h1 align="center">
  <img src="assets/logo.svg" width="100%" alt="Twittier. Next-gen social interface" />
</h1>

<p align="center">
  <strong><em>The Twitter CLI nobody asked for</em></strong>
</p>

- [Features](#features)
- [Comparison with twurl](#comparison-with-twurl)
- [Quick start](#quick-start)
- [API](#api)
  - [`init`](#init)
  - [`post`](#post)
    - [Using an alt profile](#using-an-alt-profile)
  - [`delete`](#delete)
  - [`feed`](#feed)
  - [`version`](#version)
  - [`help`](#help)
  - [Global arguments](#global-arguments)
- [Building from source](#building-from-source)
- [Troubleshooting](#troubleshooting)
  - [I am getting authentication errors](#i-am-getting-authentication-errors)
- [Acknowledgements](#acknowledgements)
- [Footnotes](#footnotes)

## Features

- 🚀 Fastest Twitter client on the market<sup>1</sup>
- 🤑 Absolutely free to use for live tweeting, doom scrolling, and more!
- <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" alt="rust logo" height="20px" /> Written in a language nobody cares about
- 😈 Supports multiple profiles so you can use your alts to troll with ease!
- 🤓 Allows you to dick around at work while looking like you're doing some hard-code nerd work in the terminal!

## Comparison with twurl

[twurl](https://github.com/twitter/twurl) is a Twitter CLI developed and released by Twitter

|                                                                      | twurl                        | Twittier                     |
| -------------------------------------------------------------------- | ---------------------------- | ---------------------------- |
| Is Next-Gen                                                          | ❌                           | ✅                           |
| Blazing fast<sup>1</sup>                                             | ❌                           | ✅                           |
| Shorter command name to save you valuable keystrokes                 | ❌                           | ✅                           |
| NFTs<sup>2</sup>                                                     | ❌                           | ✅                           |
| Organic & Fair Trade                                                 | ❌                           | ✅                           |
| Massive sex appeal                                                   | ❌                           | ✅                           |
| Language name matches regex `ru[a-z]{2}`                             | ✅                           | ✅                           |
| <small><sub>Officially supported</sub></small>                       | <small><sub>✅</sub></small> | <small><sub>❌</sub></small> |
| <small><sub>Supports a fuller range of the Twitter API</sub></small> | <small><sub>✅</sub></small> | <small><sub>❌</sub></small> |

## Quick start

1. Download executable from [the releases page](https://github.com/ericyd/twitter/releases)
2. Initialize your credentials file: `tw init`
3. [Create a developer account](https://developer.twitter.com/en/docs/twitter-api). Then create an app. Then grant it write permissions. Then generate an access token and secret. (This is a [better guide than I would write](https://dev.to/sumedhpatkar/beginners-guide-how-to-apply-for-a-twitter-developer-account-1kh7))
4. Copy/paste your API key, API secret, Access token, and Access token secret into your `~/.twitter_credentials.toml` file
5. See what's new: `tw feed`
6. Add to the conversation `tw post "new phone who dis"`

## API

### `init`

Initializes the credentials file. Generally you want to run this first and then [create a Twitter developer account](https://dev.to/sumedhpatkar/beginners-guide-how-to-apply-for-a-twitter-developer-account-1kh7) and populate the credentials

Arguments
* `-c`, `--credentials` (Optional)

Examples

```bash
# Defaults to ~/.twitter_credentials.toml
tw init
# Custom file
tw init -c ~/my-custom-credentials-file
```

### `post`

Alerts the world that you are still alive.

Aliases
* `tweet`
* `p`

Arguments
* `message` (Required)
* `-p`, `--profile` (Optional). Allows you to specify an alt account to use

Examples
```bash
tw post "I might have poor grammar but so are you"
tw tweet "Calamine lotion tastes funny"
tw p 'Who took the cookies from the cookie jar? Twas me, bitches'
```

#### Using an alt profile

If you want to use with multiple profiles, you'll need to set up your credentials file as follows

```toml
[default]
api_key = ""
api_key_secret = ""
access_token = ""
access_token_secret = ""

[alt1]
api_key = ""
api_key_secret = ""
access_token = ""
access_token_secret = ""
```

Running commands without a `-p` or `--profile` argument will use the `default` credentials, whereas specifying a profile will use those credentials. For example

```bash
tw post "can confirm: @ericydauenhauer is def a human" --profile alt1
```

### `delete`

Delete a prior lapse in judgment

Arguments
* `id` (Required)

Examples
```bash
tw delete 123456
```

### `feed`

See what people are saying about you

Arguments
* `count` (Optional)

Examples
```bash
# Defaults to 10 or something
tw feed
# Get an exact number (less than 100 plz)
tw feed 42
```

### `version`

Print useful information that you will need when you're filing bug reports for this software

Examples
```bash
tw -v
tw version
tw --version
```

### `help`

Help me, Obi-Wan Kenobi; you're my only hope.

Examples
```bash
tw -h
tw help
tw --help
```
### Global arguments

* `--debug`: Prints a bunch of extra info
* `help`, `--help`, `-h`: Include with another argument to get specific help message for that command.

```bash
tw post "fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck fuck donald trump" --debug
tw post -h
tw feed --help
tw init help
```

## Building from source

Install [Rust and Cargo](https://www.rust-lang.org/learn/get-started)

```bash
# Clone repo as needed
git clone https://github.com/ericyd/twittier && cd twittier

# Build and create link
cargo build --release
ln -s "$(pwd)/target/release/tw" /usr/local/bin/tw

# Use
tw -h
```

## Troubleshooting

### I am getting authentication errors

Be sure to generate an access token and secret after you update your app to have Read & Write permissions. The default is Read-only which will not work.

## Acknowledgements

* Fonts in logo: [Procrastinating Pixie](https://www.fontspace.com/pro-pixie-font-f44959) and [Lemon Milk](https://www.fontspace.com/lemon-milk-font-f44669)

## Footnotes

- <sup>1</sup>This has never been, and will never be, measured
- <sup>2</sup>Non-functional Tweets
