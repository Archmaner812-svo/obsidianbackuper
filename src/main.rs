use std::time::Duration;
use std::{io, str};
use fs_extra::copy_items;
use fs_extra::dir::{CopyOptions, copy, create};
use std::thread;
use std::os;
use fs_extra::dir;
use std::process::Command;
use std::path::{self, Path};

fn main() {
    println!("812");
    let obs1 = get_obsidian_folder();
    let obs2 = get_obsidian_folder_name(&obs1);
    
    
    
    let whiler = true;
    while whiler == true {
        copy_directory_to_tmp_dir(&obs1);

                        
        
        let zip_check = ziping_of_copy();
        if zip_check == true{
            println!("done")
        }
        thread::sleep(Duration::from_hours(1));
        
    }
    

}



fn get_obsidian_folder() -> String {
    let mut obsidian_folder_path = String::new();
    
    io::stdin()
        .read_line(&mut obsidian_folder_path)
        .expect("абаюдна");
    
    println!("Путь получен: {}",obsidian_folder_path);
    obsidian_folder_path.trim().to_string()
}
fn copy_directory_to_tmp_dir(obsidian_folder_path: &str) {
    let mut tmp_dir = String::new();
    tmp_dir = String::from(r"C:\ts2");
    let tmp_dir2 = tmp_dir.clone();
    fs_extra::dir::create_all(tmp_dir,true)
        .expect("Недостаточно прав");
    
    println!("{}",tmp_dir2);
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    copy(obsidian_folder_path,  tmp_dir2, &options)
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