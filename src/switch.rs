use anyhow::{anyhow, Result};
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use colored::*;
use nanoid::nanoid;
use regex::Regex;
use zip::ZipArchive;

use crate::config::{print_userchrome, Userchrome, UserchromeConfig};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

fn run_arkenfox_script(profile: &str, name: &str, args: Vec<&str>) -> Result<()> {
    let suffix = match env::consts::OS {
        "windows" => ".bat",
        &_ => ".sh",
    };

    let script = Path::new(&profile).join(name.to_owned() + suffix);
    if !script.exists() {
        return Err(anyhow!(
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

    cmd.status()?;

    Ok(())
}

const START_LINE: &str = "/** nyoom-managed config; do not edit */";
const END_LINE: &str = "/** end of nyoom-managed config */";

fn patch_user_file(userchrome: &Userchrome, f: PathBuf) -> Result<()> {
    let contents = fs::read_to_string(&f).unwrap_or("".to_owned());
    let lines: Vec<String> = contents.split('\n').map(|a| a.to_owned()).collect();

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
            ret_lines = lines[0..start_idx + 1].to_vec();
            ret_lines.append(&mut new_lines);
            ret_lines.append(&mut lines[end_idx..].to_vec());
            ret_set = true;
        }
    }

    if !ret_set {
        ret_lines = lines.clone();
        ret_lines.push(START_LINE.to_owned());
        ret_lines.append(&mut new_lines);
        ret_lines.push(END_LINE.to_owned());
        ret_lines.push("".to_owned());
    }

    fs::write(&f, ret_lines.join("\n"))?;

    Ok(())
}

fn user(userchrome: &Userchrome, profile: &str, step_counter: &mut i32) -> Result<()> {
    let arkenfox = Path::new(&profile).join("user-overrides.js").exists();

    if arkenfox {
        patch_user_file(userchrome, Path::new(&profile).join("user-overrides.js"))?;

        println!("{} updating arkenfox", step_counter.to_string().green());
        *step_counter += 1;

        run_arkenfox_script(profile, "updater", vec!["-s"])?;
        run_arkenfox_script(profile, "prefsCleaner", vec!["-s"])?;
    } else {
        patch_user_file(userchrome, Path::new(&profile).join("user.js"))?;
    }

    Ok(())
}

fn handle_source_zip(url: &str, target_dir: &PathBuf) -> Result<()> {
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
            &extracted_contents_last_path.ok_or(anyhow!(""))?,
            &target_dir,
        )?;
    } else {
        copy_dir_all(&temp_extract_path, &target_dir)?;
    }

    fs::remove_file(&temp_download_path)?;
    fs::remove_dir_all(&temp_extract_path)?;

    Ok(())
}

fn handle_source(source: &str, target_dir: &PathBuf) -> Result<()> {
    let github_regex = Regex::new(r"github:(?P<repo>([\w_-]+)/([\w_-]+))(#(?P<ref>[\w_-]+))?")?;

    if let Some(github) = github_regex.captures(source) {
        let ref_str = match github.name("ref") {
            Some(ref_match) => ref_match.into(),
            None => "main",
        };

        let url = format!(
            "https://github.com/{}/archive/refs/heads/{}.zip",
            &github["repo"], &ref_str,
        );

        handle_source_zip(&url, &target_dir)?;
    }

    Ok(())
}

pub fn switch(userchrome: &Userchrome, profile: String) -> Result<()> {
    print_userchrome(userchrome, false);
    println!();

    let temp_path = env::temp_dir().join(nanoid!());

    let mut step_counter = 1;

    println!("{} cloning repository", step_counter.to_string().green());
    step_counter += 1;

    handle_source(&userchrome.source, &temp_path)?;

    println!("{} installing userchrome", step_counter.to_string().green());
    step_counter += 1;

    let new_chrome_dir = Path::new(&profile).join("chrome");

    if new_chrome_dir.exists() {
        fs::remove_dir_all(&new_chrome_dir)?;
    }

    let mut cloned_chrome_dir = temp_path.join("chrome");
    if !cloned_chrome_dir.exists() {
        cloned_chrome_dir = temp_path;
    }

    copy_dir_all(&cloned_chrome_dir, &new_chrome_dir)?;

    println!("{} applying user.js", "3".green());
    step_counter += 1;

    user(userchrome, profile.as_str(), &mut step_counter)?;

    println!("{}", "done!".green());

    Ok(())
}
