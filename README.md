# nyoom

Firefox userchrome manager, written in [Rust](https://rust-lang.org/).

![Demo](/.github/demo.gif)

## Install

### Nix

#### Flake

nyoom is published to [FlakeHub](https://flakehub.com/). You can add the input to your flake with the `fh` CLI by running `fh add ryanccn/nyoom` or adding the following to your inputs:

```nix
{
  inputs = {
    nyoom = {
      url = "https://flakehub.com/f/ryanccn/nyoom/*.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    }
  }
}
```

Alternatively, you can get the main branch from GitHub directly:

```nix
{
  inputs = {
    nyoom = {
      url = "github:ryanccn/nyoom";
      inputs.nixpkgs.follows = "nixpkgs";
    }
  }
}
```

#### Profile

```bash
# stable version
$ nix profile install "https://flakehub.com/f/ryanccn/nyoom/*.tar.gz"
# main branch
$ nix profile install github:ryanccn/nyoom
```

### GitHub Releases

You can download pre-built binaries from [GitHub Releases](https://github.com/ryanccn/nyoom/releases/latest). Builds are available for:

- macOS amd64 (Apple Silicon)
- macOS aarch64 (Intel)
- Linux amd64 (statically linked musl)
- Linux aarch64 (statically linked musl)
- Windows amd64 (dynamically linked MSVC)

## Usage

### Adding a userchrome

nyoom specifies sources for userchromes in a special format.

- **GitHub**: `github:<owner>/<repo>[#ref]`
- **Codeberg**: `codeberg:<owner>/<repo>[#ref]`
- **GitLab**: `gitlab:<owner>/<repo>[#ref]`
- **URL** (to zip file): `url:<url>`

You can add a new userchrome by using the `nyoom add` command:

```bash
$ nyoom add <name> <source>
```

Then, to specify config options specific to a userchrome, use the `nyoom config` commands:

```bash
$ nyoom config set <name> <key> <value>        # value is treated as string
$ nyoom config set --raw <name> <key> <value>  # value is treated as a value
$ nyoom config unset <name> <key>
$ nyoom config list
```

These config options will be automatically added to and removed from your `user-overrides.js` (preferred) or `user.js` upon switching.

### Switching

First, you need to configure the full path to where your Firefox profile is.

```bash
$ nyoom profile <directory>
```

Then, run `nyoom switch <name>` to switch to a userchrome you previously added. nyoom will retrieve the source, install the contents of the userchrome in the `chrome` directory, inject settings into `user-overrides.js` or `user.js`, and update arkenfox (thereby syncing `user-overrides.js` with `user.js`) if arkenfox is detected.

## License

GPLv3
