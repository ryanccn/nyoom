// SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
    sync::LazyLock,
};

use eyre::{bail, Result};
use regex::Regex;
use url::Url;

use crate::utils;

static GITHUB_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"github:(?P<repo>([\w\-_]+)/([\w\-_]+))(#(?P<ref>[\w\-_]+))?").unwrap()
});

static CODEBERG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"codeberg:(?P<repo>([\w\-_]+)/([\w\-_]+))(#(?P<ref>[\w\-_]+))?").unwrap()
});

static GITLAB_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"gitlab:(?P<repo>[\w\-_/]+)(#(?P<ref>[\w\-_]+))?").unwrap());

#[derive(Clone, Debug)]
pub enum ParsedSource {
    GitHub { repo: String, r#ref: String },
    Codeberg { repo: String, r#ref: String },
    GitLab { repo: String, r#ref: String },
    Url { inner: Url, implicit: bool },
    Path { inner: PathBuf, implicit: bool },
}

impl ParsedSource {
    pub fn should_canonicalize(&self) -> bool {
        matches!(self, Self::Url { .. } | Self::Path { .. })
    }

    pub async fn retrieve(&self, target: impl AsRef<Path>) -> Result<()> {
        let target = target.as_ref();

        match self {
            ParsedSource::GitHub { repo, r#ref } => {
                let url: Url =
                    format!("https://github.com/{repo}/archive/refs/heads/{ref}.tar.gz").parse()?;
                utils::download::archive(&url, target).await?;
            }

            ParsedSource::Codeberg { repo, r#ref } => {
                let url: Url =
                    format!("https://codeberg.org/{repo}/archive/{ref}.tar.gz").parse()?;
                utils::download::archive(&url, target).await?;
            }

            ParsedSource::GitLab { repo, r#ref } => {
                let url: Url =
                    format!("https://gitlab.com/{repo}/-/archive/{ref}/source-{ref}.tar.gz")
                        .parse()?;
                utils::download::archive(&url, target).await?;
            }

            ParsedSource::Url { inner: url, .. } => {
                utils::download::archive(url, target).await?;
            }

            ParsedSource::Path { inner: path, .. } => {
                utils::copy_dir_all(path, target).await?;
            }
        }

        Ok(())
    }
}

impl Display for ParsedSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::GitHub { repo, r#ref } => format!("github:{repo}#{ref}"),
            Self::Codeberg { repo, r#ref } => format!("codeberg:{repo}#{ref}"),
            Self::GitLab { repo, r#ref } => format!("gitlab:{repo}#{ref}"),

            Self::Url { inner, implicit } => {
                format!("{}{inner}", if *implicit { "" } else { "url:" })
            }

            Self::Path { inner, implicit } => {
                format!(
                    "{}{}",
                    if *implicit { "" } else { "path:" },
                    inner.to_string_lossy()
                )
            }
        })
    }
}

impl FromStr for ParsedSource {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(github) = GITHUB_REGEX.captures(s) {
            return Ok(Self::GitHub {
                repo: github["repo"].to_owned(),
                r#ref: github.name("ref").map_or("main", |m| m.as_str()).into(),
            });
        }

        if let Some(codeberg) = CODEBERG_REGEX.captures(s) {
            return Ok(Self::Codeberg {
                repo: codeberg["repo"].to_owned(),
                r#ref: codeberg.name("ref").map_or("main", |m| m.as_str()).into(),
            });
        }

        if let Some(gitlab) = GITLAB_REGEX.captures(s) {
            return Ok(Self::GitLab {
                repo: gitlab["repo"].to_owned(),
                r#ref: gitlab.name("ref").map_or("main", |m| m.as_str()).into(),
            });
        }

        if let Some(url) = s.strip_prefix("url:") {
            return Ok(Self::Url {
                inner: url.parse()?,
                implicit: false,
            });
        }

        if let Some(path) = s.strip_prefix("path:") {
            let parsed_path = Path::new(path);
            if parsed_path.is_dir() {
                return Ok(Self::Path {
                    inner: parsed_path.canonicalize()?,
                    implicit: false,
                });
            }

            bail!("path {parsed_path:?} is not a directory")
        }

        if s.starts_with("https://") || s.starts_with("http://") {
            return Ok(Self::Url {
                inner: s.parse()?,
                implicit: true,
            });
        }

        let parsed_path = Path::new(s);
        if parsed_path.is_dir() {
            return Ok(Self::Path {
                inner: parsed_path.canonicalize()?,
                implicit: true,
            });
        }

        bail!("invalid source: {s:?}")
    }
}
