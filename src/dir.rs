use fs_extra::dir::CopyOptions;
use fs_extra::dir::copy;
use std::path::Path;
use zip::CompressionMethod;
use zip_extensions::zip_create_from_directory_with_options;

pub fn movedir(obsidian_folder_path: &str) {
    let tmp_dir = String::from(r"C:\obsidianbuckupdir");
    fs_extra::dir::create_all(&tmp_dir, true).expect("Недостаточно прав");

    println!("{}", &tmp_dir);
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    copy(obsidian_folder_path, &tmp_dir, &options).expect("ошибка копирования");
}

pub fn ziping_of_copy() {
    println!("архивирование");
    let tmp_dir = std::path::PathBuf::from(r"C:\obsidianbuckupdir");
    let zip_path = std::path::PathBuf::from(r"C:\obsidianbuckup.zip");

    let zip_options =
        zip::write::SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    zip_create_from_directory_with_options(&zip_path, &tmp_dir, |_| zip_options)
        .expect("ошибка архивирования");
}
