use async_recursion::async_recursion;
use regex::Regex;
use std::{
    env,
    io::{BufReader, Cursor},
    path::PathBuf,
    process::exit,
};
use tokio::fs;
use tokio::process::Command;

use color_eyre::eyre::{eyre, Result};
use nanoid::nanoid;
use owo_colors::OwoColorize as _;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
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
                .ok_or_else(|| eyre!("could not find path in unpacked directory"))?,
            target_dir,
        )
        .await?;
    } else {
        copy_dir_all(&temp_extract_path, target_dir).await?;
    }

    fs::remove_dir_all(&temp_extract_path).await?;

    Ok(())
}

pub fn check_firefox() {
    let system =
        System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()));
    let is_running = system.processes_by_name("firefox").count() != 0;

    if is_running {
        println!("{}", "Firefox is running, refusing to continue!".yellow());
        exit(1);
    }
}

pub fn is_remote_source(source: &str) -> bool {
    let remote_regex = Regex::new(r"^(https?://|github:|codeberg:|gitlab:)").unwrap();
    remote_regex.is_match(source)
}

pub async fn download_and_cache(source: &str, cache_path: &PathBuf) -> Result<bool> {
    let git_url = construct_git_url(source)?;

    if !cache_path.exists() {
        println!("Cloning repository to cache...");
        let output = Command::new("git")
            .args(&[
                "clone",
                "--depth",
                "1",
                &git_url,
                cache_path.to_str().unwrap(),
            ])
            .output()
            .await?;

        if !output.status.success() {
            return Err(eyre!(
                "Failed to clone repository: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(true)
    } else {
        // Fetch logic here
        let output = Command::new("git")
            .args(&["-C", cache_path.to_str().unwrap(), "fetch", "--depth", "1"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(eyre!("Failed to fetch updates"));
        }

        let output = Command::new("git")
            .args(&[
                "-C",
                cache_path.to_str().unwrap(),
                "rev-list",
                "HEAD...origin/HEAD",
                "--count",
            ])
            .output()
            .await?;

        let behind_count = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u32>()?;

        if behind_count > 0 {
            // Reset logic here
            let output = Command::new("git")
                .args(&[
                    "-C",
                    cache_path.to_str().unwrap(),
                    "reset",
                    "--hard",
                    "origin/HEAD",
                ])
                .output()
                .await?;

            if !output.status.success() {
                return Err(eyre!("Failed to reset to latest commit"));
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn construct_git_url(source: &str) -> Result<String> {
    match source.split_once(':') {
        Some(("github", repo)) => Ok(format!("https://github.com/{}.git", repo)),
        Some(("codeberg", repo)) => Ok(format!("https://codeberg.org/{}.git", repo)),
        Some(("gitlab", repo)) => Ok(format!("https://gitlab.com/{}.git", repo)),
        Some(("http", _)) | Some(("https", _)) => Ok(source.to_string()),
        _ if source.contains("://") => Ok(source.to_string()),
        _ => Ok(format!("https://{}", source)),
    }
}
