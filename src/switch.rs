use color_eyre::eyre::{eyre, Result};

use std::{
    env,
    path::{Path, PathBuf},
    process::Stdio,
};
use tokio::{fs, process::Command};

use owo_colors::OwoColorize as _;
use regex::Regex;

use crate::{
    config::{print_userchrome, Userchrome, UserchromeConfig},
    utils,
};

async fn run_arkenfox_script(profile: &str, name: &str, args: Vec<&str>) -> Result<()> {
    let suffix = match env::consts::OS {
        "windows" => ".bat",
        &_ => ".sh",
    };

    let script = Path::new(&profile).join(name.to_owned() + suffix);
    if !script.exists() {
        return Err(eyre!(
            "script {} doesn't exist in profile {}",
            name,
            profile
        ));
    }

    let mut cmd = Command::new(script);
    cmd.args(args);
    cmd.current_dir(profile);

    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    cmd.status().await?;

    Ok(())
}

const START_LINE: &str = "/** nyoom-managed config; do not edit */";
const END_LINE: &str = "/** end of nyoom-managed config */";

async fn patch_user_file(userchrome: &Userchrome, f: PathBuf) -> Result<()> {
    let contents = fs::read_to_string(&f).await.unwrap_or(String::new());
    let lines: Vec<String> = contents
        .split('\n')
        .map(std::borrow::ToOwned::to_owned)
        .collect();

    let mut new_lines = vec![
        "user_pref(\"toolkit.legacyUserProfileCustomizations.stylesheets\", true);".to_owned(),
    ];

    for c in &userchrome.configs {
        let UserchromeConfig { key, value, raw } = c;

        let value = if *raw {
            value.clone()
        } else {
            format!("\"{value}\"")
        };

        new_lines.push(format!("user_pref(\"{key}\", {value});"));
    }

    let mut ret_lines: Vec<String> = vec![];
    let start_idx = lines.iter().position(|k| k.eq(&START_LINE));
    let end_idx = lines.iter().position(|k| k.eq(&END_LINE));

    let mut ret_set = false;

    if let Some(start_idx) = start_idx {
        if let Some(end_idx) = end_idx {
            ret_lines = lines[0..=start_idx].to_vec();
            ret_lines.append(&mut new_lines);
            ret_lines.append(&mut lines[end_idx..].to_vec());
            ret_set = true;
        }
    }

    if !ret_set {
        ret_lines.clone_from(&lines);
        ret_lines.push(START_LINE.to_owned());
        ret_lines.append(&mut new_lines);
        ret_lines.push(END_LINE.to_owned());
        ret_lines.push(String::new());
    }

    fs::write(&f, ret_lines.join("\n")).await?;

    Ok(())
}

async fn user(userchrome: &Userchrome, profile: &str, step_counter: &mut i32) -> Result<()> {
    let arkenfox = Path::new(&profile).join("user-overrides.js").exists();

    if arkenfox {
        patch_user_file(userchrome, Path::new(&profile).join("user-overrides.js")).await?;

        println!("{} updating arkenfox", step_counter.to_string().green());
        *step_counter += 1;

        run_arkenfox_script(profile, "updater", vec!["-s"]).await?;
        run_arkenfox_script(profile, "prefsCleaner", vec!["-s"]).await?;
    } else {
        patch_user_file(userchrome, Path::new(&profile).join("user.js")).await?;
    }

    Ok(())
}

async fn handle_source(source: &str, target_dir: &PathBuf) -> Result<()> {
    let github_regex = Regex::new(r"github:(?P<repo>([\w\-_]+)/([\w\-_]+))(#(?P<ref>[\w\-_]+))?")?;
    let codeberg_regex =
        Regex::new(r"codeberg:(?P<repo>([\w\-_]+)/([\w\-_]+))(#(?P<ref>[\w\-_]+))?")?;
    let gitlab_regex = Regex::new(r"gitlab:(?P<repo>[\w\-_/]+)(#(?P<ref>[\w\-_]+))?")?;

    if let Some(github) = github_regex.captures(source) {
        let ref_str = match github.name("ref") {
            Some(ref_match) => ref_match.into(),
            None => "main",
        };

        let url = format!(
            "https://github.com/{}/archive/refs/heads/{}.zip",
            &github["repo"], &ref_str,
        );

        utils::download_zip(&url, target_dir).await?;
    } else if let Some(codeberg) = codeberg_regex.captures(source) {
        let ref_str = match codeberg.name("ref") {
            Some(ref_match) => ref_match.into(),
            None => "main",
        };

        let url = format!(
            "https://codeberg.org/{}/archive/{}.zip",
            &codeberg["repo"], &ref_str,
        );

        utils::download_zip(&url, target_dir).await?;
    } else if let Some(gitlab) = gitlab_regex.captures(source) {
        let ref_str = match gitlab.name("ref") {
            Some(ref_match) => ref_match.into(),
            None => "main",
        };

        let url = format!(
            "https://gitlab.com/{}/-/archive/{}/source-{}.zip",
            &gitlab["repo"], &ref_str, &ref_str,
        );

        utils::download_zip(&url, target_dir).await?;
    } else if let Some(url) = source.strip_prefix("url:") {
        utils::download_zip(url, target_dir).await?;
    } else {
        return Err(eyre!("Invalid source specification: {}", source));
    }

    Ok(())
}

pub async fn switch(userchrome: &Userchrome, profile: String) -> Result<()> {
    print_userchrome(userchrome, false);
    println!();

    let mut step_counter = 1;
    let new_chrome_dir = Path::new(&profile).join("chrome");

    println!("{} retrieving source", step_counter.to_string().green());
    step_counter += 1;

    if let Some(cache_path) = &userchrome.cache_path {
        utils::copy_dir_all(cache_path, &new_chrome_dir).await?;
    } else {
        handle_source(&userchrome.source, &new_chrome_dir).await?;
    }

    println!("{} installing userchrome", step_counter.to_string().green());
    step_counter += 1;

    if new_chrome_dir.exists() {
        fs::remove_dir_all(&new_chrome_dir).await?;
    }

    utils::copy_dir_all(&new_chrome_dir, &new_chrome_dir).await?;

    println!("{} applying user.js", step_counter.to_string().green());
    step_counter += 1;

    user(userchrome, profile.as_str(), &mut step_counter).await?;

    println!("{}", "done!".green());

    Ok(())
}
