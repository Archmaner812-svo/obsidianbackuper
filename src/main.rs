use powershell_script;
use std::time::Duration;
use std::{fs, io, str};
use fs_extra::copy_items;
use fs_extra::dir::{CopyOptions, copy, create};
use std::thread;
use std::os;
use fs_extra::dir;
use std::process::Command;
use std::path::{self, Path};
use std::fs::OpenOptions;
use std::io::Write;


fn main() {
    
    

}



fn get_obsidian_folder() -> String {
    let mut obsidian_folder_path = String::new();
    
    io::stdin()
        .read_line(&mut obsidian_folder_path)
        .expect("абаюдна");
    
    println!("Путь получен: {}",obsidian_folder_path);
    obsidian_folder_path.trim().to_string()
}
fn copy_dir_to_tmp_dir(obsidian_folder_path: &str) {
    let tmp_dir = String::from(r"C:\ts2");
    fs_extra::dir::create_all(&tmp_dir,true)
        .expect("Недостаточно прав");
    
    println!("{}",&tmp_dir);
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    copy(obsidian_folder_path,  &tmp_dir, &options)
    .expect("ошбика копирования");


    
}

fn ziping_of_copy() -> bool {
    println!("архивирование");
    // let zip_direct =String::from(r"");
    Command::new("powershell")
        .args(&[
            "-Command",
            r"Compress-Archive -Path 'C:\ts2' -DestinationPath 'C:\ts2\obsidian_backup.zip' -Force"
        ])
        .status()
        .expect("Не удалось создать архив");

    let succses = true;
    succses
}
fn get_obsidian_folder_name(path_to_folder:&String) -> String{
    let path = Path::new(&path_to_folder);
    let folder_name = path
    .file_name()
    .expect("Путь не содержит имени файла")
    .to_str()
    .expect("Имя файла не в UTF-8")
    .to_string();
    println!("{}",folder_name);



    folder_name
    
}






fn get_and_save_user_id(){
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
    writeln!(file,"{user_id}").expect("Не удалось запсать user id");
    println!("если вы знаете что ошибились то найдите файл obsbackupsetting.txt и запиште туда ваш айди в телеграмм в во вторую строку");
    };
        
}

fn get_token(){
    println!("Напиши токен своего бота");
    let mut token = String::new();
    io::stdin()
        .read_line(&mut token)
        .expect("");
    token = format!("$env:TELOXIDE_TOKEN={}>",token);
    println!("{}",token)

}