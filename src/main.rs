mod constants;
use constants::{get_env, dotenv_load, ALLOWED_USERS};
use regex::Regex;
use reqwest::{self, Result};
use tokio;
mod search;
use crate::search::google_search;
use teloxide::{prelude::*, types::ParseMode, macros::BotCommands};
mod parse;
use html2text::from_read;
use parse::get_site_data;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "get user id")]
    GetId,
}

fn check_user(users: Vec<&str>, chat_id: String) -> bool{
    let mut check = false;
    for u in users {
        if u.to_string() == chat_id {
            check = true;
        }
    }
    check
}
#[tokio::main]
async fn main() -> Result<()> {
    dotenv_load();

    pretty_env_logger::init();
    log::info!("Starting search bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    
        // Handle commands
        let cmd = &msg.text().unwrap();
        let mut is_command = false;
        if cmd.to_owned() == "/getid" {
            bot.send_message(msg.chat.id, msg.chat.id.to_string()).await?;
            is_command = true;
        }
        if is_command {
            return Ok(());
        }

        // Handle messages
        let allowed_users = get_env(ALLOWED_USERS);
        if let None = &allowed_users {
            log::warn!("Users not allowed");
        } else {
            log::info!("Allowed users {:?}", &allowed_users);
        }
        let allowed_users = allowed_users.unwrap();
        let allowed_users = allowed_users.split(",").collect::<Vec<&str>>();
      
        if !check_user(allowed_users, msg.chat.id.to_string()) {
            log::warn!("User not allowed {:?}", msg);
            return Ok(());
        }
        
        let text = msg.text().unwrap();

        let url_re = Regex::new(r"^https?:\/\/[a-zA-Z0-9\-_\.\?\/=&]+$").unwrap();
        let capts = url_re.captures(text);
        if let Some(val) = capts {
            let val = val.get(0).unwrap().as_str().to_string();
            let data = get_site_data(val.as_str()).await;
            if let Err(err) = data {
                log::error!("Failed open site {:?}", err);
                bot.send_message(msg.chat.id, err.to_string()).await?;
                return Ok(());
            }
            let data = data.unwrap();
           
            let subs = data.as_bytes().chunks(2999);
            for sub in subs {
                let sub = from_read(sub, 60);
                let re = Regex::new(r"^\s+$").unwrap();
                let capts = re.captures(sub.as_str());
                
                if let Some(_) = capts {
                    continue;
                }
                if sub == "" {
                    continue;
                }

                bot.send_message(msg.chat.id, sub).await?;
            }

            bot.send_message(msg.chat.id, "To begining")
                .reply_to_message_id(msg.id)
                .await?;
            return Ok(());
        }

        let data = google_search(text).await;
        if let Err(err) = data {
            log::error!("Failed search {:?}", err);
            bot.send_message(msg.chat.id, err.to_string()).await?;
            return Ok(());
        }
        let (data, raw) = data.unwrap();
        if let None = data {
            bot.send_message(msg.chat.id, raw).await?;
            return Ok(());
        }
        let data = data.unwrap();

        for item in data.items {
            bot.send_message(
                msg.chat.id,
                format!(
                    "<b>{}</b>\n{}\n<code>{}</code>",
                    item.title, item.snippet, item.link
                ),
            )
            .parse_mode(ParseMode::Html)
            .await?;
        }
    
        bot.send_message(msg.chat.id, "To ask")
        .reply_to_message_id(msg.id)
        .await?;

        Ok(())
    })
    .await;

    Ok(())
}
