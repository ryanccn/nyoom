Output .github/demo.gif

Set Shell "bash"
Set FontFamily "RyanTerm Nerd Font"
Set FontSize 40
Set LineHeight 1.5
Set Framerate 30
Set Width 2400
Set Height 1200
Set Theme "Rose Pine"

Hide
Type 'cargo build -r' Enter
Type 'export PS1="\033[2m$ \033[22m"' Enter
Type 'export NYOOM_TEMP_CONFIG="$(mktemp -d)/nyoom.toml"' Enter
Type 'alias nyoom="./target/release/nyoom -c $NYOOM_TEMP_CONFIG"' Enter
Type 'nyoom profile "/Users/ryanccn/Library/Application Support/Firefox/Profiles/zlbiiwgt.ryanccn"' Enter
Type 'clear' Enter
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
Show
Sleep 2s

Type 'nyoom preset material' Sleep 500ms Enter
Sleep 5s

Hide
Type 'clear' Enter
Show
Sleep 2s

Type 'nyoom list' Sleep 500ms Enter
Sleep 5s

Hide
Type 'clear' Enter
Show
Sleep 2s

Type 'nyoom switch edge' Sleep 500ms Enter
Sleep 15s
