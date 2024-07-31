use std::{
    io::Cursor,
    path::{Path, PathBuf},
    process::exit,
};

use async_recursion::async_recursion;
use color_eyre::eyre::{eyre, Result};
use owo_colors::OwoColorize as _;
use regex::Regex;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tokio::{fs, process::Command, task};

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
    let bytes = reqwest::get(url).await?.bytes().await?;
    let target_dir = target_dir.clone();

    task::spawn_blocking(move || -> Result<()> {
        let reader = Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(reader)?;

        archive.extract(&target_dir)?;
        Ok(())
    })
    .await??;

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

pub async fn download_and_cache(source: &str, cache_path: &Path) -> Result<bool> {
    let git_url = construct_git_url(source);

    if cache_path.exists() {
        let fetch_output = Command::new("git")
            .args(["-C", cache_path.to_str().unwrap(), "fetch", "--depth", "1"])
            .output()
            .await?;

        if !fetch_output.status.success() {
            return Err(eyre!("Failed to fetch updates"));
        }

        let behind_count = String::from_utf8_lossy(
            &Command::new("git")
                .args([
                    "-C",
                    cache_path.to_str().unwrap(),
                    "rev-list",
                    "HEAD...origin/HEAD",
                    "--count",
                ])
                .output()
                .await?
                .stdout,
        )
        .trim()
        .parse::<u32>()?;

        if behind_count > 0 {
            Command::new("git")
                .args([
                    "-C",
                    cache_path.to_str().unwrap(),
                    "reset",
                    "--hard",
                    "origin/HEAD",
                ])
                .output()
                .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        println!("Cloning repository to cache...");
        let clone_output = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                &git_url,
                cache_path.to_str().unwrap(),
            ])
            .output()
            .await?;

        if !clone_output.status.success() {
            return Err(eyre!(
                "Failed to clone repository: {}",
                String::from_utf8_lossy(&clone_output.stderr)
            ));
        }
        Ok(true)
    }
}

fn construct_git_url(source: &str) -> String {
    match source.split_once(':') {
        Some(("github", repo)) => format!("https://github.com/{repo}.git"),
        Some(("codeberg", repo)) => format!("https://codeberg.org/{repo}.git"),
        Some(("gitlab", repo)) => format!("https://gitlab.com/{repo}.git"),
        Some(("http" | "https", _)) => source.to_string(),
        _ if source.contains("://") => source.to_string(),
        _ => format!("https://{source}"),
    }
}
