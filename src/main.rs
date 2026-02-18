mod dir;
mod get;
use frankenstein::AsyncTelegramApi;
use frankenstein::client_reqwest::Bot;
use frankenstein::methods::{SendDocumentParams, SendMessageParams};
use std::fs;
use std::sync::Arc;
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
    let bot = Arc::new(Bot::new(token));
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
        let file = dir::ziping_of_copy();
        let params = SendMessageParams::builder()
            .chat_id(chat_id)
            .text("начинаем отправку")
            .build();
        println!("отправлен лог об начале отправки");
        bot.send_message(&params).await.unwrap();

        let send_params = SendDocumentParams::builder()
            .chat_id(chat_id)
            .document(file.clone())
            .caption("Вот ваш файл!")
            .build();

        let bot_clone = Arc::clone(&bot);
        let send_task = tokio::spawn(async move { bot_clone.send_document(&send_params).await });

        let start = std::time::Instant::now();
        let dots = ["|", "/", "-", "\\"];
        let mut i = 0;

        while !send_task.is_finished() {
            print!(
                "\r{} {:.1}s",
                dots[i % dots.len()],
                start.elapsed().as_secs_f32()
            );
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
            i += 1;
        }

        let result = send_task.await.unwrap();
        match result {
            Ok(_) => println!(
                "\r✅ Отправлено! За {:.1}s          ",
                start.elapsed().as_secs_f32()
            ),
            Err(_e) => println!("\r❌ Ошибка!                      "),
        }

        println!("отправлен архив");
        interval.tick().await;
    }
}
