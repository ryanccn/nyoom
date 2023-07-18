use std::{env, fs, io::Write, path::PathBuf, process::exit};

use anyhow::{anyhow, Result};
use colored::*;
use nanoid::nanoid;
use zip::ZipArchive;

pub fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }

    Ok(())
}

pub fn download_zip(url: &str, target_dir: &PathBuf) -> Result<()> {
    let mut resp = reqwest::blocking::get(url)?;
    resp = resp.error_for_status()?;

    let bytes = resp.bytes()?;
    let temp_download_path = env::temp_dir().join(nanoid!() + ".zip");
    let temp_extract_path = env::temp_dir().join(nanoid!());

    let mut out_file = fs::File::create(&temp_download_path)?;
    out_file.write_all(&bytes)?;

    let in_file = fs::File::open(&temp_download_path)?;

    let mut zip = ZipArchive::new(in_file)?;
    fs::create_dir(&temp_extract_path)?;
    zip.extract(&temp_extract_path)?;

    let extracted_contents = fs::read_dir(&temp_extract_path)?;

    let mut extracted_contents_size = 0;
    let mut extracted_contents_last_path: Option<PathBuf> = None;
    for f in extracted_contents {
        extracted_contents_last_path = f?.path().into();
        extracted_contents_size += 1;
    }

    if extracted_contents_size == 1 {
        copy_dir_all(
            &extracted_contents_last_path
                .ok_or(anyhow!("could not find path in unpacked directory"))?,
            target_dir,
        )?;
    } else {
        copy_dir_all(&temp_extract_path, target_dir)?;
    }

    fs::remove_file(&temp_download_path)?;
    fs::remove_dir_all(&temp_extract_path)?;

    Ok(())
}

pub fn check_profile(profile: &str) -> Result<()> {
    let mut profile_contents = fs::read_dir(profile)?;
    let is_in_use = profile_contents.any(|f| {
        if let Ok(f) = f {
            let file_name = f.file_name().into_string();

            if let Ok(file_name) = file_name {
                return file_name.ends_with("shm") || file_name.ends_with("wal");
            }
        }

        false
    });

    if is_in_use {
        println!("{}", "Profile is in use, refusing to continue!".red());
        exit(1);
    }

    Ok(())
}
