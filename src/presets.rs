// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::LazyLock;

use crate::config::{Userchrome, UserchromeConfig};

pub static PRESETS: LazyLock<Vec<Userchrome>> = LazyLock::new(|| {
    vec![
        Userchrome {
            name: "edge".to_owned(),
            source: "github:bmFtZQ/edge-frfox".to_owned(),
            configs: vec![
                UserchromeConfig {
                    key: "svg.context-properties.content.enabled".to_owned(),
                    value: "true".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "layout.css.color-mix.enabled".to_owned(),
                    value: "true".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "layout.css.light-dark.enabled".to_owned(),
                    value: "true".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "widget.macos.native-context-menus".to_owned(),
                    value: "false".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "browser.tabs.tabMinWidth".to_owned(),
                    value: "66".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "browser.tabs.tabClipWidth".to_owned(),
                    value: "86".to_owned(),
                    raw: true,
                },
            ],
        },
        Userchrome {
            name: "shyfox".to_owned(),
            source: "github:Naezr/ShyFox".to_owned(),
            configs: vec![
                UserchromeConfig {
                    key: "svg.context-properties.content.enabled".to_owned(),
                    value: "true".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "layout.css.has-selector.enabled".to_owned(),
                    value: "true".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "browser.urlbar.suggest.calculator".to_owned(),
                    value: "true".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "browser.urlbar.unitConversion.enabled".to_owned(),
                    value: "true".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "browser.urlbar.trimHttps".to_owned(),
                    value: "true".to_owned(),
                    raw: true,
                },
                UserchromeConfig {
                    key: "browser.urlbar.trimURLs".to_owned(),
                    value: "true".to_owned(),
                    raw: true,
                },
            ],
        },
    ]
});
