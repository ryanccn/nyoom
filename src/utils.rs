use async_recursion::async_recursion;
use std::{
    env,
    io::{BufReader, Cursor},
    path::PathBuf,
    process::exit,
};
use tokio::fs;

use anyhow::{anyhow, Result};
use nanoid::nanoid;
use owo_colors::OwoColorize;
use sysinfo::{ProcessRefreshKind, RefreshKind, System, SystemExt};
use zip::ZipArchive;

#[async_recursion]
pub async fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst).await?;

    let mut dir_entries = fs::read_dir(src).await?;
    while let Ok(Some(entry)) = dir_entries.next_entry().await {
        let ty = entry.file_type().await?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name())).await?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name())).await?;
        }
    }

    Ok(())
}

pub async fn download_zip(url: &str, target_dir: &PathBuf) -> Result<()> {
    let mut resp = reqwest::get(url).await?;
    resp = resp.error_for_status()?;

    let bytes = resp.bytes().await?;
    let reader = BufReader::new(Cursor::new(bytes));

    let temp_extract_path = env::temp_dir().join(nanoid!());

    let mut zip = ZipArchive::new(reader)?;
    fs::create_dir(&temp_extract_path).await?;
    zip.extract(&temp_extract_path)?;

    let mut extracted_contents = fs::read_dir(&temp_extract_path).await?;

    let mut extracted_contents_size = 0;
    let mut extracted_contents_last_path: Option<PathBuf> = None;

    while let Ok(Some(f)) = extracted_contents.next_entry().await {
        extracted_contents_last_path = f.path().into();
        extracted_contents_size += 1;
    }

    if extracted_contents_size == 1 {
        copy_dir_all(
            &extracted_contents_last_path
                .ok_or(anyhow!("could not find path in unpacked directory"))?,
            target_dir,
        )
        .await?;
    } else {
        copy_dir_all(&temp_extract_path, target_dir).await?;
    }

    fs::remove_dir_all(&temp_extract_path).await?;

    Ok(())
}

pub fn check_firefox() -> Result<()> {
    let system =
        System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()));
    let is_running = system.processes_by_name("firefox").count() != 0;

    if is_running {
        println!("{}", "Firefox is running, refusing to continue!".yellow());
        exit(1);
    }

    Ok(())
}
