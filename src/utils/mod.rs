// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{ffi::OsString, path::Path};
use tokio::fs;

use sysinfo::{ProcessRefreshKind, RefreshKind, System};

use async_recursion::async_recursion;
use eyre::{Result, bail};

pub mod download;

#[async_recursion]
pub async fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
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

pub fn check_firefox() -> Result<()> {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()),
    );

    if system.processes_by_name(&OsString::from("firefox")).count() > 0 {
        bail!("Firefox is running, refusing to continue!");
    }

    Ok(())
}
