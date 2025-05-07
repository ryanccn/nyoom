// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use eyre::{Result, bail, eyre};
use temp_dir::TempDir;

use std::{io, path::Path, process::Stdio};
use tokio::{fs, process::Command};

use anstream::println;
use owo_colors::OwoColorize as _;

use crate::{
    config::{PrintContext, Userchrome, UserchromeConfig},
    source::ParsedSource,
    utils,
};

#[cfg(windows)]
static ARKENFOX_SCRIPT_SUFFIX: &str = ".bat";
#[cfg(not(windows))]
static ARKENFOX_SCRIPT_SUFFIX: &str = ".sh";

async fn run_arkenfox_script(profile: &Path, name: &str, args: &[&str]) -> Result<()> {
    let script = profile.join(name.to_owned() + ARKENFOX_SCRIPT_SUFFIX);

    let mut cmd = Command::new(script);
    cmd.args(args).current_dir(profile);

    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    if !cmd.status().await?.success() {
        bail!("failed to run arkenfox script {name:?}");
    }

    Ok(())
}

const START_LINE: &str = "/** nyoom-managed config; do not edit */";
const END_LINE: &str = "/** end of nyoom-managed config */";

async fn patch_user_file(f: &Path, userchrome: Option<&Userchrome>, now: &str) -> Result<()> {
    let contents = match fs::read_to_string(f).await {
        Ok(contents) => contents,
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                String::new()
            } else {
                return Err(err.into());
            }
        }
    };

    let lines = contents.lines().collect::<Vec<_>>();

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

    let mut ret_lines: Vec<&str> = Vec::new();
    let start_idx = lines.iter().position(|k| k == &START_LINE);
    let end_idx = lines.iter().position(|k| k == &END_LINE);

    if let (Some(start_idx), Some(end_idx)) = (start_idx, end_idx) {
        ret_lines.extend(lines[0..=start_idx].iter());
        ret_lines.extend(new_lines.iter().map(|s| s.as_str()));
        ret_lines.extend(lines[end_idx..].iter());
    } else {
        fs::rename(
            &f,
            f.with_file_name(format!(
                "{}.nyoom-{now}.bak",
                f.file_name()
                    .ok_or_else(|| eyre!("could not obtain file name"))?
                    .to_string_lossy()
            )),
        )
        .await?;

        ret_lines.clone_from(&lines);
        ret_lines.push(START_LINE);
        ret_lines.extend(new_lines.iter().map(|s| s.as_str()));
        ret_lines.push(END_LINE);
    }

    if !ret_lines.last().is_some_and(|s| s.is_empty()) {
        ret_lines.push("");
    }

    fs::write(&f, ret_lines.join("\n")).await?;

    Ok(())
}

async fn apply_user_file(
    userchrome: Option<&Userchrome>,
    profile: &Path,
    now: &str,
    step_counter: &mut i32,
) -> Result<()> {
    let arkenfox = profile.join("user-overrides.js").exists();

    if arkenfox {
        patch_user_file(&profile.join("user-overrides.js"), userchrome, now).await?;

        println!("{} updating arkenfox", step_counter.green());
        *step_counter += 1;

        run_arkenfox_script(profile, "updater", &["-s"]).await?;
        run_arkenfox_script(profile, "prefsCleaner", &["-s"]).await?;
    } else {
        patch_user_file(&profile.join("user.js"), userchrome, now).await?;
    }

    Ok(())
}

pub async fn switch(userchrome: Option<&Userchrome>, profile: &Path) -> Result<()> {
    if let Some(userchrome) = userchrome {
        userchrome.print(false, PrintContext::Normal);
        println!();
    }

    let mut step_counter = 1;
    let now = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();

    if let Some(userchrome) = userchrome {
        println!("{} retrieving source", step_counter.green());
        step_counter += 1;

        let temp_dir = TempDir::new()?;

        userchrome
            .source
            .parse::<ParsedSource>()?
            .retrieve(&temp_dir)
            .await?;

        println!("{} installing userchrome", step_counter.green());
        println!("{} {}", "╰".cyan().dimmed(), profile.display().dimmed());
        step_counter += 1;

        let new_chrome_dir = profile.join("chrome");
        let new_chrome_name_file = new_chrome_dir.join(".nyoom-chrome-name");

        if new_chrome_dir.exists() {
            if new_chrome_name_file.exists() {
                fs::remove_dir_all(&new_chrome_dir).await?;
            } else {
                fs::rename(
                    &new_chrome_dir,
                    &profile.join(format!("chrome.nyoom-{now}.bak")),
                )
                .await?;
            }
        }

        let src_chrome_dir = if temp_dir.path().join("chrome").exists() {
            &temp_dir.path().join("chrome")
        } else {
            temp_dir.path()
        };

        utils::copy_dir_all(src_chrome_dir, &new_chrome_dir).await?;
        fs::write(&new_chrome_name_file, &userchrome.name).await?;
    } else {
        println!("{} removing userchrome", step_counter.green());
        step_counter += 1;
        fs::remove_dir_all(profile.join("chrome")).await?;
    }

    println!("{} applying user.js", step_counter.green());
    step_counter += 1;

    apply_user_file(userchrome, profile, &now, &mut step_counter).await?;

    println!("{}", "done!".green());

    Ok(())
}
