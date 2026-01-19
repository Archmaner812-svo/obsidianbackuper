use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

pub fn get_obsidian_folder() -> String {
    let mut obsidian_folder_path = String::new();
    println!("введите полный путь до папки с обсидианом");
    io::stdin()
        .read_line(&mut obsidian_folder_path)
        .expect("абаюдна");

    println!("Путь получен: {}", obsidian_folder_path);
    obsidian_folder_path.trim().to_string()
}
pub fn get_obsidian_folder_name(path_to_folder: &String) -> String {
    let path = Path::new(&path_to_folder);
    let folder_name = path
        .file_name()
        .expect("Путь не содержит имени файла")
        .to_str()
        .expect("Имя файла не в UTF-8")
        .to_string();
    println!("{}", folder_name);

    folder_name
}
pub fn get_and_save_user_id() {
    let mut user_id = String::new();
    io::stdin()
        .read_line(&mut user_id)
        .expect("неудалось получить ваш id");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) // создаёт файл, если не существует
        .open("obsbackupsetting.txt")
        .expect("Не удалось открыть файл");
    writeln!(file, "{user_id}").expect("Не удалось запсать user id");
    println!(
        "если вы знаете что ошибились то найдите файл obsbackupsetting.txt и запиште туда ваш айди в телеграмм в первую строку"
    );
}

pub fn get_token() -> String {
    println!("Напиши токен своего бота");
    let mut token = String::new();
    io::stdin().read_line(&mut token).expect("");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) // создаёт файл, если не существует
        .open("obsbackuptoken.txt")
        .expect("Не удалось открыть файл");
    writeln!(file, "{token}").expect("Не удалось запсать token");
    token
}
