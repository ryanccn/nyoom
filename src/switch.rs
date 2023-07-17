use std::{
    env, fs,
    io::Result,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use colored::*;
use nanoid::nanoid;

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

fn run_arkenfox_script(profile: &str, name: &str, args: Vec<&str>) {
    let suffix = match env::consts::OS {
        "windows" => ".bat",
        &_ => ".sh",
    };

    let script = Path::new(&profile).join(name.to_owned() + suffix);
    if !script.exists() {
        panic!("script {} doesn't exist in profile {}", name, profile);
    }

    let mut cmd = Command::new(script);
    cmd.args(args);
    cmd.current_dir(profile);

    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    cmd.status().unwrap();
}

const START_LINE: &str = "/** nyoom-managed config; do not edit */";
const END_LINE: &str = "/** end of nyoom-managed config */";

fn patch_user_file(userchrome: &Userchrome, f: PathBuf) {
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

    fs::write(&f, ret_lines.join("\n")).unwrap();
}

fn user(userchrome: &Userchrome, profile: &str) {
    let arkenfox = Path::new(&profile).join("user-overrides.js").exists();

    if arkenfox {
        patch_user_file(userchrome, Path::new(&profile).join("user-overrides.js"));

        println!("{} updating arkenfox", "4".green());
        run_arkenfox_script(profile, "updater", vec!["-s"]);
        run_arkenfox_script(profile, "prefsCleaner", vec!["-s"]);
    } else {
        patch_user_file(userchrome, Path::new(&profile).join("user.js"));
    }
}

pub fn switch(userchrome: &Userchrome, profile: String) {
    print_userchrome(userchrome, false);
    println!();

    let temp_path = env::temp_dir().join(nanoid!());

    println!("{} cloning repository", "1".green());

    let mut clone_cmd = Command::new("git");
    clone_cmd.args([
        "clone",
        "--depth=1",
        &userchrome.clone_url,
        temp_path.to_str().unwrap(),
    ]);
    clone_cmd.stdin(Stdio::null());
    clone_cmd.stdout(Stdio::null());
    clone_cmd.stderr(Stdio::null());

    clone_cmd.status().unwrap();

    println!("{} installing userchrome", "2".green());

    let new_chrome_dir = Path::new(&profile).join("chrome");

    if new_chrome_dir.exists() {
        fs::remove_dir_all(&new_chrome_dir).unwrap();
    }

    let mut cloned_chrome_dir = temp_path.join("chrome");
    if !cloned_chrome_dir.exists() {
        cloned_chrome_dir = temp_path;
    }

    copy_dir_all(&cloned_chrome_dir, &new_chrome_dir).unwrap();

    println!("{} applying user.js", "3".green());

    user(userchrome, profile.as_str());

    println!("{}", "done!".green());
}
