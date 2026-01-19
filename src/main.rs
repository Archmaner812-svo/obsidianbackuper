mod dir;
mod get;
use chrono;
use frankenstein::AsyncTelegramApi;
use frankenstein::client_reqwest::Bot;
use frankenstein::methods::{SendDocumentParams, SendMessageParams};
use std::fs;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match fs::read_to_string("obsbackuptoken.txt") {
        Ok(content) => {
            println!("значение токена найдено")
        }
        Err(e) => {
            println!("не нашли значение вашего токена(\nвведите его еще раз");
            get::get_token();
        }
    }
    let token = fs::read_to_string("obsbackuptoken.txt").unwrap();
    let token = token.trim();
    println!("{token}");

    match fs::read_to_string("obsbackupsetting.txt") {
        Ok(chat_id) => {
            println!("значение чат айди найдено");
        }
        Err(e) => {
            println!("не нашли значение вашего чат айди\nвведите его еще раз");
            get::get_and_save_user_id();
        }
    };
    let bot = Bot::new(token);
    let chat_id = fs::read_to_string("obsbackupsetting.txt").expect("");
    println!("{chat_id}");

    let obsfoleder = get::get_obsidian_folder();
    println!("бот запущен");
    let mut interval = time::interval(Duration::from_secs(3600));

    let chat_id: i64 = chat_id
        .trim()
        .parse()
        .expect("Не удалось преобразовать строку в i64");
    println!("{chat_id}");

    loop {
        dir::movedir(&obsfoleder);
        dir::ziping_of_copy();
        interval.tick().await;

        let params = SendMessageParams::builder()
            .chat_id(chat_id)
            .text("начинаем отправку")
            .build();
        println!("отправлен лог об начале отправки");
        // let now = chrono::offset::Utc::now();
        bot.send_message(&params).await.unwrap();
        let file = std::path::PathBuf::from(r"C:\obsidianbuckupdir\obsidian_backup.zip");
        let params = SendDocumentParams::builder()
            .chat_id(chat_id)
            .document(file)
            .caption("Вот ваш файл!") // Опционально
            .build();
        bot.send_document(&params).await.unwrap();
        println!("отправлен архив");
        interval.tick().await;
    }
}
