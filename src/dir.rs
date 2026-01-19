use fs_extra::dir::CopyOptions;
use fs_extra::dir::copy;
use std::process::Command;

pub fn movedir(obsidian_folder_path: &str) {
    let tmp_dir = String::from(r"C:\obsidianbuckupdir");
    fs_extra::dir::create_all(&tmp_dir, true).expect("Недостаточно прав");

    println!("{}", &tmp_dir);
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    copy(obsidian_folder_path, &tmp_dir, &options).expect("ошбика копирования");
}

pub fn ziping_of_copy() {
    println!("архивирование");
    fs_extra::dir::create(r"C:\obsidianbuckupdir\zipper", true).expect("okak");
    Command::new("powershell")
        .args(&[
            "-Command",
            r"Compress-Archive -Path 'C:\obsidianbuckupdir' -DestinationPath 'C:\obsidianbuckupdir\obsidian_backup.zip' -Force",
        ])
        .status()
        .expect("Не удалось создать архив");
}
