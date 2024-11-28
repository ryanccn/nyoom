// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    io::{BufReader, Cursor},
    path::Path,
};
use tokio::fs;
use tokio_stream::{wrappers::ReadDirStream, StreamExt as _};

use anstream::{eprint, stderr};
use crossterm::{cursor, terminal, ExecutableCommand as _};
use owo_colors::OwoColorize as _;
use temp_dir::TempDir;

use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use xz2::bufread::XzDecoder;
use zip::ZipArchive;
use zstd::stream::Decoder as ZstdDecoder;

use eyre::{bail, eyre, Result};

async fn strip_root(dir: &Path) -> Result<()> {
    let entries = ReadDirStream::new(fs::read_dir(dir).await?)
        .collect::<Result<Vec<_>, _>>()
        .await?;

    if entries.len() == 1 && entries[0].path().is_dir() {
        let subdirectory = entries[0].path();

        let mut sub_entries = fs::read_dir(&subdirectory).await?;
        while let Ok(Some(entry)) = sub_entries.next_entry().await {
            let target_path = dir.join(entry.file_name());
            fs::rename(entry.path(), &target_path).await?;
        }

        fs::remove_dir(subdirectory).await?;
    }

    Ok(())
}

pub async fn archive(url: &str, target_dir: &Path) -> Result<()> {
    let ext = Path::new(url)
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| eyre!("could not infer file extension"))?;

    eprint!("{} {}  ", "╰".cyan().dimmed(), url.dimmed());

    stderr().execute(cursor::SavePosition)?;

    let mut resp = reqwest::get(url).await?.error_for_status()?.bytes_stream();
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
    super::copy_dir_all(temp_extract_path, target_dir).await?;

    fs::remove_dir_all(temp_extract_path).await?;

    Ok(())
}
