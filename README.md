<!--
SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>

SPDX-License-Identifier: CC-BY-NC-SA-4.0
-->

# nyoom

Firefox userchrome manager, written in [Rust](https://rust-lang.org/).

![Demo](/.github/demo.gif)

## Install

### Nix

#### Flake

```nix
{
  inputs = {
    nyoom = {
      url = "github:ryanccn/nyoom";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { ... }@inputs:
    {
      # ...
      environment.systemPackages = with pkgs; [
        inputs.nyoom.packages.${pkgs.system}.nyoom
      ];
    };
}
```

#### Profile

```bash
$ nix profile install github:ryanccn/nyoom
```

### GitHub Releases

You can download pre-built binaries from [GitHub Releases](https://github.com/ryanccn/nyoom/releases/latest). Builds are available for:

- macOS aarch64, x86_64
- Linux x86_64, aarch64 (static)
- Windows x86_64, aarch64 (dynamic, MSVC)

## Usage

### Adding a userchrome

nyoom specifies sources for userchromes in a special format.

- **GitHub**: `github:<owner>/<repo>[#ref]`
- **Codeberg**: `codeberg:<owner>/<repo>[#ref]`
- **GitLab**: `gitlab:<owner>/<repo>[#ref]`
- **URL** (to a known archive format of `.zip`, `.tar`, `.tgz`, `.tar.gz`, `.tar.xz`, `.tar.bz2` or `.tar.zst`): `url:<url>` or just `<url>`
- **Path** to a directory: `path:<path>`

You can add a new userchrome by using the `nyoom add` command:

```bash
$ nyoom add <name> <source>
```

Then, to specify config options specific to a userchrome, use the `nyoom config` commands:

```bash
$ nyoom config set <name> <key> <value>        # value is treated as string
$ nyoom config set --raw <name> <key> <value>  # value is treated as a JavaScript value
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

You can run `nyoom update` on subsequent runs to reapply the userchrome using the latest data from remote, and use `nyoom switch out` to uninstall the userchrome.

## License

GPLv3

[![xkcd 2959: Beam of Light](https://imgs.xkcd.com/comics/beam_of_light.png)](https://xkcd.com/2959)
