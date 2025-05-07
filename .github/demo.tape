#! /usr/bin/env nix
#! nix shell -i --impure --expr ``
#! nix with (import (builtins.getFlake ''nixpkgs'') {});
#! nix [ bash coreutils ncurses fish eza bat vhs vivid ]
#! nix `` --command vhs

# SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
#
# SPDX-License-Identifier: GPL-3.0-or-later

Output .github/demo.gif

Set Shell "fish"
Set Theme "Catppuccin Mocha"

Set FontFamily "IBM Plex Mono"
Set FontSize 20
Set LineHeight 1.5
Set Padding 40
Set TypingSpeed 20ms

Set Width 1200
Set Height 800
Set Framerate 30

Hide

Type@0 'exec fish --no-config' Enter Wait

Type@0 'fish_config theme choose "Catppuccin Mocha"' Enter Wait
Type@0 'set fish_autosuggestion_enabled 0' Enter Wait
Type@0 'function fish_prompt; echo "$(set_color -d)>$(set_color normal) "; end' Enter Wait
Type@0 'set -x LS_COLORS "$(vivid generate catppuccin-mocha)"' Enter Wait

Type@0 'set nyoom_temp_config "$(mktemp -d)/nyoom.toml"' Enter Wait
Type@0 'set nyoom_temp_profile "/tmp/firefox"' Enter Wait

Type@0 'mkdir -p "$nyoom_temp_profile"' Enter Wait
Type@0 'touch "$nyoom_temp_profile/user-overrides.js"' Enter Wait
Type@0 'touch "$nyoom_temp_profile/user.js"' Enter Wait
Type@0 'touch "$nyoom_temp_profile/updater.sh" && chmod +x "$nyoom_temp_profile/updater.sh"' Enter Wait
Type@0 'touch "$nyoom_temp_profile/prefsCleaner.sh" && chmod +x "$nyoom_temp_profile/prefsCleaner.sh"' Enter Wait

Type@0 'alias nyoom="target/release/nyoom -c $nyoom_temp_config"' Enter Wait
Type@0 'alias ls="eza"' Enter Wait
Type@0 'alias cat="bat --style plain --color always --paging never"' Enter Wait
Type@0 'nyoom profile "$nyoom_temp_profile"' Enter Wait

Type@0 'clear' Enter Wait

Show

Type 'nyoom add edge github:bmFtZQ/edge-frfox' Enter Wait
Sleep 1s
Type 'nyoom config set --raw edge svg.context-properties.content.enabled true' Enter Wait
Sleep 1s
Type 'nyoom config set --raw edge layout.css.color-mix.enabled true' Enter Wait
Sleep 1s

Hide
Type 'clear' Enter Wait
Show

Type 'nyoom preset shyfox' Enter Wait
Sleep 1.5s

Hide
Type 'clear' Enter Wait
Show

Type 'nyoom list' Enter Wait
Sleep 1.5s

Hide
Type 'clear' Enter Wait
Show

Type 'nyoom switch edge' Enter Wait
Sleep 1.5s

Hide
Type 'cd "$nyoom_temp_profile"' Enter Wait
Type 'clear' Enter Wait
Show

Type 'ls chrome' Enter Wait
Sleep 1s
Type 'cat user-overrides.js' Enter Wait
Sleep 1s

Hide
Type 'rm -rf "$nyoom_temp_profile" "$nyoom_temp_config"' Enter Wait
