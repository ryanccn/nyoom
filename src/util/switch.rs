use nanoid::nanoid;
use std::{env, fs, io::Result, path::Path, process::Command};

use crate::config::Userchrome;

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

pub fn switch(userchrome: &Userchrome, profile: String) {
    let temp_path = env::temp_dir().join(nanoid!());

    let mut clone_cmd = Command::new("git");
    clone_cmd.args([
        "clone",
        "--depth=1",
        &userchrome.clone_url,
        temp_path.to_str().unwrap(),
    ]);
    clone_cmd.status().unwrap();

    let new_chrome_dir = Path::new(&profile).join("chrome");

    fs::remove_dir_all(&new_chrome_dir).unwrap();

    let mut cloned_chrome_dir = temp_path.join("chrome");
    if !cloned_chrome_dir.exists() {
        cloned_chrome_dir = temp_path;
    }

    copy_dir_all(&cloned_chrome_dir, &new_chrome_dir).unwrap();
}
