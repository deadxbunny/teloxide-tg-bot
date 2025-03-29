extern crate dotenv;
use std::env;

use rand::prelude::*;
use teloxide::{prelude::*, requests::JsonRequest};

/* FUN */
pub fn get_time_random_phrases() -> String {
    let phrases = [
        String::from("Рабочий день окончен. Пиздуй домой"),
        String::from("Все, отработал, вали домой"),
        String::from("Ты сам на время посмотреть не можешь?"),
        String::from("Оставайся на ночную смену. Ты наказан."),
        String::from(
            "Что? Хочешь узнать сколько осталось работать? Ты на пожизненной каторге, черт.",
        ),
        String::from("Тут должна была быть фраза, но я ее еще не придумал..."),
    ];

    let mut rng = rand::rng();
    let rand_el = rng.random_range(0..=phrases.len() - 1);

    format!("{}", phrases[rand_el])
}

pub fn get_bot_token_random_phrases() -> String {
    let phrases = [
        String::from("Мм..нет"),
        String::from("Ох...ах....не угадал"),
        String::from("Ты долбаеб? Да?"),
        String::from("Вы только посмотрите на этого малютку, токен он захотел, ах..."),
        String::from("Ладно, держи: 7543044783:GSH0yzRlvCxP05H3LN4XyI3BL4Nxxxxxxx"),
        String::from("... пшел нах"),
    ];

    let mut rng = rand::rng();
    let rand_el = rng.random_range(0..=phrases.len() - 1);

    format!("{}", phrases[rand_el])
}

pub fn err_command_random_phrases() -> String {
    let phrases = [
        String::from("Перепрочитай описание к команде, казел тупой."),
        String::from("Ебаный дебил, читай /commands"),
        String::from("Отказано."),
        String::from("Ну ну...попробуй еще раз, может с 105ой попытки получится..."),
    ];

    let mut rng = rand::rng();
    let rand_el = rng.random_range(0..=phrases.len() - 1);

    format!("{}", phrases[rand_el])
}

pub fn rest_time_phrases() -> String {
    let phrases = [
        String::from("Отвали."),
        String::from("На день недели посмотри, ебантяй"),
        String::from("У меня выходной"),
        String::from("Че те надо урод"),
    ];

    let mut rng = rand::rng();
    let rand_el = rng.random_range(0..=phrases.len() - 1);

    format!("{}", phrases[rand_el])
}

pub fn access_token(user_pass: String, bot_token: String) -> String {
    if !user_pass.is_empty() {
        if user_pass == env::var("PASS_FOR_TOKEN").expect("PASSWORD NOT FOUND") {
            format!("так уж и быть..\n{}", bot_token)
        } else {
            get_bot_token_random_phrases()
        }
    } else {
        err_command_random_phrases()
    }
}

// DICE GAME

pub fn dice_game(bot: Bot, msg: Message) -> JsonRequest<teloxide::payloads::SendDice> {
    bot.send_dice(msg.chat.id)
}