
# Rust Telegram Search Bot

This Rust kit allows you to create a Telegram bot that can search through the Search API and open other websites. **Currently, search is only supported through the Google Search API.**

## Prerequisites

Before you start, make sure you have the following:

- Rust installed on your system
- A Telegram Bot API token and a Google Cloud Platform project with the necessary credentials for the Google Search API

## Getting Started

1. Install the crate:

```sh
cargo install search-bot
```

2. Setup `.env` file from `.env.example` file in your [`workdir`]:
```ini
GOOGLE_SEARCH_API_KEY=[secret-string]
GOOGLE_SEARCH_APP_ID=[special-string]
TELOXIDE_TOKEN=[secret-string]
ALLOWED_USERS=id1,id2 # To get your id, start the bot and run command /getid
```

3. Navigate to your `workdir`:
```sh
cd /path/to/my/workdir
```

4. Run bot:
___
```sh
search-bot
```
To watch more logs set the system env variable `RUST_LOG=info`
___

## Usage
`To search`: enter any text to the bot.  
`To open any site`: enter to the bot a site URL starting with `http://` or `https://`

Happy hacking!