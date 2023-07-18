Output .github/demo.gif

Set Shell "bash"
Set FontFamily "RyanTerm Nerd Font"
Set FontSize 40
Set LineHeight 1.5
Set Framerate 60
Set PlaybackSpeed 0.5
Set Width 2400
Set Height 1200
Set Theme "Rose Pine"

Hide

Type@0 'cargo build -r' Enter Sleep 1s

Type@0 'export NYOOM_TEMP_CONFIG="$(mktemp -d)/nyoom.toml"' Enter Sleep 1s
Type@0 'export NYOOM_TEMP_PROFILE="$(mktemp -d)"' Enter Sleep 1s

Type@0 'touch "$NYOOM_TEMP_PROFILE/user-overrides.js"' Enter Sleep 1s
Type@0 'touch "$NYOOM_TEMP_PROFILE/user.js"' Enter Sleep 1s
Type@0 'echo "#!/bin/bash" > "$NYOOM_TEMP_PROFILE/updater.sh" && chmod +x "$NYOOM_TEMP_PROFILE/updater.sh"' Enter Sleep 1s
Type@0 'echo "#!/bin/bash" > "$NYOOM_TEMP_PROFILE/prefsCleaner.sh" && chmod +x "$NYOOM_TEMP_PROFILE/prefsCleaner.sh"' Enter Sleep 1s

Type@0 'alias nyoom="./target/release/nyoom -c $NYOOM_TEMP_CONFIG"' Enter Sleep 1s
Type@0 'alias ls="exa" cat="bat --style=plain"' Enter Sleep 1s
Type@0 'nyoom profile "$NYOOM_TEMP_PROFILE"' Enter Sleep 1s

Type@0 'export PS1="\033[2m$ \033[22m"' Enter Sleep 1s
# https://github.com/sharkdp/vivid/pull/108
Type@0 'export LS_COLORS="$(vivid generate rose-pine)"' Enter Sleep 1s

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

Hide
Type 'cd "$NYOOM_TEMP_PROFILE"' Enter
Type 'clear' Enter
Sleep 1s
Show
Sleep 2s

Type 'ls chrome' Sleep 500ms Enter
Sleep 5s
Type 'cat user-overrides.js' Sleep 500ms Enter
Sleep 10s

Hide
Type 'rm -rf "$NYOOM_TEMP_PROFILE" "$NYOOM_TEMP_CONFIG"' Enter
