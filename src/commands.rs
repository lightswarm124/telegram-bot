use teloxide::prelude::*;
use teloxide::macros::BotCommands;
use reqwest::Client;
use crate::market_data::fetch_market_data;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
    Help,
    Market,
    #[command(rename = "unknown")]
    Unknown(String),
}

pub async fn handle_command(
    bot: Bot,
    message: Message,
    command: Command,
    client: Client,
    api_key: String,
) -> ResponseResult<()> {
    match command {
        Command::Start => {
            bot.send_message(message.chat.id, "Welcome! Use /help to see available commands.")
                .await?;
        }
        Command::Help => {
            bot.send_message(message.chat.id, "/start - Start the bot\n/help - Show help\n/market - Get market data")
                .await?;
        }
        Command::Market => {
            let data = fetch_market_data(&client, &api_key).await.unwrap_or_else(|_| "Failed to fetch market data.".to_string());
            bot.send_message(message.chat.id, data).await?;
        }
        Command::Unknown(command) => {
            bot.send_message(message.chat.id, format!("Unknown command: {}", command))
                .await?;
        }
    }

    Ok(())
}
