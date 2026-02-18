use fs_extra::dir::CopyOptions;
use fs_extra::dir::copy;
use std::path::{Path, PathBuf};
use zip::CompressionMethod;
use zip_extensions::zip_create_from_directory_with_options;

fn get_temp_dir() -> PathBuf {
    std::env::temp_dir().join("obsidianbackup")
}

pub fn movedir(obsidian_folder_path: &str) {
    let tmp_dir = get_temp_dir();
    let _ = fs_extra::dir::remove(&tmp_dir);
    fs_extra::dir::create_all(&tmp_dir, true).expect("Недостаточно прав");

    println!("Временная директория: {}", tmp_dir.display());
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    copy(obsidian_folder_path, &tmp_dir, &options).expect("ошибка копирования");
}

pub fn ziping_of_copy() -> PathBuf {
    println!("архивирование");
    let tmp_dir = get_temp_dir();
    let zip_path = std::env::temp_dir().join("obsidianbackup.zip");

    let zip_options =
        zip::write::SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    zip_create_from_directory_with_options(&zip_path, &tmp_dir, |_| zip_options)
        .expect("ошибка архивирования");

    zip_path
}
