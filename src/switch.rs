// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use eyre::{bail, Result};
use temp_dir::TempDir;

use std::{env, path::Path, process::Stdio, sync::LazyLock};
use tokio::{fs, process::Command};

use owo_colors::OwoColorize as _;
use regex::Regex;

use crate::{
    config::{print_userchrome, Userchrome, UserchromeConfig},
    utils,
};

async fn run_arkenfox_script(profile: &Path, name: &str, args: &[String]) -> Result<()> {
    let suffix = match env::consts::OS {
        "windows" => ".bat",
        &_ => ".sh",
    };

    let script = Path::new(&profile).join(name.to_owned() + suffix);
    if !script.exists() {
        bail!(
            "script {} doesn't exist in profile {}",
            name,
            profile.display()
        );
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

async fn patch_user_file(f: &Path, userchrome: Option<&Userchrome>) -> Result<()> {
    let contents = fs::read_to_string(f).await?;
    let lines = contents
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let mut new_lines = vec![
        "user_pref(\"toolkit.legacyUserProfileCustomizations.stylesheets\", true);".to_owned(),
    ];

    if let Some(userchrome) = userchrome {
        for c in &userchrome.configs {
            let UserchromeConfig { key, value, raw } = c;

            let value = if *raw {
                value.clone()
            } else {
                format!("\"{value}\"")
            };

            new_lines.push(format!("user_pref(\"{key}\", {value});"));
        }
    }

    let mut ret_lines: Vec<String> = Vec::new();
    let start_idx = lines.iter().position(|k| k.eq(&START_LINE));
    let end_idx = lines.iter().position(|k| k.eq(&END_LINE));

    let mut ret_set = false;

    if let Some(start_idx) = start_idx {
        if let Some(end_idx) = end_idx {
            ret_lines.extend(lines[0..=start_idx].to_vec());
            ret_lines.append(&mut new_lines);
            ret_lines.extend(lines[end_idx..].to_vec());
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

async fn user(
    userchrome: Option<&Userchrome>,
    profile: &Path,
    step_counter: &mut i32,
) -> Result<()> {
    let arkenfox = profile.join("user-overrides.js").exists();

    if arkenfox {
        patch_user_file(&profile.join("user-overrides.js"), userchrome).await?;

        println!("{} updating arkenfox", step_counter.to_string().green());
        *step_counter += 1;

        run_arkenfox_script(profile, "updater", &["-s".to_owned()]).await?;
        run_arkenfox_script(profile, "prefsCleaner", &["-s".to_owned()]).await?;
    } else {
        patch_user_file(&profile.join("user.js"), userchrome).await?;
    }

    Ok(())
}

static GITHUB_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"github:(?P<repo>([\w\-_]+)/([\w\-_]+))(#(?P<ref>[\w\-_]+))?").unwrap()
});

static CODEBERG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"codeberg:(?P<repo>([\w\-_]+)/([\w\-_]+))(#(?P<ref>[\w\-_]+))?").unwrap()
});

static GITLAB_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"cgitlab:(?P<repo>[\w\-_/]+)(#(?P<ref>[\w\-_]+))?").unwrap());

async fn handle_source(source: &str, target_dir: &Path) -> Result<()> {
    if let Some(github) = GITHUB_REGEX.captures(source) {
        let ref_str = github
            .name("ref")
            .map_or("main", |ref_match| ref_match.into());

        let url = format!(
            "https://github.com/{}/archive/refs/heads/{}.zip",
            &github["repo"], &ref_str,
        );

        utils::download_zip(&url, target_dir).await?;
    } else if let Some(codeberg) = CODEBERG_REGEX.captures(source) {
        let ref_str = codeberg
            .name("ref")
            .map_or("main", |ref_match| ref_match.into());

        let url = format!(
            "https://codeberg.org/{}/archive/{}.zip",
            &codeberg["repo"], &ref_str,
        );

        utils::download_zip(&url, target_dir).await?;
    } else if let Some(gitlab) = GITLAB_REGEX.captures(source) {
        let ref_str = gitlab
            .name("ref")
            .map_or("main", |ref_match| ref_match.into());

        let url = format!(
            "https://gitlab.com/{}/-/archive/{}/source-{}.zip",
            &gitlab["repo"], &ref_str, &ref_str,
        );

        utils::download_zip(&url, target_dir).await?;
    } else if let Some(url) = source.strip_prefix("url:") {
        utils::download_zip(url, target_dir).await?;
    } else {
        bail!("Invalid source specification: {}", source);
    }

    Ok(())
}

pub async fn switch(userchrome: Option<&Userchrome>, profile: &Path) -> Result<()> {
    if let Some(userchrome) = userchrome {
        print_userchrome(userchrome, false);
        println!();
    }

    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    let mut step_counter = 1;

    if let Some(userchrome) = userchrome {
        println!("{} retrieving source", step_counter.to_string().green());
        step_counter += 1;

        handle_source(&userchrome.source, temp_path).await?;

        println!("{} installing userchrome", step_counter.to_string().green());
        step_counter += 1;

        let new_chrome_dir = profile.join("chrome");

        if new_chrome_dir.exists() {
            fs::remove_dir_all(&new_chrome_dir).await?;
        }

        let mut cloned_chrome_dir = temp_path.join("chrome");
        if !cloned_chrome_dir.exists() {
            temp_path.clone_into(&mut cloned_chrome_dir);
        }

        utils::copy_dir_all(&cloned_chrome_dir, &new_chrome_dir).await?;
        fs::write(new_chrome_dir.join(".nyoom-chrome-name"), &userchrome.name).await?;
    } else {
        println!("{} removing userchrome", step_counter.to_string().green());
        step_counter += 1;
        fs::remove_dir_all(profile.join("chrome")).await?;
    }

    println!("{} applying user.js", step_counter.to_string().green());
    step_counter += 1;

    user(userchrome, profile, &mut step_counter).await?;

    println!("{}", "done!".green());

    Ok(())
}
