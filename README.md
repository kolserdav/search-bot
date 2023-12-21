
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
USERS=id1,id2
```
3. Navigate to your `workdir`:
```sh
cd /abs/path/to/my/workdir
```
4. Run bot:
```sh
search-bot
```