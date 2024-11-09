# SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
#
# SPDX-License-Identifier: GPL-3.0-or-later

Output .github/demo.gif

Set Shell "fish"
Set FontFamily "RyanTerm Nerd Font"
Set FontSize 40
Set LineHeight 1.5
Set Framerate 60
Set PlaybackSpeed 0.5
Set Width 2400
Set Height 1200
Set Theme "Catppuccin Macchiato"

Hide

Type@0 'set nyoom_temp_config "$(mktemp -d)/nyoom.toml"' Enter Sleep 1s
Type@0 'set nyoom_temp_profile "$(mktemp -d)"' Enter Sleep 1s

Type@0 'touch "$nyoom_temp_profile/user-overrides.js"' Enter Sleep 1s
Type@0 'touch "$nyoom_temp_profile/user.js"' Enter Sleep 1s
Type@0 'echo "#!/bin/sh" > "$nyoom_temp_profile/updater.sh" && chmod +x "$nyoom_temp_profile/updater.sh"' Enter Sleep 1s
Type@0 'echo "#!/bin/sh" > "$nyoom_temp_profile/prefsCleaner.sh" && chmod +x "$nyoom_temp_profile/prefsCleaner.sh"' Enter Sleep 1s

Type@0 'alias nyoom="./target/release/nyoom -c $nyoom_temp_config"' Enter Sleep 1s
Type@0 'alias ls="eza" cat="bat --style=plain"' Enter Sleep 1s
Type@0 'nyoom profile "$nyoom_temp_profile"' Enter Sleep 1s

Type@0 'fish_config theme choose "Catppuccin Macchiato"' Enter Sleep 1s
Type@0 'set fish_autosuggestion_enabled 0' Enter Sleep 1s
Type@0 'function fish_prompt; echo "$(set_color -d)\$$(set_color normal) "; end' Enter Sleep 1s
Type@0 'set -x LS_COLORS "$(vivid generate catppuccin-macchiato)"' Enter Sleep 1s

Type@0 'clear' Enter
Sleep 1s

Show

Sleep 2.5s

Type 'nyoom add edge github:bmFtZQ/edge-frfox' Sleep 500ms Enter
Sleep 2.5s
Type 'nyoom config set edge svg.context-properties.content.enabled true --raw' Sleep 500ms Enter
Sleep 2s
Type 'nyoom config set edge layout.css.color-mix.enabled true --raw' Sleep 500ms Enter
Sleep 5s

Hide
Type 'clear' Enter
Sleep 1s
Show
Sleep 2s

Type 'nyoom preset material' Sleep 500ms Enter
Sleep 5s

Hide
Type 'clear' Enter
Sleep 1s
Show
Sleep 2s

Type 'nyoom list' Sleep 500ms Enter
Sleep 10s

Hide
Type 'clear' Enter
Sleep 1s
Show
Sleep 2s

Type 'nyoom switch edge' Sleep 500ms Enter
Sleep 10s
Screenshot .github/demo.png

Hide
Type 'cd "$nyoom_temp_profile"' Enter
Type 'clear' Enter
Sleep 1s
Show
Sleep 2s

Type 'ls chrome' Sleep 500ms Enter
Sleep 5s
Type 'cat user-overrides.js' Sleep 500ms Enter
Sleep 10s

Hide
Type 'rm -rf "$nyoom_temp_profile" "$nyoom_temp_config"' Enter
