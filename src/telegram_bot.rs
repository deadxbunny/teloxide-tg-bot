// WORKING VERSION ---------------------------------------------------------------------
use teloxide::{prelude::*, utils::command::BotCommands};
extern crate dotenv;

use dotenv::dotenv;
use std::env;

use crate::changelogs;
use crate::timezone;
use crate::fun;

#[tokio::main]
pub async fn start_bot() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new(env::var("BOT_TOKEN").expect("TOKEN NOT FOUND")); // main bot
    // let bot = Bot::new(env::var("T_BOT_TOKEN").expect("TOKEN NOT FOUND")); // testing bot

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды которые поддерживаются:")]
enum Command {
    #[command(description = "спиcок доступных комманд и подсказок")]
    Commands,
    #[command(description = "список изменений")]
    Changelogs,
    #[command(description = "остаток рабочего дня. example = /pain [chlb, msk]")]
    Pain {choice_city: String},
    #[command(description = "показать токен бота. example = /showmytoken [pass]", parse_with = "split")]
    ShowMyToken {pass: String},
    #[command(description = "бросить кубик (нахуя?)")]
    Dice,
    #[command(description = "тестовый функционал (не лезь сюда, понял?)")]
    Testing,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Commands => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Changelogs => bot.send_message(msg.chat.id, changelogs::changes()).await?,
        Command::Pain {choice_city} => bot.send_message(msg.chat.id, format!("{}", timezone::get_time(choice_city))).await?,
        Command::ShowMyToken{pass} => {
            bot.send_message(msg.chat.id, format!("{}", fun::access_token(pass, env::var("BOT_TOKEN").expect("TOKEN NOT FOUND").to_string()))).await?
        },
        Command::Dice => fun::dice_game(bot, msg).await?,
        Command::Testing => bot.send_message(msg.chat.id, timezone::testing()).await?
    };
    Ok(())
}