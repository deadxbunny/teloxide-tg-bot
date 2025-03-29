mod telegram_bot;
mod changelogs;
mod timezone;
mod fun;

fn main() {
    println!("Бот запущен. @dxb3303_bot");
    telegram_bot::start_bot();
}