// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    io::{BufReader, Cursor},
    path::Path,
};
use tokio::fs;
use tokio_stream::StreamExt as _;

use anstream::{eprint, eprintln, stderr};
use crossterm::{ExecutableCommand as _, cursor, terminal};
use eyre::{Result, bail, eyre};
use owo_colors::OwoColorize as _;
use temp_dir::TempDir;

use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use xz2::bufread::XzDecoder;
use zip::ZipArchive;
use zstd::stream::Decoder as ZstdDecoder;

use url::Url;

async fn strip_root(dir: &Path) -> Result<()> {
    let mut entries = fs::read_dir(dir).await?;

    let first = entries.next_entry().await?;
    let only_entry = entries.next_entry().await?.is_none();

    if let Some(first) = first {
        if only_entry && first.path().is_dir() {
            let subdirectory = first.path();

            let mut sub_entries = fs::read_dir(&subdirectory).await?;
            while let Some(from) = sub_entries.next_entry().await? {
                let to = dir.join(from.file_name());
                fs::rename(from.path(), &to).await?;
            }

            fs::remove_dir(subdirectory).await?;
        }
    }

    Ok(())
}

pub async fn archive(url: &Url, target: &Path) -> Result<()> {
    let ext = Path::new(url.path())
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| eyre!("could not infer file extension"))?;

    eprint!("{} {}  ", "╰".cyan().dimmed(), url.dimmed());

    stderr().execute(cursor::SavePosition)?;

    let mut resp = reqwest::get(url.to_owned())
        .await?
        .error_for_status()?
        .bytes_stream();

    let mut data: Vec<u8> = Vec::new();

    while let Some(chunk) = resp.next().await {
        data.extend(chunk?);

        stderr()
            .execute(cursor::RestorePosition)?
            .execute(terminal::Clear(terminal::ClearType::UntilNewLine))?;

        eprint!(
            "{}",
            humansize::format_size(data.len(), humansize::DECIMAL)
                .cyan()
                .dimmed(),
        );
    }

    stderr()
        .execute(cursor::RestorePosition)?
        .execute(terminal::Clear(terminal::ClearType::UntilNewLine))?;

    eprintln!(
        "{}",
        humansize::format_size(data.len(), humansize::DECIMAL)
            .green()
            .dimmed(),
    );

    let reader = BufReader::new(Cursor::new(data));

    let temp_extract_dir = TempDir::new()?;
    let temp_extract_path = temp_extract_dir.path();

    match ext {
        "zip" => {
            let mut zip = ZipArchive::new(reader)?;
            zip.extract(temp_extract_path)?;
        }

        "tar" => {
            let mut archive = tar::Archive::new(reader);
            archive.unpack(temp_extract_path)?;
        }

        "gz" | "tgz" => {
            let gz = GzDecoder::new(reader);
            let mut archive = tar::Archive::new(gz);
            archive.unpack(temp_extract_path)?;
        }

        "xz" => {
            let xz = XzDecoder::new(reader);
            let mut archive = tar::Archive::new(xz);
            archive.unpack(temp_extract_path)?;
        }

        "bz2" => {
            let bz = BzDecoder::new(reader);
            let mut archive = tar::Archive::new(bz);
            archive.unpack(temp_extract_path)?;
        }

        "zst" => {
            let zstd = ZstdDecoder::new(reader)?;
            let mut archive = tar::Archive::new(zstd);
            archive.unpack(temp_extract_path)?;
        }

        _ => {
            bail!("unsupported archive extension: {ext:?}");
        }
    }

    strip_root(temp_extract_path).await?;
    super::copy_dir_all(temp_extract_path, target).await?;

    fs::remove_dir_all(temp_extract_path).await?;

    Ok(())
}
